use super::{
  utils::{calculate_stats, day_start_end, month_start_end, year_start_end},
  Stat,
};
use crate::{repositories::Period as DbPeriod, schema::Context};
use chrono::{DateTime, Local, TimeZone};
use juniper::{GraphQLEnum, GraphQLInputObject};
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(GraphQLInputObject)]
pub struct StatsInput {
  year: Option<YearStatsInput>,
  month: Option<MonthStatsInput>,
  day: Option<DayStatsInput>,
  study_year: Option<StudyYearStatsInput>,
  study_period: Option<StudyPeriodStatsInput>,
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
enum Period {
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
  let (start_time, end_time) = if let Some(input) = input {
    if let Some(YearStatsInput { year }) = input.year {
      year_start_end(year)
    } else if let Some(MonthStatsInput { year, month }) = input.month {
      month_start_end(year, month as u32)
    } else if let Some(DayStatsInput { year, month, day }) = input.day {
      day_start_end(year, month as u32, day as u32)
    } else if let Some(StudyYearStatsInput { year }) = input.study_year {
      context.repos.study_year.get_by_year(year).await.unwrap()
    } else if let Some(StudyPeriodStatsInput { year, period }) = input.study_period {
      context
        .repos
        .study_period
        .get_by_year_and_period(year, period.into())
        .await
        .unwrap()
    } else {
      (
        MIN_DATETIME.with_timezone(&Local),
        MAX_DATETIME.with_timezone(&Local),
      )
    }
  } else {
    (
      MIN_DATETIME.with_timezone(&Local),
      MAX_DATETIME.with_timezone(&Local),
    )
  };

  let sessions = context
    .repos
    .user_session
    .get_range(start_time, end_time)
    .await
    .unwrap();

  let mut user_sessions = HashMap::new();
  for sess in sessions {
    user_sessions
      .entry(sess.user_id)
      .or_insert_with(Vec::new)
      .push((
        sess.start_time.with_timezone(&Local),
        sess.end_time.with_timezone(&Local),
      ));
  }

  calculate_stats(&user_sessions, start_time, end_time)
}
