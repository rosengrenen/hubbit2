use async_graphql::{Context, Object};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::services::user::UserService;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
  pub id: Uuid,
}

#[Object]
impl User {
  async fn id(&self) -> String {
    self.id.to_string()
  }

  async fn nick(&self, context: &Context<'_>) -> String {
    let user_service = context.data_unchecked::<UserService>();
    let user = user_service.get_by_id(self.id, false).await.unwrap();
    user.nick
  }

  async fn first_name(&self, context: &Context<'_>) -> String {
    let user_service = context.data_unchecked::<UserService>();
    let user = user_service.get_by_id(self.id, false).await.unwrap();
    user.first_name
  }

  async fn last_name(&self, context: &Context<'_>) -> String {
    let user_service = context.data_unchecked::<UserService>();
    let user = user_service.get_by_id(self.id, false).await.unwrap();
    user.last_name
  }

  async fn avatar_url(&self, context: &Context<'_>) -> String {
    let user_service = context.data_unchecked::<UserService>();
    let user = user_service.get_by_id(self.id, false).await.unwrap();
    user.avatar_url
  }
}
