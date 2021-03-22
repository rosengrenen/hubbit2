use super::util::{
  calculate_stats, day_start_end, join_stats, map_sessions, month_start_end, year_start_end,
};
use crate::{
  repositories::{Period, UserSessionRepository},
  schema::stats::Stat,
  RedisPool,
};
use chrono::{DateTime, Datelike, Local, TimeZone};
use juniper::futures::future::join_all;
use lazy_static::lazy_static;
use mobc_redis::{redis::RedisError, AsyncCommands};
use serde::{Deserialize, Serialize};
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
struct RedisPartialMonthEntry {
  day: u32,
  stats: HashMap<Uuid, Stat>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct RedisPartialYearEntry {
  month: u32,
  stats: HashMap<Uuid, Stat>,
}

lazy_static! {
  static ref MIN_DATETIME: DateTime<Local> = Local.ymd(2000, 1, 1).and_hms(0, 0, 0);
  static ref MAX_DATETIME: DateTime<Local> = Local.ymd(2099, 12, 31).and_hms(23, 59, 59);
}

pub struct StatsService {
  user_session_repo: UserSessionRepository,
  redis_pool: RedisPool,
  earliest_date: Mutex<Option<DateTime<Local>>>,
}

impl Clone for StatsService {
  fn clone(&self) -> Self {
    Self::new(self.user_session_repo.clone(), self.redis_pool.clone())
  }
}

impl StatsService {
  pub fn new(user_session_repo: UserSessionRepository, redis_pool: RedisPool) -> Self {
    Self {
      user_session_repo,
      redis_pool,
      earliest_date: Mutex::new(None),
    }
  }

  pub async fn get_day(&self, year: i32, month: u32, day: u32) -> HashMap<Uuid, Stat> {
    let now = Local::now();
    let date = Local.ymd(year, month, day);
    // If date is in the future, abort
    if date > now.date() {
      return HashMap::new();
    }

    let earliest_date = self.get_earliest_date().await;
    if date < earliest_date.date() {
      return HashMap::new();
    }

    // Only check redis if not current day
    let key = format!("day:({},{},{})", year, month, day);
    if date != now.date() {
      let mut redis_conn = self.redis_pool.get().await.unwrap();
      let raw_stats: Result<String, RedisError> = redis_conn.get(&key).await;
      if let Ok(raw_stats) = raw_stats {
        if let Ok(stats) = serde_json::from_str::<HashMap<Uuid, Stat>>(&raw_stats) {
          return stats;
        }
      }
    }

    let (start_time, end_time) = day_start_end(year, month, day);
    let stats = self.get_stats_for_range(start_time, end_time).await;

    // Only save to redis if current day
    if date != now.date() {
      let stats = stats.clone();
      let redis_pool = self.redis_pool.clone();
      tokio::spawn(async move {
        let mut redis_conn = redis_pool.get().await.unwrap();
        redis_conn
          .set::<String, String, String>(key, serde_json::to_string(&stats).unwrap())
          .await
          .unwrap();
      });
    }

    stats
  }

  pub async fn get_month(&self, year: i32, month: u32) -> HashMap<Uuid, Stat> {
    let now = Local::now();
    // If date is in the future, abort
    if year > now.year() || (year == now.year() && month > now.month()) {
      return HashMap::new();
    }

    let earliest_date = self.get_earliest_date().await;
    if year < earliest_date.year()
      || (year == earliest_date.year() && month < earliest_date.month())
    {
      return HashMap::new();
    }

    // If month is current month, work with partial month cache, which is a bit more complicated
    let key = format!("month:({},{})", year, month);
    if year == now.year() && month == now.month() {
      let mut partial_entry = {
        let mut redis_conn = self.redis_pool.get().await.unwrap();
        let raw_partial_entry: Result<String, RedisError> = redis_conn.get(&key).await;
        if let Ok(raw_partial_entry) = raw_partial_entry {
          if let Ok(partial_entry) =
            serde_json::from_str::<RedisPartialMonthEntry>(&raw_partial_entry)
          {
            partial_entry
          } else {
            RedisPartialMonthEntry::default()
          }
        } else {
          RedisPartialMonthEntry::default()
        }
      };

      let stats_futures = (partial_entry.day + 1..=now.day())
        .map(|day| self.get_day(year, month, day))
        .collect::<Vec<_>>();
      let mut stats = join_all(stats_futures).await;
      let todays_stats = stats.pop().unwrap();
      for stat in stats {
        join_stats(&mut partial_entry.stats, &stat);
      }
      {
        let mut partial_entry = partial_entry.clone();
        partial_entry.day = now.day() - 1;
        let redis_pool = self.redis_pool.clone();
        tokio::spawn(async move {
          let mut redis_conn = redis_pool.get().await.unwrap();
          redis_conn
            .set::<String, String, String>(key, serde_json::to_string(&partial_entry).unwrap())
            .await
            .unwrap();
        });
      }
      join_stats(&mut partial_entry.stats, &todays_stats);
      partial_entry.stats
    } else {
      {
        let mut redis_conn = self.redis_pool.get().await.unwrap();
        let raw_stats: Result<String, RedisError> = redis_conn.get(&key).await;
        if let Ok(raw_stats) = raw_stats {
          if let Ok(stats) = serde_json::from_str::<HashMap<Uuid, Stat>>(&raw_stats) {
            return stats;
          }
        }
      }

      let (start_time, end_time) = month_start_end(year, month);
      let stats = self.get_stats_for_range(start_time, end_time).await;
      let redis_pool = self.redis_pool.clone();
      {
        let stats = stats.clone();
        tokio::spawn(async move {
          let mut redis_conn = redis_pool.get().await.unwrap();
          redis_conn
            .set::<String, String, String>(key, serde_json::to_string(&stats).unwrap())
            .await
            .unwrap();
        });
      }

      stats
    }
  }

