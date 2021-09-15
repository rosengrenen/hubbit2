use actix_web::{
  web::{self, ServiceConfig},
  HttpResponse,
};
use actix_web_httpauth::headers::authorization::{Bearer, Scheme};
use sqlx::PgPool;

use crate::{
  config::Config,
  error::HubbitResult,
  repositories::{
    api_key::ApiKeyRepository, device::DeviceRepository, session::SessionRepository,
    user_session::UserSessionRepository,
  },
};

async fn update_sessions(
  mut mac_addrs: web::Json<Vec<String>>,
  req: web::HttpRequest,
  pool: web::Data<PgPool>,
  config: web::Data<Config>,
) -> HubbitResult<HttpResponse> {
  let pool = PgPool::clone(&pool);
  let api_key_repo = ApiKeyRepository::new(pool.clone());
  let device_repo = DeviceRepository::new(pool.clone());
  let session_repo = SessionRepository::new(pool.clone());
  let user_session_repo = UserSessionRepository::new(pool);

  for mac_addr in mac_addrs.iter_mut() {
    *mac_addr = mac_addr.to_uppercase();
  }
  mac_addrs.sort_unstable();
  mac_addrs.dedup();

  let auth_header = match req.headers().get("Authorization") {
    Some(auth_header) => auth_header,
    _ => return Ok(HttpResponse::Unauthorized().finish()),
  };

  let bearer = match Bearer::parse(auth_header) {
    Ok(bearer) => bearer,
    _ => return Ok(HttpResponse::Unauthorized().finish()),
  };
  api_key_repo.get_by_key(bearer.token()).await?;

  let devices = device_repo.get_by_addrs(&mac_addrs).await?;

  let mut user_ids = devices
    .iter()
    .map(|device| device.user_id)
    .collect::<Vec<_>>();
  user_ids.sort_unstable();
  user_ids.dedup();
  user_session_repo
    .update_sessions(&user_ids, config.session_lifetime_s)
    .await?;

  let devices = devices
    .into_iter()
    .map(|device| (device.user_id, device.address))
    .collect::<Vec<_>>();
  session_repo
    .update_sessions(&devices, config.session_lifetime_s)
    .await?;

  Ok(HttpResponse::Ok().finish())
}

pub fn init(config: &mut ServiceConfig) {
  config.service(
    web::resource("/sessions")
      .route(web::post().to(update_sessions))
      .route(web::put().to(update_sessions)),
  );
}
