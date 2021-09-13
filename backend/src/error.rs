use actix_web::ResponseError;

#[derive(Debug, thiserror::Error)]
pub enum HubbitError {
  #[error("Config error")]
  ConfigError(#[from] crate::config::ConfigError),
  #[error("Reqwest error")]
  ReqwestError(#[from] reqwest::Error),
  #[error("Serde json error")]
  SerdeJsonError(#[from] serde_json::Error),
  #[error("Redis error")]
  RedisError(#[from] mobc_redis::redis::RedisError),
  #[error("Mobc redis pool error")]
  MobcRedisError(#[from] mobc::Error<mobc_redis::redis::RedisError>),
  #[error("Sqlx error")]
  SqlxError(#[from] sqlx::Error),
  #[error("Io error")]
  IoError(#[from] std::io::Error),
  #[error("Entity not found")]
  NotFound,
}

pub type HubbitResult<T> = Result<T, HubbitError>;

impl ResponseError for HubbitError {}
