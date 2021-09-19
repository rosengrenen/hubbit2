mod device;
pub mod me;
pub mod session;
pub mod stats;
pub mod user;

use std::fmt::Display;

use async_graphql::{
  guard::Guard, Context, ErrorExtensions, MergedObject, Result, Schema, Subscription,
};
use async_trait::async_trait;
use futures::StreamExt;

use crate::{
  broker::SimpleBroker, event::UserEvent, models::GammaUser,
  repositories::user_session::UserSessionRepository,
};

use self::{
  device::DeviceMutation,
  me::MeQuery,
  session::{ActiveSession, SessionQuery},
  stats::StatsQuery,
  user::{User, UserQuery},
};

pub type HubbitSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

#[derive(MergedObject, Default)]
pub struct QueryRoot(SessionQuery, StatsQuery, MeQuery, UserQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(DeviceMutation);

#[derive(Default)]
pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
  async fn user_join(&self, context: &Context<'_>) -> impl futures::Stream<Item = ActiveSession> {
    let user_session_repo = context.data_unchecked::<UserSessionRepository>().clone();
    SimpleBroker::<UserEvent>::subscribe().filter_map(move |event| {
      let user_session_repo = user_session_repo.clone();
      async move {
        if let UserEvent::Join(user_id) = event {
          match user_session_repo.get_active().await {
            Ok(active_session) => active_session
              .into_iter()
              .find(|session| session.user_id == user_id)
              .map(|joined_session| ActiveSession {
                user: User { id: user_id },
                start_time: joined_session.start_time,
              }),
            _ => None,
          }
        } else {
          None
        }
      }
    })
  }

  async fn user_leave(&self) -> impl futures::Stream<Item = User> {
    SimpleBroker::<UserEvent>::subscribe().filter_map(|event| async move {
      if let UserEvent::Leave(user_id) = event {
        Some(User { id: user_id })
      } else {
        None
      }
    })
  }
}

pub type HubbitSchemaResult<T> = Result<T, HubbitSchemaError>;

#[derive(Clone, Copy, Debug)]
pub enum HubbitSchemaError {
  NotLoggedIn,
  NotAuthorized,
  InternalError,
  InvalidInput,
  NotFound,
}

impl ErrorExtensions for HubbitSchemaError {
  fn extend(&self) -> async_graphql::Error {
    self.extend_with(|err, e| match err {
      HubbitSchemaError::NotLoggedIn => e.set("code", "NOT_LOGGED_IN"),
      HubbitSchemaError::NotAuthorized => e.set("code", "NOT_AUTHORIZED"),
      HubbitSchemaError::InvalidInput => e.set("code", "INVALID_INPUT"),
      HubbitSchemaError::NotFound => e.set("code", "NOT_FOUND"),
      _ => (),
    })
  }
}

impl Display for HubbitSchemaError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      HubbitSchemaError::NotLoggedIn => write!(f, "Not logged in"),
      HubbitSchemaError::NotAuthorized => write!(f, "Not authorized"),
      HubbitSchemaError::InternalError => write!(f, "Internal unrecoverable error"),
      HubbitSchemaError::InvalidInput => write!(f, "Invalid input"),
      HubbitSchemaError::NotFound => write!(f, "Not found"),
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
      Err(HubbitSchemaError::NotLoggedIn.extend())
    }
  }
}
