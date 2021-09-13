use async_graphql::EmptyMutation;
use backend::schema::{HubbitSchema, QueryRoot, SubscriptionRoot};

fn main() {
  let schema = HubbitSchema::build(QueryRoot::default(), EmptyMutation, SubscriptionRoot).finish();

  println!("{}", &schema.sdl());
}
