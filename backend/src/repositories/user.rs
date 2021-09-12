use crate::{config::Config, models::GammaUser};
use anyhow::Result;
use lazy_static::lazy_static;
use reqwest::{header::AUTHORIZATION, Client, Url};
use uuid::Uuid;

lazy_static! {
  static ref AUTH_HEADER: String =
    format!("pre-shared {}", std::env::var("GAMMA_API_KEY").unwrap());
}

#[derive(Clone)]
pub struct UserRepository {
  config: Config,
}

impl UserRepository {
  pub fn new(config: Config) -> Self {
    Self { config }
  }

  pub async fn get_by_id(&self, id: Uuid) -> Result<Option<GammaUser>> {
    let client = Client::new();
    let res = client
      .get(
        Url::parse(&format!(
          "{}/api/users/{}",
          self.config.gamma_internal_url, id
        ))
        .unwrap(),
      )
      .header(
        AUTHORIZATION,
        format!("pre-shared {}", self.config.gamma_api_key),
      )
      .send()
      .await?;
    if res.status() == 404 {
      return Ok(None);
    }

    let body = res.text().await?;
    let user: GammaUser = serde_json::from_str(&body)?;
    Ok(Some(user))
  }
}
