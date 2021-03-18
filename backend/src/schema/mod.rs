use crate::repositories::{
  ApiKeyRepository, MacAddressRepository, SessionRepository, StudyPeriodRepository,
  StudyYearRepository, UserSessionRepository,
};
use actix_web::{cookie::Cookie, http::HeaderMap};
use actix_web_httpauth::headers::authorization::{Bearer, Scheme};
use chrono::{DateTime, Duration, TimeZone, Utc};
use juniper::{graphql_object, EmptySubscription, GraphQLObject, RootNode};
use sqlx::{Pool, Postgres};
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Context {
  pub pool: Arc<Pool<Postgres>>,
  pub headers: HeaderMap,
  pub cookies: Vec<Cookie<'static>>,
}

impl juniper::Context for Context {}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn schema() -> Schema {
  Schema::new(Query, Mutation, EmptySubscription::<Context>::new())
}

pub struct Query;

#[derive(Debug, GraphQLObject)]
struct User {
  id: Uuid,
}

#[derive(Debug, GraphQLObject)]
struct Stat {
  user: User,
  score: i32,
  time: i32,
}

fn calculate_stats(
  user_sessions: &HashMap<Uuid, Vec<(DateTime<Utc>, DateTime<Utc>)>>,
) -> Vec<Stat> {
  let mut stats = user_sessions
    .into_iter()
    .map(|(&user_id, sessions)| {
      let minutes = sessions.iter().fold(0, |prev, &(start_time, end_time)| {
        prev + (end_time - start_time).num_minutes() as i32
      });

      Stat {
        user: User { id: user_id },
        score: minutes,
        time: minutes,
      }
    })
    .collect::<Vec<_>>();
  stats.sort_by_key(|s| -s.score);
  stats
}

fn calculate_stats_for_range(
  user_sessions: &HashMap<Uuid, Vec<(DateTime<Utc>, DateTime<Utc>)>>,
  range_start_time: DateTime<Utc>,
  range_end_time: DateTime<Utc>,
) -> Vec<Stat> {
  let mut stats = user_sessions
    .into_iter()
    .map(|(&user_id, sessions)| {
      let minutes = sessions.iter().fold(0, |prev, &(start_time, end_time)| {
        // Don't count session time outside of the range
        let session_minutes = if end_time > range_end_time {
          (range_end_time - start_time).num_minutes() as i32
        } else if start_time < range_start_time {
          (end_time - range_start_time).num_minutes() as i32
        } else {
          (end_time - start_time).num_minutes() as i32
        };

        prev + session_minutes
      });

      Stat {
        user: User { id: user_id },
        score: minutes,
        time: minutes,
      }
    })
    .collect::<Vec<_>>();
  stats.sort_by_key(|s| -s.score);
  stats
}

#[graphql_object(context = Context)]
impl Query {
  async fn stats_alltime(context: &Context) -> Vec<Stat> {
    let user_session_repo = UserSessionRepository::new(&context.pool);

    let sessions = user_session_repo.get_all().await.unwrap();

    let mut user_sessions = HashMap::new();
    for sess in sessions {
      user_sessions
        .entry(sess.user_id)
        .or_insert(vec![])
        .push((sess.start_time, sess.end_time));
    }

    calculate_stats(&user_sessions)
  }

  async fn stats_year(year: i32, context: &Context) -> Vec<Stat> {
    let user_session_repo = UserSessionRepository::new(&context.pool);

    let start_time = Utc.ymd(year, 1, 1).and_hms(0, 0, 0);
    let end_time = Utc.ymd(year, 12, 31).and_hms(23, 59, 59);
    let sessions = user_session_repo
      .get_range(start_time, end_time)
      .await
      .unwrap();

    let mut user_sessions = HashMap::new();
    for sess in sessions {
      user_sessions
        .entry(sess.user_id)
        .or_insert(vec![])
        .push((sess.start_time, sess.end_time));
    }

    calculate_stats_for_range(&user_sessions, start_time, end_time)
  }

