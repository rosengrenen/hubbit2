use std::collections::HashMap;

use async_graphql::{guard::Guard, Context, InputObject, Object, SimpleObject};
use chrono::{Datelike, Duration, Local, TimeZone};
use log::error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  models::Period,
  repositories::{study_period::StudyPeriodRepository, study_year::StudyYearRepository},
  schema::{AuthGuard, HubbitSchemaError, HubbitSchemaResult},
  services::{
    stats::{Stat as ServiceStat, StatsService},
    user::UserService,
  },
};

use super::user::User;

#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct Stat {
  pub user: User,
  pub duration_seconds: i64,
  pub current_position: i32,
  pub prev_position: Option<i32>,
}

#[derive(InputObject)]
pub struct StatsStudyYearInput {
  year: i32,
}

#[derive(InputObject)]
pub struct StatsStudyPeriodInput {
  year: i32,
  period: Period,
}

#[derive(InputObject)]
pub struct StatsMonthInput {
  year: i32,
  month: i32,
}

#[derive(InputObject)]
pub struct StatsWeekInput {
  year: i32,
  week: i32,
}

#[derive(InputObject)]
pub struct StatsDayInput {
  year: i32,
  month: i32,
  day: i32,
}

#[derive(SimpleObject)]
pub struct StatsStudyYearPayload {
  stats: Vec<Stat>,
  year: i32,
}

#[derive(SimpleObject)]
pub struct StatsStudyPeriodPayload {
  stats: Vec<Stat>,
  year: i32,
  period: Period,
}

#[derive(Default)]
pub struct StatsQuery;

#[Object]
impl StatsQuery {
  #[graphql(guard(AuthGuard()))]
  pub async fn stats_alltime(&self, context: &Context<'_>) -> HubbitSchemaResult<Vec<Stat>> {
    let stats_service = context.data_unchecked::<StatsService>();
    let stats = stats_service.get_alltime().await.map_err(|e| {
      error!("[Schema error] {:?}", e);
      HubbitSchemaError::InternalError
    })?;

    prefetch_users(context, &stats).await?;

    Ok(sort_and_map_stats(stats, &None))
  }

  #[graphql(guard(AuthGuard()))]
  pub async fn stats_study_year(
    &self,
    context: &Context<'_>,
    input: Option<StatsStudyYearInput>,
  ) -> HubbitSchemaResult<StatsStudyYearPayload> {
    let year = if let Some(input) = input {
      input.year
    } else {
      let study_year_repo = context.data_unchecked::<StudyYearRepository>();
      study_year_repo
        .get_current()
        .await
        .map_err(|e| {
          error!("[Schema error] {:?}", e);
          HubbitSchemaError::InternalError
        })?
        .year
    };

    let stats_service = context.data_unchecked::<StatsService>();
    let stats = stats_service.get_study_year(year).await.map_err(|e| {
      error!("[Schema error] {:?}", e);
      HubbitSchemaError::InternalError
    })?;

    let previous_stats = if context
      .look_ahead()
      .field("stats")
      .field("prevPosition")
      .exists()
    {
      stats_service.get_study_year(year - 1).await.ok()
    } else {
      None
    };

    prefetch_users(context, &stats).await?;

    let stats = sort_and_map_stats(stats, &previous_stats);
    Ok(StatsStudyYearPayload { stats, year })
  }

  #[graphql(guard(AuthGuard()))]
  pub async fn stats_study_period(
    &self,
    context: &Context<'_>,
    input: Option<StatsStudyPeriodInput>,
  ) -> HubbitSchemaResult<StatsStudyPeriodPayload> {
    let (year, period) = if let Some(input) = input {
      (input.year, input.period)
    } else {
      let study_period_repo = context.data_unchecked::<StudyPeriodRepository>();
      let study_period = study_period_repo.get_current().await.map_err(|e| {
        error!("[Schema error] {:?}", e);
        HubbitSchemaError::InternalError
      })?;
      (study_period.year, study_period.period.into())
    };

    let stats_service = context.data_unchecked::<StatsService>();
    let stats = stats_service
      .get_study_period(year, period)
      .await
      .map_err(|e| {
        error!("[Schema error] {:?}", e);
        HubbitSchemaError::InternalError
      })?;

    let previous_stats = if context
      .look_ahead()
      .field("stats")
      .field("prevPosition")
      .exists()
    {
      let (prev_year, prev_period) = match period {
        Period::Summer => (year - 1, Period::LP4),
        Period::LP1 => (year, Period::Summer),
        Period::LP2 => (year, Period::LP1),
        Period::LP3 => (year, Period::LP2),
        Period::LP4 => (year, Period::LP3),
      };
      stats_service
        .get_study_period(prev_year, prev_period)
        .await
        .ok()
    } else {
      None
    };

    prefetch_users(context, &stats).await?;

    let stats = sort_and_map_stats(stats, &previous_stats);
    Ok(StatsStudyPeriodPayload {
      stats,
      year,
      period,
    })
  }

