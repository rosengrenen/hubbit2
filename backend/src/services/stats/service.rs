use super::util::{
  calculate_stats, day_start_end, join_stats, map_sessions, month_start_end, year_start_end,
};
use crate::{
  repositories::{Period, StudyPeriodRepository, StudyYearRepository, UserSessionRepository},
  schema::stats::Stat,
  RedisPool,
};
use anyhow::{bail, Result};
use chrono::{DateTime, Datelike, Local, TimeZone};
use juniper::futures::future::join_all;
use lazy_static::lazy_static;
use mobc_redis::{redis::RedisError, AsyncCommands};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::Mutex;
use uuid::Uuid;

// Cache
// Keep track of first time entry, if date is before that or after today, return empty
// * day
//   - old: cache it
//   - current: don't cache it
// * month
//   - old: cache the whole thing
//   - current: n=cur day, sum of 1..n days of current month, needs to keep track of n
// * year
//   - old: cache the whole thing
//   - current: n=cur month, sum of 1..n months of current year, needs to keep track of n
// * all time
//   - sum of years, months, days, don't cache anything extra
// * study year
//   - old: cache the whole thing
//   - current: don't cache anything extra, uses full month caches when possible and day
//     caches for leading and trailing months (that aren't full)
// * study period
//   - same as for study year, but gets less full months

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct PartialMonth {
  day: u32,
  stats: HashMap<Uuid, Stat>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct PartialYear {
  month: u32,
  stats: HashMap<Uuid, Stat>,
}

lazy_static! {
  static ref MIN_DATETIME: DateTime<Local> = Local.ymd(2000, 1, 1).and_hms(0, 0, 0);
  static ref MAX_DATETIME: DateTime<Local> = Local.ymd(2099, 12, 31).and_hms(23, 59, 59);
}

type Stats = HashMap<Uuid, Stat>;

pub struct StatsService {
  user_session_repo: UserSessionRepository,
  study_year_repo: StudyYearRepository,
  study_period_repo: StudyPeriodRepository,
  redis_pool: RedisPool,
  earliest_date: Mutex<Option<DateTime<Local>>>,
}

impl Clone for StatsService {
  fn clone(&self) -> Self {
    Self::new(
      self.user_session_repo.clone(),
      self.study_year_repo.clone(),
      self.study_period_repo.clone(),
      self.redis_pool.clone(),
    )
  }
}

impl StatsService {
  pub fn new(
    user_session_repo: UserSessionRepository,
    study_year_repo: StudyYearRepository,
    study_period_repo: StudyPeriodRepository,
    redis_pool: RedisPool,
  ) -> Self {
    Self {
      user_session_repo,
      redis_pool,
      earliest_date: Mutex::new(None),
      study_year_repo,
      study_period_repo,
    }
  }

  pub async fn redis_get<T: DeserializeOwned>(&self, key: &str) -> Result<T> {
    let mut redis_conn = self.redis_pool.get().await?;
    let res: Result<String, RedisError> = redis_conn.get(key).await;
    let res = match res {
      Ok(res) => res,
      Err(_) => bail!("Couldn't get key from redis"),
    };

    match serde_json::from_str::<T>(&res) {
      Ok(res) => Ok(res),
      Err(_) => bail!("Couldn't parse to type"),
    }
  }

  async fn redis_set<T>(redis_pool: RedisPool, key: String, value: T) -> Result<()>
  where
    T: Serialize,
  {
    if let Ok(mut redis_conn) = redis_pool.get().await {
      redis_conn
        .set::<String, String, String>(key, serde_json::to_string(&value)?)
        .await?;
      Ok(())
    } else {
      bail!("Could not get redis connection");
    }
  }

  pub async fn get_day(&self, year: i32, month: u32, day: u32) -> Result<Stats> {
    let now = Local::now();
    let requested_date = Local.ymd(year, month, day);

    // If date is in the future, abort
    if requested_date > now.date() {
      return Ok(HashMap::new());
    }

    // If date is before earliest date, abort
    let earliest_date = self.get_earliest_date().await?;
    if requested_date < earliest_date.date() {
      return Ok(HashMap::new());
    }

    // Only check redis if not current day
    let key = format!("day:({},{},{})", year, month, day);
    if requested_date != now.date() {
      if let Ok(stats) = self.redis_get::<Stats>(&key).await {
        return Ok(stats);
      }
    }

    let (start_time, end_time) = day_start_end(year, month, day);
    let stats = self.get_stats_for_range(start_time, end_time).await?;

    // Only save to redis if current day
    if requested_date != now.date() {
      let stats = stats.clone();
      let redis_pool = self.redis_pool.clone();
      tokio::spawn(async move {
        Self::redis_set(redis_pool, key, stats).await;
      });
    }

    Ok(stats)
  }

  pub async fn get_month(&self, year: i32, month: u32) -> Result<Stats> {
    let now = Local::now();

    // If date is in the future, abort
    if year > now.year() || (year == now.year() && month > now.month()) {
      return Ok(HashMap::new());
    }

    // If date is before earliest date, abort
    let earliest_date = self.get_earliest_date().await?;
    if year < earliest_date.year()
      || (year == earliest_date.year() && month < earliest_date.month())
    {
      return Ok(HashMap::new());
    }

    // If month is current month, work with partial month cache, which is a bit more complicated
    let key = format!("month:({},{})", year, month);
    if year == now.year() && month == now.month() {
      let mut partial_entry: PartialMonth = self.redis_get(&key).await.unwrap_or_default();
      let mut stats = join_all(
        (partial_entry.day + 1..=now.day())
          .map(|day| self.get_day(year, month, day))
          .collect::<Vec<_>>(),
      )
      .await
      .into_iter()
      .collect::<Result<Vec<Stats>>>()?;
      // partial_entry.day should never be set to today, so the list should never be empty
      let todays_stats = stats.pop().unwrap();
      for stat in stats {
        join_stats(&mut partial_entry.stats, &stat);
      }

      let new_partial_entry = PartialMonth {
        day: now.day() - 1,
        ..partial_entry.clone()
      };
      let redis_pool = self.redis_pool.clone();
      tokio::spawn(async move {
        Self::redis_set(redis_pool, key, new_partial_entry).await;
      });

      join_stats(&mut partial_entry.stats, &todays_stats);
      Ok(partial_entry.stats)
    } else {
      if let Ok(stats) = self.redis_get(&key).await {
        return Ok(stats);
      }

      let (start_time, end_time) = month_start_end(year, month);
      let stats = self.get_stats_for_range(start_time, end_time).await?;
      let stats_clone = stats.clone();
      let redis_pool = self.redis_pool.clone();
      tokio::spawn(async move {
        Self::redis_set(redis_pool, key, stats_clone).await;
      });

      Ok(stats)
    }
  }

  pub async fn get_year(&self, year: i32) -> Result<Stats> {
    let now = Local::now();
    // If date is in the future, abort
    if year > now.year() {
      return Ok(HashMap::new());
    }

    // If date is before earliest date, abort
    let earliest_date = self.get_earliest_date().await?;
    if year < earliest_date.year() {
      return Ok(HashMap::new());
    }

    // If month is current year, work with partial year cache, which is a bit more complicated
    let key = format!("year:{}", year);
    if year == now.year() {
      let mut partial_entry: PartialYear = self.redis_get(&key).await.unwrap_or_default();

      let mut stats = join_all(
        (partial_entry.month + 1..=now.month())
          .map(|month| self.get_month(year, month))
          .collect::<Vec<_>>(),
      )
      .await
      .into_iter()
      .collect::<Result<Vec<Stats>>>()?;
      // partial_entry.month should never be set to the current month, so the list should never be empty
      let current_month_stats = stats.pop().unwrap();
      for stat in stats {
        join_stats(&mut partial_entry.stats, &stat);
      }

      let new_partial_entry = PartialYear {
        month: now.month() - 1,
        ..partial_entry.clone()
      };
      let redis_pool = self.redis_pool.clone();
      tokio::spawn(async move {
        Self::redis_set(redis_pool, key, new_partial_entry).await;
      });

      join_stats(&mut partial_entry.stats, &current_month_stats);
      Ok(partial_entry.stats)
    } else {
      if let Ok(stats) = self.redis_get(&key).await {
        return Ok(stats);
      }

      let (start_time, end_time) = year_start_end(year);
      let stats = self.get_stats_for_range(start_time, end_time).await?;
      let stats_clone = stats.clone();
      let redis_pool = self.redis_pool.clone();
      tokio::spawn(async move {
        Self::redis_set(redis_pool, key, stats_clone).await;
      });

      Ok(stats)
    }
  }

  pub async fn get_study_year(&self, year: i32) -> Result<Stats> {
    let now = Local::now();
    let (start_date, end_date) = self.study_year_repo.get_by_year(year).await?;

    if now.date() >= start_date && now.date() <= end_date {
      let start_month = start_date.month();
      let cur_month = now.month();
      let stats: Stats = if start_month == cur_month {
        join_all(
          (start_date.day()..=now.day())
            .map(|day| self.get_day(start_date.year(), start_month, day))
            .collect::<Vec<_>>(),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<Stats>>>()?
        .into_iter()
        .fold(HashMap::new(), |mut prev, cur| {
          join_stats(&mut prev, &cur);
          prev
        })
      } else {
        let mut day_futures = Vec::new();
        let mut month_futures = Vec::new();
        // Leading days
        let (_, end_of_month) = month_start_end(start_date.year(), start_date.month());
        for day in start_date.day()..=end_of_month.day() {
          day_futures.push(self.get_day(start_date.year(), start_month, day));
        }
        // Middle months
        for month in start_month + 1..cur_month {
          month_futures.push(self.get_month(year, month));
        }

        // Trailing days
        for day in 1..now.day() {
          day_futures.push(self.get_day(now.year(), cur_month, day));
        }

        // Await all stats and then join them
        let (day_stats, month_stats) = tokio::join!(join_all(day_futures), join_all(month_futures));
        let mut stats = day_stats
          .into_iter()
          .collect::<Result<Vec<Stats>>>()?
          .into_iter()
          .fold(HashMap::new(), |mut prev, cur| {
            join_stats(&mut prev, &cur);
            prev
          });
        month_stats
          .into_iter()
          .collect::<Result<Vec<Stats>>>()?
          .into_iter()
          .for_each(|stat| join_stats(&mut stats, &stat));
        stats
      };

      Ok(stats)
    } else {
      let key = format!("study-period:{}", year);
      if let Ok(stats) = self.redis_get(&key).await {
        return Ok(stats);
      }

      let stats = self
        .get_stats_for_range(start_date.and_hms(0, 0, 0), end_date.and_hms(23, 59, 59))
        .await?;
      let stats_clone = stats.clone();
      let redis_pool = self.redis_pool.clone();
      tokio::spawn(async move {
        Self::redis_set(redis_pool, key, stats_clone).await;
      });

      Ok(stats)
    }
  }

  pub async fn get_study_period(&self, year: i32, period: Period) -> Result<Stats> {
    let now = Local::now();
    let (start_date, end_date) = self
      .study_period_repo
      .get_by_year_and_period(year, period)
      .await?;

    if now.date() >= start_date && now.date() <= end_date {
      let start_month = start_date.month();
      let cur_month = now.month();
      let stats: Stats = if start_month == cur_month {
        join_all(
          (start_date.day()..=now.day())
            .map(|day| self.get_day(start_date.year(), start_month, day))
            .collect::<Vec<_>>(),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<Stats>>>()?
        .into_iter()
        .fold(HashMap::new(), |mut prev, cur| {
          join_stats(&mut prev, &cur);
          prev
        })
      } else {
        let mut day_futures = Vec::new();
        let mut month_futures = Vec::new();
        // Leading days
        let (_, end_of_month) = month_start_end(start_date.year(), start_date.month());
        for day in start_date.day()..=end_of_month.day() {
          day_futures.push(self.get_day(start_date.year(), start_month, day));
        }
        // Middle months
        for month in start_month + 1..cur_month {
          month_futures.push(self.get_month(year, month));
        }

        // Trailing days
        for day in 1..now.day() {
          day_futures.push(self.get_day(now.year(), cur_month, day));
        }

        // Await all stats and then join them
        let (day_stats, month_stats) = tokio::join!(join_all(day_futures), join_all(month_futures));
        let mut stats = day_stats
          .into_iter()
          .collect::<Result<Vec<Stats>>>()?
          .into_iter()
          .fold(HashMap::new(), |mut prev, cur| {
            join_stats(&mut prev, &cur);
            prev
          });
        month_stats
          .into_iter()
          .collect::<Result<Vec<Stats>>>()?
          .into_iter()
          .for_each(|stat| join_stats(&mut stats, &stat));
        stats
      };

      Ok(stats)
    } else {
      let period_num: i32 = period.into();
      let key = format!("study-period:({},{})", year, period_num);
      if let Ok(stats) = self.redis_get(&key).await {
        return Ok(stats);
      }

      let stats = self
        .get_stats_for_range(start_date.and_hms(0, 0, 0), end_date.and_hms(23, 59, 59))
        .await?;
      let stats_clone = stats.clone();
      let redis_pool = self.redis_pool.clone();
      tokio::spawn(async move {
        Self::redis_set(redis_pool, key, stats_clone).await;
      });

      Ok(stats)
    }
  }

  pub async fn get_lifetime(&self) -> Result<Stats> {
    let now = Local::now();
    let earliest_date = self.get_earliest_date().await?;
    let stats = join_all(
      (earliest_date.year()..=now.year())
        .map(|year| self.get_year(year))
        .collect::<Vec<_>>(),
    )
    .await
    .into_iter()
    .collect::<Result<Vec<Stats>>>()?;
    Ok(stats.iter().fold(HashMap::new(), |mut prev, cur| {
      join_stats(&mut prev, &cur);
      prev
    }))
  }

  async fn get_stats_for_range(
    &self,
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
  ) -> Result<Stats> {
    let sessions = self
      .user_session_repo
      .get_range(start_time, end_time)
      .await?;
    let sessions = map_sessions(sessions);
    Ok(calculate_stats(&sessions, start_time, end_time))
  }

  async fn get_earliest_date(&self) -> Result<DateTime<Local>> {
    let mut earliest_date_lock = self.earliest_date.lock().await;
    if let Some(earliest_date) = *earliest_date_lock {
      return Ok(earliest_date);
    }

    if let Ok(earliest_date) = self.redis_get("earliest_date").await {
      return Ok(earliest_date);
    }

    let sessions = self
      .user_session_repo
      .get_range(*MIN_DATETIME, *MAX_DATETIME)
      .await?;
    let earliest_date = sessions.iter().fold(*MAX_DATETIME, |prev, cur| {
      prev.min(cur.start_time.with_timezone(&Local))
    });
    *earliest_date_lock = Some(earliest_date);

    let redis_pool = self.redis_pool.clone();
    let earliest_date_clone = earliest_date.clone();
    tokio::spawn(async move {
      Self::redis_set(redis_pool, "earliest_date".to_owned(), earliest_date_clone).await;
    });

    Ok(earliest_date)
  }
}
