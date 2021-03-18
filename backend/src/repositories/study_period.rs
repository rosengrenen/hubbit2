use crate::models::StudyPeriod;
use anyhow::{bail, Result};
use chrono::{DateTime, TimeZone, Utc};
use sqlx::{Pool, Postgres};

pub struct StudyPeriodRepository<'a> {
  pool: &'a Pool<Postgres>,
}

impl<'a> StudyPeriodRepository<'a> {
  pub fn new(pool: &'a Pool<Postgres>) -> Self {
    Self { pool }
  }

  pub async fn get_by_year_and_period(
    &self,
    year: i32,
    period: i32,
  ) -> Result<(DateTime<Utc>, DateTime<Utc>)> {
    let study_period: StudyPeriod = match sqlx::query_as!(
      StudyPeriod,
      "
SELECT *
FROM study_periods
WHERE year = $1 AND period = $2
      ",
      year,
      period
    )
    .fetch_one(self.pool)
    .await
    {
      Ok(study_period) => study_period,
      Err(_) => bail!("Something went wrong"),
    };

    Ok((
      Utc
        .from_local_datetime(&study_period.start_date.and_hms(0, 0, 0))
        .unwrap(),
      Utc
        .from_local_datetime(&study_period.end_date.and_hms(23, 59, 59))
        .unwrap(),
    ))
  }
}
