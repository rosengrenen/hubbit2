mod handlers;
mod models;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use handlers::config;
use sqlx::{PgPool, Pool, Postgres};
use std::env;

pub type DbPool = Pool<Postgres>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  let db_url = env::var("DATABASE_URL").unwrap();
  let pool = PgPool::connect(&db_url).await.unwrap();

  HttpServer::new(move || App::new().data(pool.clone()).configure(config))
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
