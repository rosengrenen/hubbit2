pub mod query;

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct Stat {
  pub user: User,
  pub score: i32,
  pub time: i32,
}
