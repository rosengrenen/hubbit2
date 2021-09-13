use chrono::NaiveDate;
use sqlx::PgPool;

use crate::{error::HubbitResult, models::StudyYear};

#[derive(Clone, Debug)]
pub struct StudyYearRepository {
  pool: PgPool,
}

impl StudyYearRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }

  pub async fn get_by_year(&self, year: i32) -> HubbitResult<(NaiveDate, NaiveDate)> {
    let study_year: StudyYear = sqlx::query_as!(
      StudyYear,
      "
SELECT *
FROM study_years
WHERE year = $1
      ",
      year
    )
    .fetch_one(&self.pool)
    .await?;

    Ok((study_year.start_date, study_year.end_date))
  }
}
