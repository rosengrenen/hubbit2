use super::{
  util::{join_stats, year_start_end},
  Stats, StatsService,
};
use crate::{
  schema::stats::Stat,
  services::util::{redis_get, redis_set},
};
use anyhow::Result;
use chrono::{Datelike, Local};
use juniper::futures::future::join_all;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct PartialYear {
  month: u32,
  stats: HashMap<Uuid, Stat>,
}

impl StatsService {
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
      let mut partial_entry: PartialYear = redis_get(self.redis_pool.clone(), &key)
        .await
        .unwrap_or_default();

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
      tokio::spawn(async move { redis_set(redis_pool, key, new_partial_entry).await });

      join_stats(&mut partial_entry.stats, &current_month_stats);
      Ok(partial_entry.stats)
    } else {
      if let Ok(stats) = redis_get(self.redis_pool.clone(), &key).await {
        return Ok(stats);
      }

      let (start_time, end_time) = year_start_end(year);
      let stats = self.get_stats_for_range(start_time, end_time).await?;
      let stats_clone = stats.clone();
      let redis_pool = self.redis_pool.clone();
      tokio::spawn(async move { redis_set(redis_pool, key, stats_clone).await });

      Ok(stats)
    }
  }
}
