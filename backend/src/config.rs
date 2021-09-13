use std::env;

#[derive(Clone, Debug)]
pub struct Config {
  pub port: String,
  pub db_url: String,
  pub redis_url: String,
  pub gamma_public_url: String,
  pub gamma_internal_url: String,
  pub gamma_api_key: String,
  pub gamma_client_id: String,
  pub gamma_client_secret: String,
}

impl Config {
  pub fn from_env() -> Result<Self, ConfigError> {
    Ok(Self {
      port: env::var("PORT").map_err(|_| ConfigError::UndefinedVar("PORT".to_string()))?,
      db_url: env::var("DATABASE_URL")
        .map_err(|_| ConfigError::UndefinedVar("DATABASE_URL".to_string()))?,
      redis_url: env::var("REDIS_URL")
        .map_err(|_| ConfigError::UndefinedVar("REDIS_URL".to_string()))?,
      gamma_public_url: env::var("GAMMA_PUBLIC_URL")
        .map_err(|_| ConfigError::UndefinedVar("GAMMA_PUBLIC_URL".to_string()))?,
      gamma_internal_url: env::var("GAMMA_INTERNAL_URL")
        .map_err(|_| ConfigError::UndefinedVar("GAMMA_INTERNAL_URL".to_string()))?,
      gamma_api_key: env::var("GAMMA_API_KEY")
        .map_err(|_| ConfigError::UndefinedVar("GAMMA_API_KEY".to_string()))?,
      gamma_client_id: env::var("GAMMA_CLIENT_ID")
        .map_err(|_| ConfigError::UndefinedVar("GAMMA_CLIENT_ID".to_string()))?,
      gamma_client_secret: env::var("GAMMA_CLIENT_SECRET")
        .map_err(|_| ConfigError::UndefinedVar("GAMMA_CLIENT_SECRET".to_string()))?,
    })
  }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum ConfigError {
  #[error("Environment variable {0} not defined")]
  UndefinedVar(String),
}
