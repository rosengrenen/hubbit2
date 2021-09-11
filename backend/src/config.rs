use std::env;

#[derive(Clone, Debug)]
pub struct Config {
  pub port: String,
  pub db_url: String,
  pub redis_url: String,
  pub gamma_url: String,
  pub gamma_api_key: String,
  pub gamma_client_id: String,
  pub gamma_client_secret: String,
}

impl Config {
  pub fn from_env() -> Self {
    Self {
      port: env::var("PORT").unwrap(),
      db_url: env::var("DATABASE_URL").unwrap(),
      redis_url: env::var("REDIS_URL").unwrap(),
      gamma_url: env::var("GAMMA_URL").unwrap(),
      gamma_api_key: env::var("GAMMA_API_KEY").unwrap(),
      gamma_client_id: env::var("GAMMA_CLIENT_ID").unwrap(),
      gamma_client_secret: env::var("GAMMA_CLIENT_SECRET").unwrap(),
    }
  }
}
