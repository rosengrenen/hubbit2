use crate::models::StudyYear;
use anyhow::{bail, Result};
use sqlx::{
  types::chrono::{DateTime, Local, TimeZone},
  PgPool,
};

#[derive(Clone, Debug)]
pub struct StudyYearRepository {
  pool: PgPool,
}

impl StudyYearRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }

  pub async fn get_by_year(&self, year: i32) -> Result<(DateTime<Local>, DateTime<Local>)> {
    let study_year: StudyYear = match sqlx::query_as!(
      StudyYear,
      "
SELECT *
FROM study_years
WHERE year = $1
      ",
      year
    )
    .fetch_one(&self.pool)
    .await
    {
      Ok(study_year) => study_year,
      Err(_) => bail!("Something went wrong"),
    };

    let start_time = Local
      .from_local_date(&study_year.start_date)
      .unwrap()
      .and_hms(0, 0, 0);
    let end_time = Local
      .from_local_date(&study_year.end_date)
      .unwrap()
      .and_hms(23, 59, 59);

    Ok((start_time, end_time))
  }
}
