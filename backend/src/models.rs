use actix_web::http::header::Date;
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
  pub mac: String,
  pub start_time: OffsetDateTime,
  pub end_time: OffsetDateTime,
  pub created_at: OffsetDateTime,
  pub updated_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserSession {
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

#[derive(Debug, sqlx::FromRow)]
pub struct StudyYear {
  pub id: Uuid,
  pub year: i32,
  pub start_date: Date,
  pub end_date: Date,
  pub created_at: OffsetDateTime,
  pub updated_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow)]
pub struct StudyPeriod {
  pub id: Uuid,
  pub year: i32,
  pub period: i32,
  pub start_date: Date,
  pub end_date: Date,
  pub created_at: OffsetDateTime,
  pub updated_at: OffsetDateTime,
}