  #[graphql(guard(AuthGuard()))]
  pub async fn stats_month(
    &self,
    context: &Context<'_>,
    input: StatsMonthInput,
  ) -> HubbitSchemaResult<Vec<Stat>> {
    let stats_service = context.data_unchecked::<StatsService>();
    let stats = stats_service
      .get_month(input.year, input.month as u32)
      .await
      .map_err(|e| {
        error!("[Schema error] {:?}", e);
        HubbitSchemaError::InternalError
      })?;

    let previous_stats = if context.look_ahead().field("prevPosition").exists() {
      let (prev_year, prev_month) = match input.month {
        1 => (input.year - 1, 12),
        _ => (input.year, input.month - 1),
      };
      stats_service
        .get_month(prev_year, prev_month as u32)
        .await
        .ok()
    } else {
      None
    };

    prefetch_users(context, &stats).await?;

    Ok(sort_and_map_stats(stats, &previous_stats))
  }

  #[graphql(guard(AuthGuard()))]
  pub async fn stats_week(
    &self,
    context: &Context<'_>,
    input: StatsWeekInput,
  ) -> HubbitSchemaResult<Vec<Stat>> {
    let stats_service = context.data_unchecked::<StatsService>();
    let stats = stats_service
      .get_week(input.year, input.week as u32)
      .await
      .map_err(|e| {
        error!("[Schema error] {:?}", e);
        HubbitSchemaError::InternalError
      })?;

    let previous_stats = if context.look_ahead().field("prevPosition").exists() {
      let (prev_year, prev_week) = match input.week {
        1 => (input.year - 1, 52),
        _ => (input.year, input.week - 1),
      };
      stats_service
        .get_week(prev_year, prev_week as u32)
        .await
        .ok()
    } else {
      None
    };

    prefetch_users(context, &stats).await?;

    Ok(sort_and_map_stats(stats, &previous_stats))
  }

  #[graphql(guard(AuthGuard()))]
  pub async fn stats_day(
    &self,
    context: &Context<'_>,
    input: StatsDayInput,
  ) -> HubbitSchemaResult<Vec<Stat>> {
    let stats_service = context.data_unchecked::<StatsService>();
    let stats = stats_service
      .get_day(input.year, input.month as u32, input.day as u32)
      .await
      .map_err(|e| {
        error!("[Schema error] {:?}", e);
        HubbitSchemaError::InternalError
      })?;

    let previous_stats = if context.look_ahead().field("prevPosition").exists() {
      let prev_day =
        Local.ymd(input.year, input.month as u32, input.day as u32) - Duration::days(1);
      stats_service
        .get_day(prev_day.year(), prev_day.month(), prev_day.day())
        .await
        .ok()
    } else {
      None
    };

    prefetch_users(context, &stats).await?;

    Ok(sort_and_map_stats(stats, &previous_stats))
  }
}

async fn prefetch_users(
  context: &Context<'_>,
  stats: &HashMap<Uuid, ServiceStat>,
) -> HubbitSchemaResult<()> {
  if context.look_ahead().field("user").exists()
    || context.look_ahead().field("stats").field("user").exists()
  {
    // Prefetch users to cache them if user field is queried
    let user_service = context.data_unchecked::<UserService>();
    let user_ids = stats.keys().copied().collect::<Vec<_>>();
    user_service
      .get_by_ids(user_ids.as_slice(), false)
      .await
      .map_err(|e| {
        error!("[Schema error] {:?}", e);
        HubbitSchemaError::InternalError
      })?;
  }

  Ok(())
}

fn sort_and_map_stats(
  stats: HashMap<Uuid, ServiceStat>,
  prev_stats: &Option<HashMap<Uuid, ServiceStat>>,
) -> Vec<Stat> {
  let prev_positions = if let Some(prev_stats) = prev_stats {
    let mut prev_stats = prev_stats
      .into_iter()
      .map(|(user_id, stat)| (*user_id, stat.duration_ms))
      .collect::<Vec<_>>();
    prev_stats.sort_by_key(|(_, dur)| -dur);
    prev_stats
      .into_iter()
      .enumerate()
      .map(|(index, (user_id, _))| (user_id, index as i32 + 1))
      .collect()
  } else {
    HashMap::new()
  };

  let mut stats = stats.into_iter().map(|(_, stat)| stat).collect::<Vec<_>>();
  stats.sort_by_key(|stat| -stat.duration_ms);
  stats
    .iter()
    .enumerate()
    .map(|(index, stat)| Stat {
      user: User { id: stat.user_id },
      duration_seconds: stat.duration_ms / 1000,
      current_position: index as i32 + 1,
      prev_position: prev_positions.get(&stat.user_id).map(|&v| v as i32),
    })
    .collect()
}
