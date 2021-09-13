use sqlx::{
  types::chrono::{DateTime, Local},
  PgPool,
};
use uuid::Uuid;

use crate::{error::HubbitResult, models::UserSession};

#[derive(Clone, Debug)]
pub struct UserSessionRepository {
  pool: PgPool,
}

impl UserSessionRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }

  pub async fn get_range(
    &self,
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
  ) -> HubbitResult<Vec<UserSession>> {
    Ok(
      sqlx::query_as!(
        UserSession,
        "
SELECT *
FROM user_sessions
WHERE end_time > $1 AND start_time < $2
ORDER BY start_time DESC
        ",
        start_time,
        end_time
      )
      .fetch_all(&self.pool)
      .await?,
    )
  }

  pub async fn get_range_for_user(
    &self,
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
    user_id: Uuid,
  ) -> HubbitResult<Vec<UserSession>> {
    Ok(
      sqlx::query_as!(
        UserSession,
        "
SELECT *
FROM user_sessions
WHERE user_id = $1 AND end_time > $2 AND start_time < $3
ORDER BY start_time DESC
        ",
        user_id,
        start_time,
        end_time
      )
      .fetch_all(&self.pool)
      .await?,
    )
  }

  pub async fn get_active(&self) -> HubbitResult<Vec<UserSession>> {
    Ok(
      sqlx::query_as!(
        UserSession,
        "
SELECT *
FROM user_sessions
WHERE end_time > NOW()
ORDER BY start_time DESC
        ",
      )
      .fetch_all(&self.pool)
      .await?,
    )
  }

  pub async fn update_sessions(&self, user_ids: &[Uuid]) -> HubbitResult<()> {
    let active_sessions: Vec<UserSession> = sqlx::query_as!(
      UserSession,
      "
UPDATE user_sessions
SET end_time = NOW() + (5 * interval '1 minute')
WHERE user_id = ANY($1) AND end_time > NOW()
RETURNING *
      ",
      user_ids
    )
    .fetch_all(&self.pool)
    .await?;

    let inactive_user_ids = user_ids
      .iter()
      .filter(|&user_id| {
        !active_sessions
          .iter()
          .any(|active_sesh| active_sesh.user_id == *user_id)
      })
      .map(|user_id| user_id.to_owned())
      .collect::<Vec<Uuid>>();

    sqlx::query!(
      "
INSERT INTO user_sessions (user_id, start_time, end_time)
SELECT user_id, NOW(), NOW() + (5 * interval '1 minute')
FROM UNNEST($1::uuid[]) as user_id
      ",
      &inactive_user_ids
    )
    .fetch_all(&self.pool)
    .await?;
    Ok(())
  }
}
