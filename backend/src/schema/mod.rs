mod stats;
mod user;

use actix_web::{cookie::Cookie, http::HeaderMap};
use juniper::{graphql_object, EmptySubscription, RootNode};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Context {
  pub pool: Arc<Pool<Postgres>>,
  pub headers: HeaderMap,
  pub cookies: Vec<Cookie<'static>>,
  // TODO:
  // session: Option<Session>
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
