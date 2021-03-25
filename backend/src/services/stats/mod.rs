mod data_storage;
mod day;
mod lifetime;
mod month;
mod study_period;
mod study_year;
mod util;
mod year;

use std::collections::HashMap;

use chrono::{DateTime, Local, TimeZone};
use lazy_static::lazy_static;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
  repositories::{StudyPeriodRepository, StudyYearRepository, UserSessionRepository},
  schema::stats::Stat,
  RedisPool,
};

pub type DateTimeRange = (DateTime<Local>, DateTime<Local>);

lazy_static! {
  pub static ref MIN_DATETIME: DateTime<Local> = Local.ymd(2000, 1, 1).and_hms(0, 0, 0);
  pub static ref MAX_DATETIME: DateTime<Local> = Local.ymd(2099, 12, 31).and_hms(23, 59, 59);
}

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
}
