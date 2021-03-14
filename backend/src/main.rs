mod handlers;
mod models;
mod repositories;
mod schema;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use schema::schema;
use sqlx::PgPool;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  let port = env::var("PORT").unwrap();

  let db_url = env::var("DATABASE_URL").unwrap();
  let pool = PgPool::connect(&db_url).await.unwrap();

  HttpServer::new(move || {
    App::new()
      .data(pool.clone())
      .data(schema())
      .configure(handlers::init)
  })
  .bind(format!("0.0.0.0:{}", port))?
  .run()
  .await
}
