use crate::models::User;
use anyhow::Result;
use reqwest::{header::AUTHORIZATION, Client, Url};
use uuid::Uuid;

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
    let user: User = serde_json::from_str(&body)?;
    Ok(user)
  }
}
