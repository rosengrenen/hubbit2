use std::collections::HashMap;

use crate::{
  models::{GammaUser, User},
  RedisPool,
};
use anyhow::Result;
use chrono::{DateTime, Local};
use mobc_redis::{redis::RedisError, AsyncCommands};
use reqwest::{header::AUTHORIZATION, Client, Url};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
struct UserEntry {
  updated_at: DateTime<Local>,
  user: User,
}

pub struct UserRepository {
  redis_pool: RedisPool,
  local_cache: RwLock<HashMap<Uuid, User>>,
}

impl Clone for UserRepository {
  fn clone(&self) -> Self {
    Self::new(self.redis_pool.clone())
  }
}

impl UserRepository {
  pub fn new(redis_pool: RedisPool) -> Self {
    Self {
      redis_pool,
      local_cache: RwLock::new(HashMap::new()),
    }
  }

  pub async fn get_by_id(&self, id: Uuid, wait_for_new_data: bool) -> Result<User> {
    // Check local cache
    if let Some(user) = self.local_cache.read().await.get(&id) {
      println!("got {} from local", id);
      return Ok(user.clone());
    }

    // Check redis cache
    // * If data is fresh, return it
    // * If data is old and wait_for_new_data is false, return old data
    //   and fetch new data in background
    // * If data is old and wait_for_new_data is true, pretend that the data didn't exist
    //   and do the normal routine of fetch data -> return (while storing to redis in the background)
    let mut redis_conn = self.redis_pool.get().await.unwrap();
    let key = format!("user:{}", id);
    let raw_user_entry: Result<String, RedisError> = redis_conn.get(&key).await;
    if let Ok(raw_user_entry) = raw_user_entry {
      println!("got {} from redis", id);
      if let Ok(user_entry) = serde_json::from_str::<UserEntry>(&raw_user_entry) {
        let mins_since_update = Local::now()
          .signed_duration_since(user_entry.updated_at)
          .num_minutes();
        if mins_since_update < 120 {
          self
            .local_cache
            .write()
            .await
            .insert(id, user_entry.user.clone());
          return Ok(user_entry.user);
        } else if mins_since_update >= 120 && !wait_for_new_data {
          self
            .local_cache
            .write()
            .await
            .insert(id, user_entry.user.clone());
          let key = key.clone();
          tokio::spawn(async move {
            let user = get_gamma_user(id).await.unwrap();
            let user_entry = UserEntry {
              user,
              updated_at: Local::now(),
            };
            redis_conn
              .set::<String, String, String>(key, serde_json::to_string(&user_entry).unwrap())
              .await
              .unwrap();
          });
          return Ok(user_entry.user);
        }
      }
    }

    let user = get_gamma_user(id).await.unwrap();
    self.local_cache.write().await.insert(id, user.clone());
    let user_entry = UserEntry {
      user: user.clone(),
      updated_at: Local::now(),
    };
    tokio::spawn(async move {
      redis_conn
        .set::<String, String, String>(key, serde_json::to_string(&user_entry).unwrap())
        .await
        .unwrap();
    });
    Ok(user)
  }
}

async fn get_gamma_user(id: Uuid) -> Result<User> {
  let gamma_api_key = std::env::var("GAMMA_API_KEY").unwrap();
  let client = Client::new();
  let body = client
    .get(Url::parse(&format!(
      "https://gamma.chalmers.it/api/users/{}",
      id
    ))?)
    .header(AUTHORIZATION, format!("pre-shared {}", gamma_api_key))
    .send()
    .await?
    .text()
    .await?;
  let gamma_user: GammaUser = serde_json::from_str(&body)?;
  let user = User {
    id,
    nick: gamma_user.nick,
    first_name: gamma_user.first_name,
    last_name: gamma_user.last_name,
    avatar_url: gamma_user.avatar_url,
    groups: gamma_user
      .groups
      .iter()
      .filter_map(|group| {
        if group.active {
          Some(group.super_group.id)
        } else {
          None
        }
      })
      .collect(),
  };
  Ok(user)
}
