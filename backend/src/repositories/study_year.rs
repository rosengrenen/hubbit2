use sqlx::{Pool, Postgres};

pub struct StudyYearRepository<'a> {
  pool: &'a Pool<Postgres>,
}

impl<'a> StudyYearRepository<'a> {
  pub fn new(pool: &'a Pool<Postgres>) -> Self {
    Self { pool }
  }
}
