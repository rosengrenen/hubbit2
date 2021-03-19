use crate::models::MacAddress;
use anyhow::{bail, Result};
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct MacAddressRepository {
  pool: PgPool,
}

impl MacAddressRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }

  pub async fn get_by_addrs(&self, mac_addrs: &[String]) -> Result<Vec<MacAddress>> {
    match sqlx::query_as!(
      MacAddress,
      "
SELECT *
FROM mac_addresses
WHERE address = ANY($1)
      ",
      mac_addrs
    )
    .fetch_all(&self.pool)
    .await
    {
      Ok(mac_addrs) => Ok(mac_addrs),
      Err(_) => bail!("Something went wrong"),
    }
  }
}
