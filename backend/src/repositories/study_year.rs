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

  pub async fn get_by_year(&self, year: i32) -> HubbitResult<StudyYear> {
    Ok(
      sqlx::query_as!(
        StudyYear,
        "
SELECT *
FROM study_years
WHERE year = $1
      ",
        year
      )
      .fetch_one(&self.pool)
      .await?,
    )
  }

  pub async fn get_current(&self) -> HubbitResult<StudyYear> {
    Ok(
      sqlx::query_as!(
        StudyYear,
        "
SELECT *
FROM study_years
WHERE start_date < NOW() AND NOW() < end_date
LIMIT 1
      "
      )
      .fetch_one(&self.pool)
      .await?,
    )
  }
}
