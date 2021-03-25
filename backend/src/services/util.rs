use crate::RedisPool;
use anyhow::{bail, Result};
use mobc_redis::{redis::RedisError, AsyncCommands};
use serde::{de::DeserializeOwned, Serialize};

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

pub async fn redis_mget<T: DeserializeOwned>(
  redis_pool: RedisPool,
  keys: &[String],
) -> Result<Vec<Option<T>>> {
  let mut redis_conn = redis_pool.get().await?;
  let raw_res: Result<Vec<Option<String>>, RedisError> = redis_conn.get(keys).await;
  match raw_res {
    Ok(raw_res) => raw_res
      .into_iter()
      .map(|raw| match raw {
        Some(raw) => match serde_json::from_str::<T>(&raw) {
          Ok(parsed) => Ok(Some(parsed)),
          Err(_) => bail!("Couldn't parse to type"),
        },
        None => Ok(None),
      })
      .collect::<Result<Vec<Option<T>>>>(),
    Err(_) => bail!("Couldn't get key from redis"),
  }
}
