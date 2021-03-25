use crate::models::StudyYear;
use anyhow::{bail, Result};
use chrono::Date;
use sqlx::{
  types::chrono::{Local, TimeZone},
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

  pub async fn get_by_year(&self, year: i32) -> Result<(Date<Local>, Date<Local>)> {
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

    let start_date = Local.from_local_date(&study_year.start_date).unwrap();
    let end_date = Local.from_local_date(&study_year.end_date).unwrap();

    Ok((start_date, end_date))
  }
}
