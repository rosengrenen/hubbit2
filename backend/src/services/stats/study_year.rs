use super::{
  util::{join_stats, month_start_end},
  Stats, StatsService,
};
use crate::services::util::{redis_get, redis_set};
use anyhow::Result;
use chrono::{Datelike, Local};
use juniper::futures::future::join_all;
use std::collections::HashMap;

impl StatsService {
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
        if start_date.year() != now.year() {
          for month in start_date.month() + 1..=12 {
            month_futures.push(self.get_month(start_date.year(), month));
          }
          for month in 1..cur_month {
            month_futures.push(self.get_month(now.year(), month));
          }
        } else {
          for month in start_month + 1..cur_month {
            month_futures.push(self.get_month(start_date.year(), month));
          }
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
      if let Ok(stats) = redis_get(self.redis_pool.clone(), &key).await {
        return Ok(stats);
      }

      let stats = self
        .get_stats_for_range(start_date.and_hms(0, 0, 0), end_date.and_hms(23, 59, 59))
        .await?;
      let stats_clone = stats.clone();
      let redis_pool = self.redis_pool.clone();
      tokio::spawn(async move { redis_set(redis_pool, key, stats_clone).await });

      Ok(stats)
    }
  }
}
