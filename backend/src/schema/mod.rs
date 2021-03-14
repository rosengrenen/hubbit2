use crate::repositories::{
  ApiKeyRepository, MacAddressRepository, SessionRepository, UserSessionRepository,
};
use actix_web::{cookie::Cookie, http::HeaderMap};
use actix_web_httpauth::headers::authorization::{Bearer, Scheme};
use juniper::{graphql_object, EmptySubscription, RootNode};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

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

#[graphql_object(context = Context)]
impl Query {
  fn api_version() -> String {
    "1.0".to_string()
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
      .map(|mac_addr| (mac_addr.user_id, mac_addr.mac))
      .collect::<Vec<_>>();
    session_repo.update_sessions(&devices).await.unwrap();

    true
  }
}
