use sqlx::PgPool;

use crate::{error::HubbitResult, models::ApiKey};

#[derive(Clone, Debug)]
pub struct ApiKeyRepository {
  pool: PgPool,
}

impl ApiKeyRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }

  pub async fn get_by_key(&self, key: &str) -> HubbitResult<ApiKey> {
    Ok(
      sqlx::query_as!(
        ApiKey,
        "
SELECT *
FROM api_keys
WHERE token = $1
        ",
        key
      )
      .fetch_one(&self.pool)
      .await?,
    )
  }
}
