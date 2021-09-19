use std::{collections::HashMap, sync::Arc};

use async_graphql::futures_util::future::join_all;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
  error::HubbitResult,
  models::GammaUser,
  repositories::user::UserRepository,
  services::util::{redis_get, redis_mget, redis_set},
  RedisPool,
};

const CACHE_VALID_MINUTES: i64 = 120;

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

  pub async fn get_by_id(&self, id: Uuid, wait_for_new_data: bool) -> HubbitResult<GammaUser> {
    // Check local cache
    if let Some(user) = self.get_cache(id).await {
      return Ok(user);
    }

    // If not in local cache, check redis
    let key = format!("user:id:{}", id);
    if let Ok(user_entry) = redis_get::<UserEntry>(self.redis_pool.clone(), &key).await {
      let mins_since_update = Local::now()
        .signed_duration_since(user_entry.updated_at)
        .num_minutes();
      if mins_since_update < CACHE_VALID_MINUTES {
        // If data exists in cache and is valid, store it in local cache and return
        self.store_user_in_cache(user_entry.user.clone()).await;
        return Ok(user_entry.user);
      } else if mins_since_update >= CACHE_VALID_MINUTES && !wait_for_new_data {
        // If data exists in cache and isn't valid, but wait_for_new_data is false,
        // return old data and spawn task in background to fetch new data
        self.store_user_in_cache(user_entry.user.clone()).await;
        let redis_pool = self.redis_pool.clone();
        let user_repo = self.user_repo.clone();
        tokio::spawn(async move {
          Self::fetch_and_store_user_redis(user_repo, redis_pool, id.to_string()).await
        });

        return Ok(user_entry.user);
      }
    }

    // If in neither local cache or redis, fetch the user
    Ok(self.fetch_and_store_user(id.to_string()).await?)
  }

  pub async fn get_by_cid(&self, cid: String) -> HubbitResult<GammaUser> {
    let key = format!("user:cid:{}", cid);
    if let Ok(id) = redis_get::<Uuid>(self.redis_pool.clone(), &key).await {
      self.get_by_id(id, false).await
    } else {
      self.fetch_and_store_user(cid).await
    }
  }

  pub async fn get_by_ids(
    &self,
    ids: &[Uuid],
    wait_for_new_data: bool,
  ) -> HubbitResult<Vec<GammaUser>> {
    // Check local cache
    let (mut users, non_local_cached_ids) = self.get_cache_many(ids).await;

    // Check redis cache
    // * If data is fresh, return it
    // * If data is old and wait_for_new_data is false, return old data
    //   and fetch new data in background
    // * If data is old and wait_for_new_data is true, pretend that the data didn't exist
    //   and do the normal routine of fetch data -> return (while storing to redis in the background)
    let mut non_redis_cached_ids = vec![];
    let keys = non_local_cached_ids
      .iter()
      .map(|id| format!("user:id:{}", id))
      .collect::<Vec<_>>();
    if let Ok(user_entries) =
      redis_mget::<UserEntry>(self.redis_pool.clone(), keys.as_slice()).await
    {
      for (i, user_entry) in user_entries.iter().enumerate() {
        if let Some(user_entry) = user_entry {
          let mins_since_update = Local::now()
            .signed_duration_since(user_entry.updated_at)
            .num_minutes();
          if mins_since_update < CACHE_VALID_MINUTES {
            users.push(user_entry.user.clone());
          } else if mins_since_update >= CACHE_VALID_MINUTES && !wait_for_new_data {
            let id = user_entry.user.id;
            users.push(user_entry.user.clone());
            let redis_pool = self.redis_pool.clone();
            let user_repo = self.user_repo.clone();
            tokio::spawn(async move {
              Self::fetch_and_store_user_redis(user_repo, redis_pool, id.to_string()).await
            });
          }
        } else {
          non_redis_cached_ids.push(non_local_cached_ids[i]);
        }
      }
    }

    let futs = non_redis_cached_ids
      .iter()
      .map(|id| {
        let user_repo = self.user_repo.clone();
        let redis_pool = self.redis_pool.clone();
        async move {
          match user_repo.get(id.to_string()).await {
            Ok(user) => {
              let user_entry = UserEntry {
                user: user.clone(),
                updated_at: Local::now(),
              };
              tokio::spawn(async move { Self::store_user_redis(redis_pool, user_entry).await });
              Ok(user)
            }
            e => e,
          }
        }
      })
      .collect::<Vec<_>>();
    let non_cached_users = join_all(futs)
      .await
      .into_iter()
      .collect::<HubbitResult<Vec<_>>>()?;

    for user in non_cached_users {
      self.store_user_in_cache(user.clone()).await;
      users.push(user);
    }

    Ok(users)
  }

  async fn fetch_and_store_user(&self, id: String) -> HubbitResult<GammaUser> {
    let user =
      Self::fetch_and_store_user_redis(self.user_repo.clone(), self.redis_pool.clone(), id).await?;
    self.store_user_in_cache(user.clone()).await;
    Ok(user)
  }

  async fn fetch_and_store_user_redis(
    user_repo: UserRepository,
    redis_pool: RedisPool,
    id: String,
  ) -> HubbitResult<GammaUser> {
    let user = user_repo.get(id).await?;
    let user_entry = UserEntry {
      user: user.clone(),
      updated_at: Local::now(),
    };
    tokio::spawn(async move { Self::store_user_redis(redis_pool, user_entry).await });
    Ok(user)
  }

  async fn store_user_redis(redis_pool: RedisPool, user_entry: UserEntry) -> HubbitResult<()> {
    let id_key = format!("user:id:{}", user_entry.user.id);
    let cid_key = format!("user:cid:{}", user_entry.user.cid);
    redis_set(redis_pool.clone(), cid_key, user_entry.user.id).await?;
    redis_set(redis_pool, id_key, user_entry).await?;
    Ok(())
  }

  async fn store_user_in_cache(&self, user: GammaUser) {
    let user_lock = self.get_user_cache_lock(user.id).await;
    let mut user_lock = user_lock.lock().await;
    *user_lock = Some(user);
  }

  async fn get_user_cache_lock(&self, id: Uuid) -> Arc<Mutex<Option<GammaUser>>> {
    let mut cache_lock = self.local_cache.lock().await;
    if let Some(cache_entry) = cache_lock.get(&id) {
      cache_entry.clone()
    } else {
      let cache_entry = Arc::new(Mutex::new(None));
      cache_lock.insert(id, cache_entry.clone());
      cache_entry
    }
  }

  async fn get_cache(&self, id: Uuid) -> Option<GammaUser> {
    let cache_lock = self.local_cache.lock().await;
    if let Some(user) = cache_lock.get(&id) {
      user.lock().await.clone()
    } else {
      None
    }
  }

  async fn get_cache_many(&self, user_ids: &[Uuid]) -> (Vec<GammaUser>, Vec<Uuid>) {
    let mut missing_user_ids = vec![];
    let mut users = vec![];
    let cache_lock = self.local_cache.lock().await;
    for id in user_ids {
      if let Some(user) = cache_lock.get(id) {
        match &*user.lock().await {
          Some(user) => users.push(user.clone()),
          None => missing_user_ids.push(*id),
        }
      } else {
        missing_user_ids.push(*id);
      }
    }

    (users, missing_user_ids)
  }
}
