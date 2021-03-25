use crate::models::{GammaUser, User};
use anyhow::Result;
use lazy_static::lazy_static;
use reqwest::{header::AUTHORIZATION, Client, Url};
use uuid::Uuid;

lazy_static! {
  static ref AUTH_HEADER: String =
    format!("pre-shared {}", std::env::var("GAMMA_API_KEY").unwrap());
}

#[derive(Clone)]
pub struct UserRepository;

impl UserRepository {
  pub fn new() -> Self {
    Self {}
  }

  pub async fn get_by_id(&self, id: Uuid) -> Result<Option<User>> {
    println!("Getting {} from gamma", id);
    let client = Client::new();
    let res = client
      .get(Url::parse(&format!("https://gamma.chalmers.it/api/users/{}", id)).unwrap())
      .header(AUTHORIZATION, &*AUTH_HEADER)
      .send()
      .await?;
    if res.status() == 404 {
      return Ok(None);
    }

    let body = res.text().await?;
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
    Ok(Some(user))
  }
}
