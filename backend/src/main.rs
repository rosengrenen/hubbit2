mod broker;
mod config;
mod event;
mod handlers;
mod models;
mod repositories;
mod schema;
mod services;
mod utils;

use std::collections::HashSet;

use actix_session::CookieSession;
use actix_web::{middleware, web, App, HttpServer};
use async_graphql::EmptyMutation;
use broker::SimpleBroker;
use dotenv::dotenv;
use event::UserEvent;
use mobc::{Connection, Pool};
use mobc_redis::{redis::Client, RedisConnectionManager};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
  config::Config,
  repositories::{
    StudyPeriodRepository, StudyYearRepository, UserRepository, UserSessionRepository,
  },
  schema::{HubbitSchema, QueryRoot, SubscriptionRoot},
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

  let schema = HubbitSchema::build(QueryRoot::default(), EmptyMutation, SubscriptionRoot)
    .data(stats_service)
    .data(hour_stats_service)
    .data(user_service)
    .data(user_session_repo.clone())
    .finish();

  tokio::spawn(async move { track_sessions(user_session_repo).await });

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

async fn track_sessions(user_session_repo: UserSessionRepository) -> anyhow::Result<()> {
  let mut present_users: HashSet<_> = loop {
    match get_active_users(&user_session_repo).await {
      Ok(present_users) => break present_users,
      _ => tokio::time::delay_for(std::time::Duration::from_secs(5)).await,
    }
  };

  loop {
    tokio::time::delay_for(std::time::Duration::from_secs(5)).await;
    if let Ok(new_present_users) = get_active_users(&user_session_repo).await {
      let mut new_users = Vec::new();
      let mut absent_users = Vec::new();
      for present_user in present_users.iter() {
        if !new_present_users.contains(present_user) {
          absent_users.push(*present_user);
        }
      }

      for new_present_user in new_present_users {
        if !present_users.contains(&new_present_user) {
          new_users.push(new_present_user);
        }
      }

      for new_user in new_users {
        present_users.insert(new_user);
        SimpleBroker::publish(UserEvent::Join(new_user));
      }

      for absent_user in absent_users {
        present_users.remove(&absent_user);
        SimpleBroker::publish(UserEvent::Leave(absent_user));
      }
    }
  }
}

async fn get_active_users(
  user_session_repo: &UserSessionRepository,
) -> anyhow::Result<HashSet<Uuid>> {
  Ok(
    user_session_repo
      .get_active()
      .await?
      .into_iter()
      .map(|session| session.user_id)
      .collect(),
  )
}
