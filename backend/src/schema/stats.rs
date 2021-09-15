use async_graphql::{guard::Guard, Context, InputObject, Object, SimpleObject};
use chrono::{DateTime, Local, TimeZone};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::{
  models::Period,
  repositories::study_period::StudyPeriodRepository,
  schema::{AuthGuard, HubbitSchemaError, HubbitSchemaResult},
  services::{stats::StatsService, user::UserService},
};

use super::user::User;

#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct Stat {
  pub user: User,
  pub score: i32,
  pub time: i32,
}

#[derive(InputObject)]
pub struct StatsInput {
  year_stats: Option<YearStatsInput>,
  month_stats: Option<MonthStatsInput>,
  day_stats: Option<DayStatsInput>,
  study_year_stats: Option<StudyYearStatsInput>,
  study_period_stats: Option<StudyPeriodStatsInput>,
}

#[derive(InputObject)]
struct YearStatsInput {
  year: i32,
}

#[derive(InputObject)]
struct MonthStatsInput {
  year: i32,
  month: i32,
}

#[derive(InputObject)]
struct DayStatsInput {
  year: i32,
  month: i32,
  day: i32,
}

#[derive(InputObject)]
struct StudyYearStatsInput {
  year: i32,
}

#[derive(InputObject)]
struct StudyPeriodStatsInput {
  year: i32,
  period: Period,
}

lazy_static! {
  static ref MIN_DATETIME: DateTime<Local> = Local.ymd(2000, 1, 1).and_hms(0, 0, 0);
  static ref MAX_DATETIME: DateTime<Local> = Local.ymd(2099, 12, 31).and_hms(23, 59, 59);
}

#[derive(Default)]
pub struct StatsQuery;

#[Object]
impl StatsQuery {
  #[graphql(guard(AuthGuard()))]
  pub async fn stats(
    &self,
    context: &Context<'_>,
    input: Option<StatsInput>,
  ) -> HubbitSchemaResult<Vec<Stat>> {
    // TODO(rasmus): weekly stats
    let stats_service = context.data_unchecked::<StatsService>();
    let stats_result = if let Some(input) = input {
      if let Some(YearStatsInput { year }) = input.year_stats {
        stats_service.get_year(year).await
      } else if let Some(MonthStatsInput { year, month }) = input.month_stats {
        stats_service.get_month(year, month as u32).await
      } else if let Some(DayStatsInput { year, month, day }) = input.day_stats {
        stats_service.get_day(year, month as u32, day as u32).await
      } else if let Some(StudyYearStatsInput { year }) = input.study_year_stats {
        stats_service.get_study_year(year).await
      } else if let Some(StudyPeriodStatsInput { year, period }) = input.study_period_stats {
        stats_service.get_study_period(year, period).await
      } else {
        stats_service.get_lifetime().await
      }
    } else {
      stats_service.get_lifetime().await
    };

    let stats = match stats_result {
      Ok(stats) => stats,
      _ => return Err(HubbitSchemaError::InternalError),
    };

    if context.look_ahead().field("user").exists() {
      // Prefetch users to cache them if user field is queried
      let user_service = context.data_unchecked::<UserService>();
      let user_ids = stats.keys().copied().collect::<Vec<_>>();
      user_service
        .get_by_ids(user_ids.as_slice(), false)
        .await
        .map_err(|_| HubbitSchemaError::InternalError)?;
    }

    let mut stats = stats.into_iter().map(|(_, stat)| stat).collect::<Vec<_>>();
    stats.sort_by_key(|stat| -stat.score);
    Ok(stats)
  }

  #[graphql(guard(AuthGuard()))]
  pub async fn current_study_period(&self, context: &Context<'_>) -> HubbitSchemaResult<Period> {
    let study_period_repo = context.data_unchecked::<StudyPeriodRepository>();
    let current_study_period = study_period_repo
      .get_current()
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    Ok(current_study_period.period.into())
  }
}
