use async_graphql::{guard::Guard, Context, Object};

use crate::models::GammaUser;

use super::{user::User, AuthGuard};

#[derive(Default)]
pub struct MeQuery;

#[Object]
impl MeQuery {
  #[graphql(guard(AuthGuard()))]
  pub async fn me(&self, context: &Context<'_>) -> User {
    let user = context.data_unchecked::<GammaUser>();
    User { id: user.id }
  }
}
