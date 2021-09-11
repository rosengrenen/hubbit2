pub mod session;
pub mod stats;
pub mod user;

use std::fmt::Display;

use async_graphql::{
  guard::Guard, Context, EmptyMutation, EmptySubscription, ErrorExtensions, MergedObject, Result,
  Schema,
};
use async_trait::async_trait;

use crate::models::GammaUser;

use self::{session::query::SessionQuery, stats::query::StatsQuery};

pub type HubbitSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct QueryRoot(SessionQuery, StatsQuery);

#[derive(Clone, Copy, Debug)]
pub enum CustomError {
  NotLoggedIn,
}

impl ErrorExtensions for CustomError {
  fn extend(&self) -> async_graphql::Error {
    self.extend_with(|err, e| match err {
      CustomError::NotLoggedIn => e.set("code", "NOT_LOGGED_IN"),
    })
  }
}

impl Display for CustomError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      CustomError::NotLoggedIn => write!(f, "Not logged in"),
    }
  }
}

pub struct AuthGuard;

#[async_trait]
impl Guard for AuthGuard {
  async fn check(&self, context: &Context<'_>) -> Result<()> {
    if context.data_opt::<GammaUser>().is_some() {
      Ok(())
    } else {
      Err(CustomError::NotLoggedIn.extend())
    }
  }
}
