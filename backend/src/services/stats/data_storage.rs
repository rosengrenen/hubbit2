use super::{
  util::{calculate_stats, map_sessions},
  Stats, StatsService, MAX_DATETIME, MIN_DATETIME,
};
use crate::RedisPool;
use anyhow::{bail, Result};
use chrono::{DateTime, Local};
use mobc_redis::{redis::RedisError, AsyncCommands};
use serde::{de::DeserializeOwned, Serialize};

impl StatsService {
  pub(super) async fn redis_get<T: DeserializeOwned>(&self, key: &str) -> Result<T> {
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

  pub(super) async fn redis_set<T>(redis_pool: RedisPool, key: String, value: T) -> Result<()>
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

  pub(super) async fn get_stats_for_range(
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

  pub(super) async fn get_earliest_date(&self) -> Result<DateTime<Local>> {
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
      Self::redis_set(redis_pool, "earliest_date".to_owned(), earliest_date_clone).await
    });

    Ok(earliest_date)
  }
}
