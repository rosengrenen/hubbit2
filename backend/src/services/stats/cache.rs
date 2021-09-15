use std::collections::HashMap;

use async_graphql::futures_util::future::join_all;
use chrono::{Datelike, Duration, Local, NaiveDate, TimeZone};

use crate::{
  error::HubbitResult,
  services::{
    stats::util::{day_time_bounds, month_time_bounds, year_time_bounds},
    util::{redis_get, redis_set},
  },
};

use super::{util::join_stats, Stats, StatsService};

impl StatsService {
  pub async fn get_range(
    &self,
    mut start_date: NaiveDate,
    mut end_date: NaiveDate,
  ) -> HubbitResult<Stats> {
    let now = Local::now();
    if end_date.and_hms(0, 0, 0) > now.naive_local() {
      end_date = now.date().naive_local();
    }

    let earliest_date = self.get_earliest_date().await?;
    if start_date < earliest_date {
      start_date = earliest_date;
    }

    let mut days = Vec::new();
    let mut months = Vec::new();
    let mut years = Vec::new();

    days.append(&mut leading_days(start_date, end_date));
    months.append(&mut leading_months(start_date, end_date));
    years.append(&mut middle_years(start_date, end_date));
    months.append(&mut trailing_months(start_date, end_date));
    days.append(&mut trailing_days(start_date, end_date));

    let day_futs = days
      .into_iter()
      .map(|(y, m, d)| self.get_day_unchecked(y, m, d))
      .collect::<Vec<_>>();
    let month_futs = months
      .into_iter()
      .map(|(y, m)| self.get_month_unchecked(y, m))
      .collect::<Vec<_>>();
    let year_futs = years
      .into_iter()
      .map(|y| self.get_year_unchecked(y))
      .collect::<Vec<_>>();

    let (day_stats, month_stats, year_stats) = tokio::join!(
      join_all(day_futs),
      join_all(month_futs),
      join_all(year_futs)
    );

    let mut stats = day_stats
      .into_iter()
      .collect::<HubbitResult<Vec<Stats>>>()?
      .into_iter()
      .fold(HashMap::new(), |mut prev, cur| {
        join_stats(&mut prev, &cur);
        prev
      });
    month_stats
      .into_iter()
      .collect::<HubbitResult<Vec<Stats>>>()?
      .into_iter()
      .for_each(|stat| join_stats(&mut stats, &stat));
    year_stats
      .into_iter()
      .collect::<HubbitResult<Vec<Stats>>>()?
      .into_iter()
      .for_each(|stat| join_stats(&mut stats, &stat));

    Ok(stats)
  }

  async fn get_day_unchecked(&self, year: i32, month: u32, day: u32) -> HubbitResult<Stats> {
    let now = Local::now();
    let requested_date = Local.ymd(year, month, day);

    // Only check redis if not current day
    let key = format!("day:({},{},{})", year, month, day);
    if requested_date != now.date() {
      if let Ok(stats) = redis_get::<Stats>(self.redis_pool.clone(), &key).await {
        return Ok(stats);
      }
    }

    let (start_time, end_time) = day_time_bounds(year, month, day);
    let stats = self.get_range_fresh(start_time, end_time).await?;

    // Only save to redis if current day
    if requested_date != now.date() {
      let stats = stats.clone();
      let redis_pool = self.redis_pool.clone();
      tokio::spawn(async move { redis_set(redis_pool, key, stats).await });
    }

    Ok(stats)
  }

  async fn get_month_unchecked(&self, year: i32, month: u32) -> HubbitResult<Stats> {
    // If month is current month, work with partial month cache, which is a bit more complicated
    let key = format!("month:({},{})", year, month);
    if let Ok(stats) = redis_get(self.redis_pool.clone(), &key).await {
      return Ok(stats);
    }

    let (start_time, end_time) = month_time_bounds(year, month);
    let stats = self.get_range_fresh(start_time, end_time).await?;
    let stats_clone = stats.clone();
    let redis_pool = self.redis_pool.clone();
    tokio::spawn(async move { redis_set(redis_pool, key, stats_clone).await });

    Ok(stats)
  }

