use std::collections::HashSet;

use actix_session::CookieSession;
use actix_web::{middleware, web, App, HttpServer};
use chrono::{Datelike, Duration, Local};
use dotenv::dotenv;
use log::{error, info, warn};
use mobc::Pool;
use mobc_redis::{redis::Client, RedisConnectionManager};
use sqlx::PgPool;
use uuid::Uuid;

use backend::{
  broker::SimpleBroker,
  config::Config,
  error::HubbitResult,
  event::UserEvent,
  handlers,
  repositories::{
    device::DeviceRepository, session::SessionRepository, study_period::StudyPeriodRepository,
    study_year::StudyYearRepository, user::UserRepository, user_session::UserSessionRepository,
  },
  schema::{HubbitSchema, MutationRoot, QueryRoot, SubscriptionRoot},
  services::{hour_stats::HourStatsService, stats::StatsService, user::UserService},
};

#[actix_web::main]
async fn main() -> HubbitResult<()> {
  dotenv().ok();
  env_logger::init();

  let config = Config::from_env()?;

  let db_pool = PgPool::connect(&config.db_url).await?;

  let redis_client = Client::open(config.redis_url.clone())?;
  let redis_manager = RedisConnectionManager::new(redis_client);
  let redis_pool = Pool::builder().build(redis_manager);

  // Create repos
  let device_repo = DeviceRepository::new(db_pool.clone());
  let session_repo = SessionRepository::new(db_pool.clone());
  let study_period_repo = StudyPeriodRepository::new(db_pool.clone());
  let study_year_repo = StudyYearRepository::new(db_pool.clone());
  let user_repo = UserRepository::new(config.clone());
  let user_session_repo = UserSessionRepository::new(db_pool.clone());

  // Create services
  let stats_service = StatsService::new(
    user_session_repo.clone(),
    study_year_repo.clone(),
    study_period_repo.clone(),
    redis_pool.clone(),
  );
  let hour_stats_service = HourStatsService::new(user_session_repo.clone());
  let user_service = UserService::new(user_repo, redis_pool.clone());

  let schema = HubbitSchema::build(
    QueryRoot::default(),
    MutationRoot::default(),
    SubscriptionRoot,
  )
  .data(device_repo)
  .data(stats_service.clone())
  .data(hour_stats_service)
  .data(session_repo)
  .data(study_period_repo)
  .data(study_year_repo)
  .data(user_service.clone())
  .data(user_session_repo.clone())
  .finish();

  tokio::spawn(async move { track_sessions(user_session_repo).await });
  tokio::spawn(async move {
    init_cache(stats_service, user_service)
      .await
      .map_err(|e| warn!("Failed to init cache: {:?}", e))
  });

  let config_clone = config.clone();
  let port = config.port.clone();
  let cookie_secret = config.cookie_secret.clone();
  let cookie_secure = config.cookie_secure;
  if cookie_secret.as_bytes().len() != 32 {
    panic!("Cookie secret must be exactly 32 bytes");
  }

  Ok(
    HttpServer::new(move || {
      App::new()
        .wrap(middleware::Logger::default())
        .wrap(CookieSession::private(cookie_secret.as_bytes()).secure(cookie_secure))
        .data(config_clone.clone())
        .data(db_pool.clone())
        .data(redis_pool.clone())
        .data(schema.clone())
        .service(web::scope("/api").configure(handlers::init))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await?,
  )
}

async fn track_sessions(user_session_repo: UserSessionRepository) -> HubbitResult<()> {
  let mut present_users: HashSet<_> = loop {
    match get_active_users(&user_session_repo).await {
      Ok(present_users) => break present_users,
      _ => {
        warn!("[Session tracker] Could not get initial active users, retrying in 5 seconds...");
        tokio::time::delay_for(std::time::Duration::from_secs(5)).await
      }
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
    } else {
      error!("[Session tracker] Could not get active users");
    }
  }
}

async fn get_active_users(
  user_session_repo: &UserSessionRepository,
) -> HubbitResult<HashSet<Uuid>> {
  Ok(
    user_session_repo
      .get_active()
      .await?
      .into_iter()
      .map(|session| session.user_id)
      .collect(),
  )
}

async fn init_cache(stats_service: StatsService, user_service: UserService) -> HubbitResult<()> {
  let earliest_date = stats_service.get_earliest_date().await?;
  let now = Local::now().naive_local().date();

  // Check days
  let mut date = earliest_date;
  loop {
    if date >= now {
      break;
    }

    stats_service
      .get_day(date.year(), date.month(), date.day())
      .await?;

    date += Duration::days(1);
  }
  info!("[Init cache] checked days");

  // Check months
  let mut year = earliest_date.year();
  let mut month = earliest_date.month();
  loop {
    if year >= now.year() && month >= now.month() {
      break;
    }

    stats_service.get_month(year, month).await?;

    if month == 12 {
      month = 1;
      year += 1;
    } else {
      month += 1;
    }
  }
  info!("[Init cache] checked months");

  // Get alltime, since it caches full years
  let alltime_stats = stats_service.get_alltime().await?;
  info!("[Init cache] checked alltime");

  // Check users
  for user_id in alltime_stats.keys() {
    user_service.get_by_id(*user_id, false).await?;
  }
  info!("[Init cache] checked users",);

  Ok(())
}
