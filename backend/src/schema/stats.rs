use std::collections::HashMap;

use async_graphql::{guard::Guard, Context, InputObject, Object, SimpleObject};
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

    Ok(sort_and_map_stats(stats))
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

    prefetch_users(context, &stats).await?;

    let stats = sort_and_map_stats(stats);
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

    prefetch_users(context, &stats).await?;

    let stats = sort_and_map_stats(stats);
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

    prefetch_users(context, &stats).await?;

    Ok(sort_and_map_stats(stats))
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

    prefetch_users(context, &stats).await?;

    Ok(sort_and_map_stats(stats))
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

    prefetch_users(context, &stats).await?;

    Ok(sort_and_map_stats(stats))
  }
}

async fn prefetch_users(
  context: &Context<'_>,
  stats: &HashMap<Uuid, ServiceStat>,
) -> HubbitSchemaResult<()> {
  if context.look_ahead().field("user").exists() {
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

fn sort_and_map_stats(stats: HashMap<Uuid, ServiceStat>) -> Vec<Stat> {
  let mut stats = stats.into_iter().map(|(_, stat)| stat).collect::<Vec<_>>();
  stats.sort_by_key(|stat| -stat.duration_ms);
  stats
    .iter()
    .map(|stat| Stat {
      user: User { id: stat.user_id },
      duration_seconds: stat.duration_ms / 1000 / 60,
    })
    .collect()
}
