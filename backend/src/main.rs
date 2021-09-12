mod config;
mod handlers;
mod models;
mod repositories;
mod schema;
mod services;
mod utils;

use actix_session::CookieSession;
use actix_web::{middleware, web, App, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription};
use dotenv::dotenv;
use mobc::{Connection, Pool};
use mobc_redis::{redis::Client, RedisConnectionManager};
use sqlx::PgPool;

use crate::{
  config::Config,
  repositories::{
    StudyPeriodRepository, StudyYearRepository, UserRepository, UserSessionRepository,
  },
  schema::{HubbitSchema, QueryRoot},
  services::{hour_stats::HourStatsService, stats::StatsService, user::UserService},
};

pub type RedisPool = Pool<RedisConnectionManager>;
pub type RedisConnection = Connection<RedisConnectionManager>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  env_logger::init();

  let config = Config::from_env();

  let db_pool = PgPool::connect(&config.db_url).await.unwrap();

  let redis_client = Client::open(config.redis_url.clone()).unwrap();
  let redis_manager = RedisConnectionManager::new(redis_client);
  let redis_pool = Pool::builder().build(redis_manager);

  // Create repos
  let user_session_repo = UserSessionRepository::new(db_pool.clone());
  let study_year_repo = StudyYearRepository::new(db_pool.clone());
  let study_period_repo = StudyPeriodRepository::new(db_pool.clone());
  let user_repo = UserRepository::new(config.clone());

  // Create services
  let stats_service = StatsService::new(
    user_session_repo.clone(),
    study_year_repo,
    study_period_repo,
    redis_pool.clone(),
  );
  let hour_stats_service = HourStatsService::new(user_session_repo.clone());
  let user_service = UserService::new(user_repo, redis_pool.clone());

  let schema = HubbitSchema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
    .data(stats_service)
    .data(hour_stats_service)
    .data(user_service)
    .data(user_session_repo)
    .finish();

  let config_clone = config.clone();
  HttpServer::new(move || {
    App::new()
      .wrap(middleware::Logger::default())
      .wrap(CookieSession::signed(&[0; 32]).secure(false))
      .data(config_clone.clone())
      .data(db_pool.clone())
      .data(redis_pool.clone())
      .data(schema.clone())
      .service(web::scope("/api").configure(handlers::init))
  })
  .bind(format!("0.0.0.0:{}", config.port))?
  .run()
  .await
}
