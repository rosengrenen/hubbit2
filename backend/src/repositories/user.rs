use reqwest::{header::AUTHORIZATION, Client};

use crate::{
  config::Config,
  error::{HubbitError, HubbitResult},
  models::GammaUser,
};

#[derive(Clone)]
pub struct UserRepository {
  config: Config,
}

impl UserRepository {
  pub fn new(config: Config) -> Self {
    Self { config }
  }

  pub async fn get(&self, id: String) -> HubbitResult<GammaUser> {
    let client = Client::new();
    let res = client
      .get(&format!(
        "{}/api/users/{}",
        self.config.gamma_internal_url, id
      ))
      .header(
        AUTHORIZATION,
        format!("pre-shared {}", self.config.gamma_api_key),
      )
      .send()
      .await?;
    if res.status() == 404 {
      return Err(HubbitError::NotFound);
    }

    let body = res.text().await?;
    Ok(serde_json::from_str(&body)?)
  }
}
