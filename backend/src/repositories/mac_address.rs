use crate::models::MacAddress;
use anyhow::{bail, Result};
use sqlx::{Pool, Postgres};

pub struct MacAddressRepository<'a> {
  pool: &'a Pool<Postgres>,
}

impl<'a> MacAddressRepository<'a> {
  pub fn new(pool: &'a Pool<Postgres>) -> Self {
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
    .fetch_all(self.pool)
    .await
    {
      Ok(mac_addrs) => Ok(mac_addrs),
      Err(_) => bail!("Something went wrong"),
    }
  }
}
