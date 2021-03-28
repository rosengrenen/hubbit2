pub mod stats;
pub mod user;

use crate::{
  repositories::{
    ApiKeyRepository, MacAddressRepository, SessionRepository, StudyPeriodRepository,
    StudyYearRepository, UserRepository, UserSessionRepository,
  },
  services::{stats::StatsService, user::UserService},
  RedisPool,
};
use actix_web::{cookie::Cookie, http::HeaderMap};
use juniper::{graphql_object, EmptySubscription, RootNode};
use uuid::Uuid;

#[derive(Clone)]
pub struct ContextRepositories {
  pub api_key: ApiKeyRepository,
  pub mac_addr: MacAddressRepository,
  pub session: SessionRepository,
  pub study_period: StudyPeriodRepository,
  pub study_year: StudyYearRepository,
  pub user_session: UserSessionRepository,
  pub user: UserRepository,
}

#[derive(Clone)]
pub struct ContextServices {
  pub stats: StatsService,
  pub user: UserService,
}

#[derive(Clone)]
pub struct Context {
  pub repos: ContextRepositories,
  pub services: ContextServices,
  pub headers: HeaderMap,
  pub cookies: Vec<Cookie<'static>>,
  pub redis_pool: RedisPool,
  pub user_id: Option<Uuid>,
}

impl juniper::Context for Context {}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn schema() -> Schema {
  Schema::new(Query, Mutation, EmptySubscription::<Context>::new())
}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
  async fn stats(
    input: Option<stats::query::StatsInput>,
    context: &Context,
  ) -> stats::query::StatsPayload {
    stats::query::stats(input, context).await
  }
}

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
  fn _empty() -> bool {
    false
  }
}
