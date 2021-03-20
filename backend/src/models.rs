use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct MacAddress {
  pub id: Uuid,
  pub user_id: Uuid,
  pub address: String,
  pub device_name: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Session {
  pub id: Uuid,
  pub user_id: Uuid,
  pub mac_address: String,
  pub start_time: DateTime<Utc>,
  pub end_time: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserSession {
  pub id: Uuid,
  pub user_id: Uuid,
  pub start_time: DateTime<Utc>,
  pub end_time: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ApiKey {
  pub id: Uuid,
  pub token: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct StudyYear {
  pub id: Uuid,
  pub year: i32,
  pub start_date: NaiveDate,
  pub end_date: NaiveDate,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct StudyPeriod {
  pub id: Uuid,
  pub year: i32,
  pub period: i32,
  pub start_date: NaiveDate,
  pub end_date: NaiveDate,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct GammaUser {
  pub id: Uuid,
  pub nick: String,
  #[serde(rename = "firstName")]
  pub first_name: String,
  #[serde(rename = "lastName")]
  pub last_name: String,
  #[serde(rename = "avatarUrl")]
  pub avatar_url: String,
  pub groups: Vec<GammaGroup>,
}

#[derive(Debug, Deserialize)]
pub struct GammaGroup {
  pub active: bool,
  #[serde(rename = "superGroup")]
  pub super_group: GammaSuperGroup,
}

#[derive(Debug, Deserialize)]
pub struct GammaSuperGroup {
  pub id: Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
  pub id: Uuid,
  pub nick: String,
  pub first_name: String,
  pub last_name: String,
  pub avatar_url: String,
  pub groups: Vec<Uuid>,
}
