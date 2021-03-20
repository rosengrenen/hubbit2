mod handlers;
mod models;
mod repositories;
mod schema;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use mobc::{Connection, Pool};
use mobc_redis::{redis::Client, RedisConnectionManager};
use schema::schema;
use sqlx::PgPool;
use std::env;

pub type RedisPool = Pool<RedisConnectionManager>;
pub type RedisConnection = Connection<RedisConnectionManager>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  let port = env::var("PORT").unwrap();
  let db_url = env::var("DATABASE_URL").unwrap();
  let redis_url = env::var("REDIS_URL").unwrap();

  let db_pool = PgPool::connect(&db_url).await.unwrap();

  let client = Client::open(redis_url).unwrap();
  let manager = RedisConnectionManager::new(client);
  let redis_pool = Pool::builder().build(manager);

  HttpServer::new(move || {
    App::new()
      .data(db_pool.clone())
      .data(redis_pool.clone())
      .data(schema())
      .configure(handlers::init)
  })
  .bind(format!("0.0.0.0:{}", port))?
  .run()
  .await
}
