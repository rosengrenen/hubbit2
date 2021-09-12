use crate::{config::Config, models::GammaUser};
use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GammaTokenResponse {
  pub access_token: String,
}

pub async fn oauth2_token(config: &Config, code: &str) -> Result<GammaTokenResponse> {
  let client = Client::new();
  let url = format!(
    "{}/api/oauth/token?grant_type=authorization_code&code={}",
    config.gamma_internal_url, code
  );
  let body_str = client
    .post(&url)
    .basic_auth(&config.gamma_client_id, Some(&config.gamma_client_secret))
    .send()
    .await?
    .text()
    .await?;
  Ok(serde_json::from_str(&body_str)?)
}

pub async fn get_current_user(config: &Config, access_token: &str) -> Result<GammaUser> {
  let client = Client::new();
  let url = format!("{}/api/users/me", config.gamma_internal_url);
  let body_str = client
    .get(&url)
    .bearer_auth(access_token)
    .send()
    .await?
    .text()
    .await?;
  Ok(serde_json::from_str(&body_str)?)
}
