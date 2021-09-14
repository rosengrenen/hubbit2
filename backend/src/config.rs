use std::{env, str::FromStr};

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
  pub session_lifetime_s: f64,
}

impl Config {
  pub fn from_env() -> Result<Self, ConfigError> {
    Ok(Self {
      port: try_read_var("PORT")?,
      db_url: try_read_var("DATABASE_URL")?,
      redis_url: try_read_var("REDIS_URL")?,
      gamma_public_url: try_read_var("GAMMA_PUBLIC_URL")?,
      gamma_internal_url: try_read_var("GAMMA_INTERNAL_URL")?,
      gamma_api_key: try_read_var("GAMMA_API_KEY")?,
      gamma_client_id: try_read_var("GAMMA_CLIENT_ID")?,
      gamma_client_secret: try_read_var("GAMMA_CLIENT_SECRET")?,
      session_lifetime_s: try_read_var("SESSION_LIFETIME_SECONDS")?,
    })
  }
}

fn try_read_var<T: FromStr>(name: &str) -> Result<T, ConfigError> {
  let value = env::var(name).map_err(|_| ConfigError::UndefinedVar(name.to_string()))?;
  Ok(
    value
      .parse::<T>()
      .map_err(|_| ConfigError::InvalidVar(name.to_string()))?,
  )
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum ConfigError {
  #[error("Environment variable {0} not defined")]
  UndefinedVar(String),
  #[error("Environment variable {0} is invalid")]
  InvalidVar(String),
}
