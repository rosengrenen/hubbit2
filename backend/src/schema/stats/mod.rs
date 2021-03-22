pub mod query;

use super::{user::User, Context};
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, GraphQLObject, Serialize)]
#[graphql(context = Context)]
pub struct Stat {
  pub user: User,
  pub score: i32,
  pub time: i32,
}
