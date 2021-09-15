use sqlx::PgPool;

use crate::{
  error::HubbitResult,
  models::{Period, StudyPeriod},
};

#[derive(Clone, Debug)]
pub struct StudyPeriodRepository {
  pool: PgPool,
}

impl StudyPeriodRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }

  pub async fn get_by_year_and_period(
    &self,
    year: i32,
    period: Period,
  ) -> HubbitResult<StudyPeriod> {
    let period_num: i32 = period.into();
    let study_period: StudyPeriod = sqlx::query_as!(
      StudyPeriod,
      "
SELECT *
FROM study_periods
WHERE year = $1 AND period = $2
      ",
      year,
      period_num
    )
    .fetch_one(&self.pool)
    .await?;

    Ok(study_period)
  }

  pub async fn get_current(&self) -> HubbitResult<StudyPeriod> {
    let study_period: StudyPeriod = sqlx::query_as!(
      StudyPeriod,
      "
SELECT *
FROM study_periods
WHERE start_date < NOW() AND end_date > NOW()
      "
    )
    .fetch_one(&self.pool)
    .await?;

    Ok(study_period)
  }
}
