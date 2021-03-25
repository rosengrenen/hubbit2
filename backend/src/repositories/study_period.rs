use crate::models::StudyPeriod;
use anyhow::{bail, Result};
use chrono::Date;
use sqlx::{
  types::chrono::{Local, TimeZone},
  PgPool,
};
use std::convert::TryFrom;

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
  ) -> Result<(Date<Local>, Date<Local>)> {
    let period_num: i32 = period.into();
    let study_period: StudyPeriod = match sqlx::query_as!(
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
    .await
    {
      Ok(study_period) => study_period,
      Err(_) => bail!("Something went wrong"),
    };

    let start_date = Local.from_local_date(&study_period.start_date).unwrap();
    let end_date = Local.from_local_date(&study_period.end_date).unwrap();

    Ok((start_date, end_date))
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

impl TryFrom<i32> for Period {
  type Error = anyhow::Error;

  fn try_from(value: i32) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(Self::LP1),
      1 => Ok(Self::LP2),
      2 => Ok(Self::LP3),
      3 => Ok(Self::LP4),
      4 => Ok(Self::Summer),
      _ => bail!("Invalid value, only 0-4 are valid"),
    }
  }
}

impl Into<i32> for Period {
  fn into(self) -> i32 {
    match self {
      Self::LP1 => 0,
      Self::LP2 => 1,
      Self::LP3 => 2,
      Self::LP4 => 3,
      Self::Summer => 4,
    }
  }
}
