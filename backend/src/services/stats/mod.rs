mod cache;
mod data;
mod util;

use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Local};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
  repositories::{Period, StudyPeriodRepository, StudyYearRepository, UserSessionRepository},
  schema::stats::Stat,
  RedisPool,
};

use self::util::{day_date_bounds, month_date_bounds, year_date_bounds};

pub type DateTimeRange = (DateTime<Local>, DateTime<Local>);

pub type Stats = HashMap<Uuid, Stat>;
pub struct StatsService {
  user_session_repo: UserSessionRepository,
  study_year_repo: StudyYearRepository,
  study_period_repo: StudyPeriodRepository,
  redis_pool: RedisPool,
  earliest_date: Mutex<Option<DateTime<Local>>>,
}

impl Clone for StatsService {
  fn clone(&self) -> Self {
    Self::new(
      self.user_session_repo.clone(),
      self.study_year_repo.clone(),
      self.study_period_repo.clone(),
      self.redis_pool.clone(),
    )
  }
}

impl StatsService {
  pub fn new(
    user_session_repo: UserSessionRepository,
    study_year_repo: StudyYearRepository,
    study_period_repo: StudyPeriodRepository,
    redis_pool: RedisPool,
  ) -> Self {
    Self {
      user_session_repo,
      redis_pool,
      earliest_date: Mutex::new(None),
      study_year_repo,
      study_period_repo,
    }
  }

  pub async fn get_day(&self, year: i32, month: u32, day: u32) -> Result<Stats> {
    let (start_date, end_date) = day_date_bounds(year, month, day);
    self.get_range(start_date, end_date).await
  }

  pub async fn get_month(&self, year: i32, month: u32) -> Result<Stats> {
    let (start_date, end_date) = month_date_bounds(year, month);
    self.get_range(start_date, end_date).await
  }

  pub async fn get_study_period(&self, year: i32, period: Period) -> Result<Stats> {
    let (start_date, end_date) = self
      .study_period_repo
      .get_by_year_and_period(year, period)
      .await?;
    self.get_range(start_date, end_date).await
  }

  pub async fn get_study_year(&self, year: i32) -> Result<Stats> {
    let (start_date, end_date) = self.study_year_repo.get_by_year(year).await?;
    self.get_range(start_date, end_date).await
  }

  pub async fn get_year(&self, year: i32) -> Result<Stats> {
    let (start_date, end_date) = year_date_bounds(year);
    self.get_range(start_date, end_date).await
  }

  pub async fn get_lifetime(&self) -> Result<Stats> {
    let now = Local::now();
    let earliest_date = self.get_earliest_date().await?;
    let start_date = earliest_date.date().naive_local();
    let end_date = now.date().naive_local();
    self.get_range(start_date, end_date).await
  }
}
