use crate::models::UserSession;
use anyhow::{bail, Result};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct UserSessionRepository<'a> {
  pool: &'a Pool<Postgres>,
}

impl<'a> UserSessionRepository<'a> {
  pub fn new(pool: &'a Pool<Postgres>) -> Self {
    Self { pool }
  }

  pub async fn update_sessions(&self, user_ids: &[Uuid]) -> Result<()> {
    let active_sessions: Vec<UserSession> = match sqlx::query_as!(
      UserSession,
      "
UPDATE user_sessions
SET end_time = NOW() + (5 * interval '1 minute')
WHERE user_id = ANY($1) AND end_time > NOW()
RETURNING *
    ",
      user_ids
    )
    .fetch_all(self.pool)
    .await
    {
      Ok(sessions) => sessions,
      Err(_) => bail!("Something went wrong"),
    };

    let inactive_user_ids = user_ids
      .into_iter()
      .filter(|&user_id| {
        active_sessions
          .iter()
          .find(|&active_sesh| active_sesh.user_id == *user_id)
          .is_none()
      })
      .map(|user_id| user_id.to_owned())
      .collect::<Vec<Uuid>>();

    match sqlx::query!(
      "
INSERT INTO user_sessions (user_id, start_time, end_time)
SELECT user_id, NOW(), NOW() + (5 * interval '1 minute')
FROM UNNEST($1::uuid[]) as user_id
      ",
      &inactive_user_ids
    )
    .fetch_all(self.pool)
    .await
    {
      Ok(_) => Ok(()),
      Err(_) => bail!("Something went wrong"),
    }
  }
}
