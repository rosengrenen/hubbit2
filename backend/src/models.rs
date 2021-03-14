use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct MacAddress {
  pub id: Uuid,
  pub user_id: Uuid,
  pub mac: String,
  pub description: String,
  pub created_at: OffsetDateTime,
  pub updated_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Session {
  pub id: Uuid,
  pub user_id: Uuid,
  pub start_time: OffsetDateTime,
  pub end_time: OffsetDateTime,
  pub created_at: OffsetDateTime,
  pub updated_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ApiKey {
  pub id: Uuid,
  pub key: String,
  pub created_at: OffsetDateTime,
  pub updated_at: OffsetDateTime,
}
