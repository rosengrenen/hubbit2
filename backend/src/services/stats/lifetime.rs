use super::{util::join_stats, Stats, StatsService};
use anyhow::Result;
use chrono::{Datelike, Local};
use juniper::futures::future::join_all;
use std::collections::HashMap;

impl StatsService {
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
}
