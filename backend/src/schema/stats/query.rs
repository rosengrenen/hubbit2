use super::Stat;
use crate::{repositories::Period as DbPeriod, schema::Context};
use chrono::{DateTime, Local, TimeZone};
use juniper::{GraphQLEnum, GraphQLInputObject};
use lazy_static::lazy_static;

#[derive(GraphQLInputObject)]
pub struct StatsInput {
  year_stats: Option<YearStatsInput>,
  month_stats: Option<MonthStatsInput>,
  day_stats: Option<DayStatsInput>,
  study_year_stats: Option<StudyYearStatsInput>,
  study_period_stats: Option<StudyPeriodStatsInput>,
}

#[derive(GraphQLInputObject)]
struct YearStatsInput {
  year: i32,
}

#[derive(GraphQLInputObject)]
struct MonthStatsInput {
  year: i32,
  month: i32,
}

#[derive(GraphQLInputObject)]
struct DayStatsInput {
  year: i32,
  month: i32,
  day: i32,
}

#[derive(GraphQLInputObject)]
struct StudyYearStatsInput {
  year: i32,
}

#[derive(GraphQLEnum)]
pub enum Period {
  LP1,
  LP2,
  LP3,
  LP4,
  Summer,
}

impl From<Period> for DbPeriod {
  fn from(gql_period: Period) -> Self {
    match gql_period {
      Period::LP1 => Self::LP1,
      Period::LP2 => Self::LP2,
      Period::LP3 => Self::LP3,
      Period::LP4 => Self::LP4,
      Period::Summer => Self::Summer,
    }
  }
}

#[derive(GraphQLInputObject)]
struct StudyPeriodStatsInput {
  year: i32,
  period: Period,
}

pub type StatsPayload = Vec<Stat>;

lazy_static! {
  static ref MIN_DATETIME: DateTime<Local> = Local.ymd(2000, 1, 1).and_hms(0, 0, 0);
  static ref MAX_DATETIME: DateTime<Local> = Local.ymd(2099, 12, 31).and_hms(23, 59, 59);
}

pub async fn stats(input: Option<StatsInput>, context: &Context) -> StatsPayload {
  let stats_service = &context.services.stats;
  let stats = if let Some(input) = input {
    if let Some(YearStatsInput { year }) = input.year_stats {
      stats_service.get_year(year).await
    } else if let Some(MonthStatsInput { year, month }) = input.month_stats {
      stats_service.get_month(year, month as u32).await
    } else if let Some(DayStatsInput { year, month, day }) = input.day_stats {
      stats_service.get_day(year, month as u32, day as u32).await
    } else if let Some(StudyYearStatsInput { year }) = input.study_year_stats {
      stats_service.get_study_year(year).await
    } else if let Some(StudyPeriodStatsInput { year, period }) = input.study_period_stats {
      stats_service.get_study_period(year, period.into()).await
    } else {
      stats_service.get_lifetime().await
    }
  } else {
    stats_service.get_lifetime().await
  }
  .unwrap();

  // Prefect users to cache them, so that no individual looksup are needed
  // If some lookahead is possible in the query this could be disabled if users
  // aren't queried
  let user_ids = stats.keys().map(|id| id.clone()).collect::<Vec<_>>();
  context
    .services
    .user
    .get_by_ids(user_ids.as_slice(), false)
    .await
    .unwrap();

  let mut stats = stats.into_iter().map(|(_, stat)| stat).collect::<Vec<_>>();
  stats.sort_by_key(|stat| -stat.score);
  stats
}
