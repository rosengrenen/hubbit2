use async_graphql::{guard::Guard, Context, Object};

use crate::{
  repositories::UserSessionRepository,
  schema::{user::User, AuthGuard},
};

#[derive(Default)]
pub struct SessionQuery;

#[Object]
impl SessionQuery {
  #[graphql(guard(AuthGuard()))]
  pub async fn current_sessions(&self, context: &Context<'_>) -> Vec<User> {
    let user_session_repo = context.data_unchecked::<UserSessionRepository>();
    let active_sessions = user_session_repo.get_active().await.unwrap();
    active_sessions
      .iter()
      .map(|session| User {
        id: session.user_id,
      })
      .collect()
  }
}
