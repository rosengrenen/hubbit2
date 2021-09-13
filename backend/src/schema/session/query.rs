use async_graphql::{guard::Guard, Context, Object, SimpleObject};
use chrono::{DateTime, Utc};

use crate::{
  repositories::UserSessionRepository,
  schema::{user::User, AuthGuard},
};

#[derive(Default)]
pub struct SessionQuery;

#[Object]
impl SessionQuery {
  #[graphql(guard(AuthGuard()))]
  pub async fn current_sessions(&self, context: &Context<'_>) -> Vec<ActiveSession> {
    let user_session_repo = context.data_unchecked::<UserSessionRepository>();
    let active_sessions = user_session_repo.get_active().await.unwrap();
    active_sessions
      .iter()
      .map(|session| ActiveSession {
        user: User {
          id: session.user_id,
        },
        start_time: session.start_time,
      })
      .collect()
  }
}

#[derive(SimpleObject)]
pub struct ActiveSession {
  pub user: User,
  pub start_time: DateTime<Utc>,
}
