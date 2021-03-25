use super::{util::day_start_end, Stats, StatsService};
use anyhow::Result;
use chrono::{Local, TimeZone};
use std::collections::HashMap;

impl StatsService {
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
      tokio::spawn(async move { Self::redis_set(redis_pool, key, stats).await });
    }

    Ok(stats)
  }
}
