use backend::schema::{HubbitSchema, MutationRoot, QueryRoot, SubscriptionRoot};

fn main() {
  let schema = HubbitSchema::build(
    QueryRoot::default(),
    MutationRoot::default(),
    SubscriptionRoot,
  )
  .finish();

  println!("{}", &schema.sdl());
}
