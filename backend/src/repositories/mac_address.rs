use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::HubbitResult, models::MacAddress};

#[derive(Clone, Debug)]
pub struct MacAddressRepository {
  pool: PgPool,
}

impl MacAddressRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }

  pub async fn get_by_addrs(&self, mac_addrs: &[String]) -> HubbitResult<Vec<MacAddress>> {
    Ok(
      sqlx::query_as!(
        MacAddress,
        "
SELECT *
FROM mac_addresses
WHERE address = ANY($1)
        ",
        mac_addrs
      )
      .fetch_all(&self.pool)
      .await?,
    )
  }

  pub async fn get_for_user(&self, user_id: Uuid) -> HubbitResult<Vec<MacAddress>> {
    Ok(
      sqlx::query_as!(
        MacAddress,
        "
SELECT *
FROM mac_addresses
WHERE user_id = $1
        ",
        user_id
      )
      .fetch_all(&self.pool)
      .await?,
    )
  }
}
