use std::{collections::HashMap, sync::Arc};

use crate::{
  models::User,
  repositories::UserRepository,
  services::util::{redis_get, redis_set},
  RedisPool,
};
use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
struct UserEntry {
  updated_at: DateTime<Local>,
  user: User,
}

pub struct UserService {
  user_repo: UserRepository,
  redis_pool: RedisPool,
  local_cache: Mutex<HashMap<Uuid, Arc<Mutex<Option<User>>>>>,
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

  pub async fn get_by_id(&self, id: Uuid, wait_for_new_data: bool) -> Result<User> {
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

  // pub async fn get_by_ids(&self, ids: &[Uuid], wait_for_new_data: bool) -> Result<Vec<User>> {
  //   // Check local cache
  //   let mut non_cached_ids = vec![];
  //   let mut users = vec![];
  //   for id in ids.iter() {
  //     if let Some(user) = self.local_cache.read().await.get(&id) {
  //       users.push(user.clone());
  //     } else {
  //       non_cached_ids.push(id);
  //     }
  //   }

  //   // Check redis cache
  //   // * If data is fresh, return it
  //   // * If data is old and wait_for_new_data is false, return old data
  //   //   and fetch new data in background
  //   // * If data is old and wait_for_new_data is true, pretend that the data didn't exist
  //   //   and do the normal routine of fetch data -> return (while storing to redis in the background)
  //   let mut redis_conn = self.redis_pool.get().await.unwrap();
  //   let key = format!("user:{}", id);
  //   let raw_user_entry: Result<String, RedisError> = redis_conn.get(&key).await;
  //   if let Ok(raw_user_entry) = raw_user_entry {
  //     if let Ok(user_entry) = serde_json::from_str::<RedisUserEntry>(&raw_user_entry) {
  //       let mins_since_update = Local::now()
  //         .signed_duration_since(user_entry.updated_at)
  //         .num_minutes();
  //       if mins_since_update < 120 {
  //         self
  //           .local_cache
  //           .write()
  //           .await
  //           .insert(id, user_entry.user.clone());
  //         return Ok(user_entry.user);
  //       } else if mins_since_update >= 120 && !wait_for_new_data {
  //         self
  //           .local_cache
  //           .write()
  //           .await
  //           .insert(id, user_entry.user.clone());
  //         let key = key.clone();
  //         let user_repo = self.user_repo.clone();
  //         tokio::spawn(async move {
  //           let user = user_repo.get_by_id(id).await.unwrap();
  //           let user_entry = RedisUserEntry {
  //             user,
  //             updated_at: Local::now(),
  //           };
  //           redis_conn
  //             .set::<String, String, String>(key, serde_json::to_string(&user_entry).unwrap())
  //             .await
  //             .unwrap();
  //         });
  //         return Ok(user_entry.user);
  //       }
  //     }
  //   }

  //   let user = self.user_repo.get_by_id(id).await.unwrap();
  //   self.local_cache.write().await.insert(id, user.clone());
  //   let user_entry = RedisUserEntry {
  //     user: user.clone(),
  //     updated_at: Local::now(),
  //   };
  //   tokio::spawn(async move {
  //     redis_conn
  //       .set::<String, String, String>(key, serde_json::to_string(&user_entry).unwrap())
  //       .await
  //       .unwrap();
  //   });
  //   Ok(user)
  // }
}
