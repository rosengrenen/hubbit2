use mobc_redis::AsyncCommands;
use serde::{de::DeserializeOwned, Serialize};

use crate::{error::HubbitResult, RedisPool};

pub async fn redis_get<T: DeserializeOwned>(redis_pool: RedisPool, key: &str) -> HubbitResult<T> {
  let mut redis_conn = redis_pool.get().await?;
  let raw_result: String = redis_conn.get(key).await?;

  Ok(serde_json::from_str::<T>(&raw_result)?)
}

pub async fn redis_set<T>(redis_pool: RedisPool, key: String, value: T) -> HubbitResult<()>
where
  T: Serialize,
{
  let mut redis_conn = redis_pool.get().await?;
  redis_conn
    .set::<String, String, String>(key, serde_json::to_string(&value)?)
    .await?;
  Ok(())
}

pub async fn redis_mget<T: DeserializeOwned>(
  redis_pool: RedisPool,
  keys: &[String],
) -> HubbitResult<Vec<Option<T>>> {
  let mut redis_conn = redis_pool.get().await?;
  let raw_result: Vec<Option<String>> = redis_conn.get(keys).await?;
  Ok(
    raw_result
      .into_iter()
      .map(|raw| -> HubbitResult<Option<T>> {
        match raw {
          Some(raw) => Ok(Some(serde_json::from_str::<T>(&raw)?)),
          None => Ok(None),
        }
      })
      .collect::<HubbitResult<Vec<Option<T>>>>()?,
  )
}
