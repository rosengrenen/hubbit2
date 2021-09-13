use chrono::NaiveDate;
use sqlx::PgPool;

use crate::{error::HubbitResult, models::StudyPeriod};

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
  ) -> HubbitResult<(NaiveDate, NaiveDate)> {
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

    Ok((study_period.start_date, study_period.end_date))
  }
}

#[derive(Clone, Copy, Debug)]
pub enum Period {
  LP1,
  LP2,
  LP3,
  LP4,
  Summer,
}

impl From<i32> for Period {
  fn from(value: i32) -> Self {
    match value {
      0 => Self::LP1,
      1 => Self::LP2,
      2 => Self::LP3,
      3 => Self::LP4,
      4 => Self::Summer,
      _ => panic!("Period integer value must be between 0 and 4"),
    }
  }
}

impl From<Period> for i32 {
  fn from(period: Period) -> Self {
    match period {
      Period::LP1 => 0,
      Period::LP2 => 1,
      Period::LP3 => 2,
      Period::LP4 => 3,
      Period::Summer => 4,
    }
  }
}