  async fn get_year_unchecked(&self, year: i32) -> HubbitResult<Stats> {
    // If month is current year, work with partial year cache, which is a bit more complicated
    let key = format!("year:{}", year);
    if let Ok(stats) = redis_get(self.redis_pool.clone(), &key).await {
      return Ok(stats);
    }

    let (start_time, end_time) = year_time_bounds(year);
    let stats = self.get_range_fresh(start_time, end_time).await?;
    let stats_clone = stats.clone();
    let redis_pool = self.redis_pool.clone();
    tokio::spawn(async move { redis_set(redis_pool, key, stats_clone).await });

    Ok(stats)
  }
}

fn last_day_of_month(year: i32, month: u32) -> u32 {
  let first_day_of_next_month = if month == 12 {
    Local.ymd(year + 1, 1, 1).and_hms(0, 0, 0)
  } else {
    Local.ymd(year, month + 1, 1).and_hms(0, 0, 0)
  };

  let last_day_of_month = first_day_of_next_month - Duration::seconds(1);
  last_day_of_month.date().naive_local().day()
}

fn leading_days(start_date: NaiveDate, end_date: NaiveDate) -> Vec<(i32, u32, u32)> {
  let start_year = start_date.year();
  let start_month = start_date.month();
  if start_year == end_date.year() && start_month == end_date.month() {
    // If range is within one month
    if start_date.day() == 1 && end_date.day() == last_day_of_month(start_year, start_month) {
      // If whole month, don't get individual days
      Vec::new()
    } else {
      (start_date.day()..=end_date.day())
        .map(|day| (start_year, start_month, day))
        .collect()
    }
  } else if start_date.day() > 1 {
    // If start date is not first day of month, fetch them separately
    let last_day_of_month = last_day_of_month(start_year, start_month);
    (start_date.day()..=last_day_of_month)
      .map(|day| (start_year, start_month, day))
      .collect()
  } else {
    // Ignore days, just fetch whole month
    Vec::new()
  }
}

fn leading_months(start_date: NaiveDate, end_date: NaiveDate) -> Vec<(i32, u32)> {
  let start_year = start_date.year();
  let start_month = start_date.month();
  if start_year == end_date.year() {
    if start_month == 1 && start_date.day() == 1 && end_date.month() == 12 && end_date.day() == 31 {
      // If one whole year, don't fetch individual months
      Vec::new()
    } else {
      let start_month = if start_date.day() == 1 {
        start_month
      } else {
        start_month + 1
      };

      let end_month = if end_date.day() == last_day_of_month(end_date.year(), end_date.month()) {
        end_date.month()
      } else {
        end_date.month() - 1
      };

      (start_month..=end_month)
        .map(|month| (start_year, month))
        .collect()
    }
  } else if start_month == 1 && start_date.day() > 1 {
    // If whole leading year, don't fetch individual months
    Vec::new()
  } else {
    (start_month..=12)
      .map(|month| (start_year, month))
      .collect()
  }
}

fn middle_years(start_date: NaiveDate, end_date: NaiveDate) -> Vec<i32> {
  let start_year = if start_date.month() == 1 && start_date.day() == 1 {
    start_date.year()
  } else {
    start_date.year() + 1
  };

  let end_year = if end_date.month() == 12 && end_date.day() == 31 {
    end_date.year()
  } else {
    end_date.year() - 1
  };
  (start_year..=end_year).collect()
}

fn trailing_months(start_date: NaiveDate, end_date: NaiveDate) -> Vec<(i32, u32)> {
  let end_year = end_date.year();
  if end_year > start_date.year() {
    (1..end_date.month())
      .map(|month| (end_year, month))
      .collect()
  } else {
    Vec::new()
  }
}

fn trailing_days(start_date: NaiveDate, end_date: NaiveDate) -> Vec<(i32, u32, u32)> {
  let end_year = end_date.year();
  let end_month = end_date.month();
  let last_day_of_month = last_day_of_month(end_year, end_month);
  if end_year == start_date.year() && end_month == start_date.month()
    || end_date.day() == last_day_of_month
  {
    // For ranges within one month there are no trailing days
    // If the end of range is last day of month, fetch whole month instead of
    // individual days
    Vec::new()
  } else {
    (1..=end_date.day())
      .map(|day| (end_year, end_month, day))
      .collect()
  }
}