  async fn stats_month(year: i32, month: i32, context: &Context) -> Vec<Stat> {
    let user_session_repo = UserSessionRepository::new(&context.pool);

    let start_time = Utc.ymd(year, month as u32, 1).and_hms(0, 0, 0);
    let end_time = if month == 12 {
      Utc.ymd(year + 1, 1, 1).and_hms(23, 59, 59)
    } else {
      Utc.ymd(year, month as u32 + 1, 1).and_hms(23, 59, 59)
    } - Duration::seconds(1);
    let sessions = user_session_repo
      .get_range(start_time, end_time)
      .await
      .unwrap();

    let mut user_sessions = HashMap::new();
    for sess in sessions {
      user_sessions
        .entry(sess.user_id)
        .or_insert(vec![])
        .push((sess.start_time, sess.end_time));
    }

    calculate_stats_for_range(&user_sessions, start_time, end_time)
  }

  async fn stats_day(year: i32, month: i32, day: i32, context: &Context) -> Vec<Stat> {
    let user_session_repo = UserSessionRepository::new(&context.pool);

    let start_time = Utc.ymd(year, month as u32, day as u32).and_hms(0, 0, 0);
    let end_time = Utc.ymd(year, month as u32, day as u32).and_hms(23, 59, 59);
    let sessions = user_session_repo
      .get_range(start_time, end_time)
      .await
      .unwrap();

    let mut user_sessions = HashMap::new();
    for sess in sessions {
      user_sessions
        .entry(sess.user_id)
        .or_insert(vec![])
        .push((sess.start_time, sess.end_time));
    }

    calculate_stats_for_range(&user_sessions, start_time, end_time)
  }

  async fn stats_study_year(year: i32, context: &Context) -> Vec<Stat> {
    let user_session_repo = UserSessionRepository::new(&context.pool);
    let study_year_repo = StudyYearRepository::new(&context.pool);

    let (start_time, end_time) = study_year_repo.get_by_year(year).await.unwrap();
    let sessions = user_session_repo
      .get_range(start_time, end_time)
      .await
      .unwrap();

    let mut user_sessions = HashMap::new();
    for sess in sessions {
      user_sessions
        .entry(sess.user_id)
        .or_insert(vec![])
        .push((sess.start_time, sess.end_time));
    }

    calculate_stats_for_range(&user_sessions, start_time, end_time)
  }

  async fn stats_study_period(year: i32, period: i32, context: &Context) -> Vec<Stat> {
    let user_session_repo = UserSessionRepository::new(&context.pool);
    let study_period_repo = StudyPeriodRepository::new(&context.pool);

    let (start_time, end_time) = study_period_repo
      .get_by_year_and_period(year, period)
      .await
      .unwrap();
    let sessions = user_session_repo
      .get_range(start_time, end_time)
      .await
      .unwrap();

    let mut user_sessions = HashMap::new();
    for sess in sessions {
      user_sessions
        .entry(sess.user_id)
        .or_insert(vec![])
        .push((sess.start_time, sess.end_time));
    }

    calculate_stats_for_range(&user_sessions, start_time, end_time)
  }
}

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
  async fn update_sessions(mut mac_addrs: Vec<String>, context: &Context) -> bool {
    let api_key_repo = ApiKeyRepository::new(&context.pool);
    let mac_addr_repo = MacAddressRepository::new(&context.pool);
    let session_repo = SessionRepository::new(&context.pool);
    let user_session_repo = UserSessionRepository::new(&context.pool);

    mac_addrs.sort_unstable();
    mac_addrs.dedup();

    let bearer = Bearer::parse(
      context
        .headers
        .get("Authorization")
        .expect("couldnt get auth header"),
    )
    .expect("couldnt parse bearer");
    api_key_repo
      .get_by_key(bearer.token())
      .await
      .expect("could not find api key");

    let mac_addrs = mac_addr_repo.get_by_addrs(&mac_addrs).await.unwrap();

    let mut user_ids = mac_addrs
      .iter()
      .map(|mac_addr| mac_addr.user_id)
      .collect::<Vec<_>>();
    user_ids.sort_unstable();
    user_ids.dedup();
    user_session_repo.update_sessions(&user_ids).await.unwrap();

    let devices = mac_addrs
      .into_iter()
      .map(|mac_addr| (mac_addr.user_id, mac_addr.address))
      .collect::<Vec<_>>();
    session_repo.update_sessions(&devices).await.unwrap();

    true
  }
}
