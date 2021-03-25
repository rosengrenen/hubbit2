use super::{
  util::{join_stats, month_start_end},
  Stats, StatsService,
};
use crate::services::util::{redis_get, redis_set};
use anyhow::Result;
use chrono::{Datelike, Local};
use juniper::futures::future::join_all;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct PartialMonth {
  day: u32,
  stats: Stats,
}

impl StatsService {
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
      let mut partial_entry: PartialMonth = redis_get(self.redis_pool.clone(), &key)
        .await
        .unwrap_or_default();
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
      tokio::spawn(async move { redis_set(redis_pool, key, new_partial_entry).await });

      join_stats(&mut partial_entry.stats, &todays_stats);
      Ok(partial_entry.stats)
    } else {
      if let Ok(stats) = redis_get(self.redis_pool.clone(), &key).await {
        return Ok(stats);
      }

      let (start_time, end_time) = month_start_end(year, month);
      let stats = self.get_stats_for_range(start_time, end_time).await?;
      let stats_clone = stats.clone();
      let redis_pool = self.redis_pool.clone();
      tokio::spawn(async move { redis_set(redis_pool, key, stats_clone).await });

      Ok(stats)
    }
  }
}
