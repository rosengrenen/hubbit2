pub mod query;
mod utils;

use super::{user::User, Context};
use juniper::GraphQLObject;

#[derive(Debug, GraphQLObject)]
#[graphql(context = Context)]
pub struct Stat {
  user: User,
  score: i32,
  time: i32,
}
