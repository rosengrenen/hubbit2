use crate::models::{GammaUser, User};
use anyhow::Result;
use reqwest::{header::AUTHORIZATION, Client, Url};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository;

impl UserRepository {
  pub fn new() -> Self {
    Self {}
  }

  pub async fn get_by_id(&self, id: Uuid) -> Result<User> {
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
}
