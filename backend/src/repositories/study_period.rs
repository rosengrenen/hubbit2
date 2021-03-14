use sqlx::{Pool, Postgres};

pub struct StudyPeriodRepository<'a> {
  pool: &'a Pool<Postgres>,
}

impl<'a> StudyPeriodRepository<'a> {
  pub fn new(pool: &'a Pool<Postgres>) -> Self {
    Self { pool }
  }
}