  pub async fn get_year(&self, year: i32) -> HashMap<Uuid, Stat> {
    let now = Local::now();
    // If date is in the future, abort
    if year > now.year() {
      return HashMap::new();
    }

    let earliest_date = self.get_earliest_date().await;
    if year < earliest_date.year() {
      return HashMap::new();
    }

    // If month is current year, work with partial year cache, which is a bit more complicated
    let key = format!("year:{}", year);
    if year == now.year() {
      let mut partial_entry = {
        let mut redis_conn = self.redis_pool.get().await.unwrap();
        let raw_partial_entry: Result<String, RedisError> = redis_conn.get(&key).await;
        if let Ok(raw_partial_entry) = raw_partial_entry {
          if let Ok(partial_entry) =
            serde_json::from_str::<RedisPartialYearEntry>(&raw_partial_entry)
          {
            partial_entry
          } else {
            RedisPartialYearEntry::default()
          }
        } else {
          RedisPartialYearEntry::default()
        }
      };

      let stats_futures = (partial_entry.month + 1..=now.month())
        .map(|month| self.get_month(year, month))
        .collect::<Vec<_>>();
      let mut stats = join_all(stats_futures).await;
      let current_months_stats = stats.pop().unwrap();
      for stat in stats {
        join_stats(&mut partial_entry.stats, &stat);
      }
      {
        let mut partial_entry = partial_entry.clone();
        partial_entry.month = now.month() - 1;
        let redis_pool = self.redis_pool.clone();
        tokio::spawn(async move {
          let mut redis_conn = redis_pool.get().await.unwrap();
          redis_conn
            .set::<String, String, String>(key, serde_json::to_string(&partial_entry).unwrap())
            .await
            .unwrap();
        });
      }
      join_stats(&mut partial_entry.stats, &current_months_stats);
      partial_entry.stats
    } else {
      {
        let mut redis_conn = self.redis_pool.get().await.unwrap();
        let raw_stats: Result<String, RedisError> = redis_conn.get(&key).await;
        if let Ok(raw_stats) = raw_stats {
          if let Ok(stats) = serde_json::from_str::<HashMap<Uuid, Stat>>(&raw_stats) {
            return stats;
          }
        }
      }

      let (start_time, end_time) = year_start_end(year);
      let stats = self.get_stats_for_range(start_time, end_time).await;
      let redis_pool = self.redis_pool.clone();
      {
        let stats = stats.clone();
        tokio::spawn(async move {
          let mut redis_conn = redis_pool.get().await.unwrap();
          redis_conn
            .set::<String, String, String>(key, serde_json::to_string(&stats).unwrap())
            .await
            .unwrap();
        });
      }

      stats
    }
  }

  pub async fn get_lifetime(&self) -> HashMap<Uuid, Stat> {
    let now = Local::now();
    let earliest_date = self.get_earliest_date().await;
    let stats_futures = (earliest_date.year()..=now.year())
      .map(|year| self.get_year(year))
      .collect::<Vec<_>>();
    let stats = join_all(stats_futures).await;
    stats.iter().fold(HashMap::new(), |mut prev, cur| {
      join_stats(&mut prev, &cur);
      prev
    })
  }

  pub async fn get_study_period(&self, year: i32, period: Period) -> HashMap<Uuid, Stat> {
    unimplemented!()
  }

  pub async fn get_study_year(&self, year: i32) -> HashMap<Uuid, Stat> {
    unimplemented!()
  }

  async fn get_stats_for_range(
    &self,
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
  ) -> HashMap<Uuid, Stat> {
    let sessions = self
      .user_session_repo
      .get_range(start_time, end_time)
      .await
      .unwrap();
    let sessions = map_sessions(sessions);
    calculate_stats(&sessions, start_time, end_time)
  }

  async fn get_earliest_date(&self) -> DateTime<Local> {
    let mut earliest_date_lock = self.earliest_date.lock().await;
    if let Some(earliest_date) = *earliest_date_lock {
      return earliest_date;
    }

    {
      let mut redis_conn = self.redis_pool.get().await.unwrap();
      let raw_earliest_date: Result<String, RedisError> = redis_conn.get("earliest_date").await;
      if let Ok(raw_earliest_date) = raw_earliest_date {
        if let Ok(earliest_date) = serde_json::from_str::<DateTime<Local>>(&raw_earliest_date) {
          return earliest_date;
        }
      }
    }

    let sessions = self
      .user_session_repo
      .get_range(*MIN_DATETIME, *MAX_DATETIME)
      .await
      .unwrap();
    let earliest_date = sessions.iter().fold(*MAX_DATETIME, |prev, cur| {
      prev.min(cur.start_time.with_timezone(&Local))
    });
    *earliest_date_lock = Some(earliest_date);

    let redis_pool = self.redis_pool.clone();
    tokio::spawn(async move {
      let mut redis_conn = redis_pool.get().await.unwrap();
      redis_conn
        .set::<String, String, String>(
          "earliest_date".to_owned(),
          serde_json::to_string(&earliest_date).unwrap(),
        )
        .await
        .unwrap();
    });

    earliest_date
  }
}
