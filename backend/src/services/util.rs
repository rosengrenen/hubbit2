use anyhow::{bail, Result};
use mobc_redis::{redis::RedisError, AsyncCommands};
use serde::{de::DeserializeOwned, Serialize};

use crate::RedisPool;

pub async fn redis_get<T: DeserializeOwned>(redis_pool: RedisPool, key: &str) -> Result<T> {
  let mut redis_conn = redis_pool.get().await?;
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

pub async fn redis_set<T>(redis_pool: RedisPool, key: String, value: T) -> Result<()>
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
