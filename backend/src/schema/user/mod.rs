use crate::repositories::UserRepository;

use super::Context;
use juniper::graphql_object;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
  pub id: Uuid,
}

#[graphql_object(Context = Context)]
impl User {
  fn id(&self) -> Uuid {
    self.id
  }

  async fn cid(&self) -> String {
    let user_repository = UserRepository::new();
    let user = user_repository.get_by_id(self.id).await.unwrap();
    user.cid
  }

  async fn nick(&self) -> String {
    let user_repository = UserRepository::new();
    let user = user_repository.get_by_id(self.id).await.unwrap();
    user.nick
  }

  async fn first_name(&self) -> String {
    let user_repository = UserRepository::new();
    let user = user_repository.get_by_id(self.id).await.unwrap();
    user.first_name
  }

  async fn last_name(&self) -> String {
    let user_repository = UserRepository::new();
    let user = user_repository.get_by_id(self.id).await.unwrap();
    user.last_name
  }

  async fn avatar_url(&self) -> String {
    let user_repository = UserRepository::new();
    let user = user_repository.get_by_id(self.id).await.unwrap();
    user.avatar_url
  }
}
