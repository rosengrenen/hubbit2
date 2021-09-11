use std::{collections::HashMap, sync::Arc};

use anyhow::{bail, Result};
use async_graphql::futures_util::future::join_all;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
  models::GammaUser,
  repositories::UserRepository,
  services::util::{redis_get, redis_mget, redis_set},
  RedisPool,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserEntry {
  updated_at: DateTime<Local>,
  user: GammaUser,
}

pub struct UserService {
  user_repo: UserRepository,
  redis_pool: RedisPool,
  local_cache: Mutex<HashMap<Uuid, Arc<Mutex<Option<GammaUser>>>>>,
}

impl Clone for UserService {
  fn clone(&self) -> Self {
    Self::new(self.user_repo.clone(), self.redis_pool.clone())
  }
}

impl UserService {
  pub fn new(user_repo: UserRepository, redis_pool: RedisPool) -> Self {
    Self {
      user_repo,
      redis_pool,
      local_cache: Mutex::new(HashMap::new()),
    }
  }

  pub async fn get_by_id(&self, id: Uuid, wait_for_new_data: bool) -> Result<GammaUser> {
    // Check local cache
    let user_entry = {
      let mut cache_lock = self.local_cache.lock().await;
      if let Some(user) = cache_lock.get(&id) {
        if let Some(user) = user.lock().await.clone() {
          return Ok(user);
        } else {
          user.clone()
        }
      } else {
        let user_entry = Arc::new(Mutex::new(None));
        cache_lock.insert(id, user_entry.clone());
        user_entry
      }
    };

    let mut user_lock = user_entry.lock().await;

    // Check redis cache
    // * If data is fresh, return it
    // * If data is old and wait_for_new_data is false, return old data
    //   and fetch new data in background
    // * If data is old and wait_for_new_data is true, pretend that the data didn't exist
    //   and do the normal routine of fetch data -> return (while storing to redis in the background)
    let key = format!("user:{}", id);
    if let Ok(user_entry) = redis_get::<UserEntry>(self.redis_pool.clone(), &key).await {
      let mins_since_update = Local::now()
        .signed_duration_since(user_entry.updated_at)
        .num_minutes();
      if mins_since_update < 120 {
        *user_lock = Some(user_entry.user.clone());
        return Ok(user_entry.user);
      } else if mins_since_update >= 120 && !wait_for_new_data {
        *user_lock = Some(user_entry.user.clone());
        let key = key.clone();
        let redis_pool = self.redis_pool.clone();
        let user_repo = self.user_repo.clone();
        tokio::spawn(async move {
          let user = user_repo.get_by_id(id).await.unwrap().unwrap();
          let user_entry = UserEntry {
            user,
            updated_at: Local::now(),
          };
          redis_set(redis_pool, key, user_entry).await
        });
        return Ok(user_entry.user);
      }
    }

    let user = self.user_repo.get_by_id(id).await.unwrap().unwrap();
    *user_lock = Some(user.clone());
    let user_entry = UserEntry {
      user: user.clone(),
      updated_at: Local::now(),
    };
    let redis_pool = self.redis_pool.clone();
    tokio::spawn(async move { redis_set(redis_pool, key, user_entry).await });
    Ok(user)
  }

  pub async fn get_by_ids(&self, ids: &[Uuid], wait_for_new_data: bool) -> Result<Vec<GammaUser>> {
    // Check local cache
    let mut non_local_cached_ids = vec![];
    let mut users = vec![];
    {
      let cache_lock = self.local_cache.lock().await;
      for id in ids.iter() {
        if let Some(user) = cache_lock.get(id) {
          match &*user.lock().await {
            Some(user) => users.push(user.clone()),
            None => non_local_cached_ids.push(id),
          }
        } else {
          non_local_cached_ids.push(id);
        }
      }
    }

    // Check redis cache
    // * If data is fresh, return it
    // * If data is old and wait_for_new_data is false, return old data
    //   and fetch new data in background
    // * If data is old and wait_for_new_data is true, pretend that the data didn't exist
    //   and do the normal routine of fetch data -> return (while storing to redis in the background)
    let mut non_redis_cached_ids = vec![];
    let keys = non_local_cached_ids
      .iter()
      .map(|id| format!("user:{}", id))
      .collect::<Vec<_>>();
    if let Ok(user_entries) =
      redis_mget::<UserEntry>(self.redis_pool.clone(), keys.as_slice()).await
    {
      for (i, user_entry) in user_entries.iter().enumerate() {
        if let Some(user_entry) = user_entry {
          let mins_since_update = Local::now()
            .signed_duration_since(user_entry.updated_at)
            .num_minutes();
          if mins_since_update < 120 {
            users.push(user_entry.user.clone());
          } else if mins_since_update >= 120 && !wait_for_new_data {
            let id = user_entry.user.id;
            let key = format!("user:{}", id);
            let redis_pool = self.redis_pool.clone();
            let user_repo = self.user_repo.clone();
            tokio::spawn(async move {
              let user = user_repo.get_by_id(id).await.unwrap().unwrap();
              let user_entry = UserEntry {
                user,
                updated_at: Local::now(),
              };
              redis_set(redis_pool, key, user_entry).await
            });
          }
        } else {
          non_redis_cached_ids.push(non_local_cached_ids[i]);
        }
      }
    }

    let mut futs = vec![];
    for id in non_redis_cached_ids {
      let user_repo = self.user_repo.clone();
      let redis_pool = self.redis_pool.clone();
      futs.push(async move {
        if let Ok(Some(user)) = user_repo.get_by_id(*id).await {
          let user_entry = UserEntry {
            user: user.clone(),
            updated_at: Local::now(),
          };
          let id = *id;
          tokio::spawn(
            async move { redis_set(redis_pool, format!("user:{}", id), user_entry).await },
          );
          Ok(user)
        } else {
          bail!("Couldn't get user")
        }
      })
    }
    let non_cached_users = join_all(futs)
      .await
      .into_iter()
      .collect::<Result<Vec<_>>>()?;

    for user in non_cached_users {
      users.push(user);
    }

    {
      let mut cache_lock = self.local_cache.lock().await;
      for user in users.iter() {
        cache_lock.insert(user.id, Arc::new(Mutex::new(Some(user.clone()))));
      }
    }

    Ok(users)
  }
}
