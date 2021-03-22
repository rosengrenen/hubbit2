use super::Context;
use juniper::graphql_object;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
  pub id: Uuid,
}

#[graphql_object(Context = Context)]
impl User {
  fn id(&self) -> Uuid {
    self.id
  }

  async fn nick(&self, context: &Context) -> String {
    let user = context
      .services
      .user
      .get_by_id(self.id, false)
      .await
      .unwrap();
    user.nick
  }

  async fn first_name(&self, context: &Context) -> String {
    let user = context
      .services
      .user
      .get_by_id(self.id, false)
      .await
      .unwrap();
    user.first_name
  }

  async fn last_name(&self, context: &Context) -> String {
    let user = context
      .services
      .user
      .get_by_id(self.id, false)
      .await
      .unwrap();
    user.last_name
  }

  async fn avatar_url(&self, context: &Context) -> String {
    let user = context
      .services
      .user
      .get_by_id(self.id, false)
      .await
      .unwrap();
    user.avatar_url
  }
}
