mod config;
mod handlers;
mod models;
mod repositories;
mod schema;
mod services;
mod utils;

use actix_session::CookieSession;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use mobc::{Connection, Pool};
use mobc_redis::{redis::Client, RedisConnectionManager};
use schema::schema;
use sqlx::PgPool;

use crate::config::Config;

pub type RedisPool = Pool<RedisConnectionManager>;
pub type RedisConnection = Connection<RedisConnectionManager>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  env_logger::init();

  let config = Config::from_env();

  let db_pool = PgPool::connect(&config.db_url).await.unwrap();

  let client = Client::open(config.redis_url.clone()).unwrap();
  let manager = RedisConnectionManager::new(client);
  let redis_pool = Pool::builder().build(manager);

  let config_clone = config.clone();
  HttpServer::new(move || {
    App::new()
      .wrap(middleware::Logger::default())
      .wrap(CookieSession::signed(&[0; 32]).secure(false))
      .data(config_clone.clone())
      .data(db_pool.clone())
      .data(redis_pool.clone())
      .data(schema())
      .service(web::scope("/api").configure(handlers::init))
  })
  .bind(format!("0.0.0.0:{}", config.port))?
  .run()
  .await
}
