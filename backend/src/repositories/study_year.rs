use crate::models::StudyYear;
use anyhow::{bail, Result};
use chrono::{DateTime, TimeZone, Utc};
use sqlx::{Pool, Postgres};

pub struct StudyYearRepository<'a> {
  pool: &'a Pool<Postgres>,
}

impl<'a> StudyYearRepository<'a> {
  pub fn new(pool: &'a Pool<Postgres>) -> Self {
    Self { pool }
  }

  pub async fn get_by_year(&self, year: i32) -> Result<(DateTime<Utc>, DateTime<Utc>)> {
    let study_year: StudyYear = match sqlx::query_as!(
      StudyYear,
      "
SELECT *
FROM study_years
WHERE year = $1
      ",
      year
    )
    .fetch_one(self.pool)
    .await
    {
      Ok(study_year) => study_year,
      Err(_) => bail!("Something went wrong"),
    };

    Ok((
      Utc
        .from_local_datetime(&study_year.start_date.and_hms(0, 0, 0))
        .unwrap(),
      Utc
        .from_local_datetime(&study_year.end_date.and_hms(23, 59, 59))
        .unwrap(),
    ))
  }
}
