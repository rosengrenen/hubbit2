use uuid::Uuid;

#[derive(Clone)]
pub enum UserEvent {
  Join(Uuid),
  Leave(Uuid),
}
