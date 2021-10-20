use actix_web::{
  web::{self, ServiceConfig},
  HttpResponse,
};
use actix_web_httpauth::headers::authorization::{Bearer, Scheme};
use log::warn;
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
  error::HubbitResult,
  repositories::{
    api_key::ApiKeyRepository, device::DeviceRepository, session::SessionRepository,
    user_session::UserSessionRepository,
  },
};

#[derive(Deserialize)]
struct SessionRequest {
  macs: Vec<(String, u32)>,
}

async fn update_sessions(
  session_req: web::Json<SessionRequest>,
  http_req: web::HttpRequest,
  pool: web::Data<PgPool>,
) -> HubbitResult<HttpResponse> {
  let pool = PgPool::clone(&pool);
  let api_key_repo = ApiKeyRepository::new(pool.clone());
  let device_repo = DeviceRepository::new(pool.clone());
  let session_repo = SessionRepository::new(pool.clone());
  let user_session_repo = UserSessionRepository::new(pool);

  let mut mac_addrs: Vec<String> = session_req
    .into_inner()
    .macs
    .into_iter()
    .map(|(mac, _)| mac.to_uppercase())
    .collect();
  mac_addrs.sort_unstable();
  mac_addrs.dedup();

  let auth_header = match http_req.headers().get("Authorization") {
    Some(auth_header) => auth_header,
    _ => {
      return {
        warn!("[Update sessions] Missing authorization header");
        Ok(HttpResponse::Unauthorized().finish())
      }
    }
  };

  let bearer = match Bearer::parse(auth_header) {
    Ok(bearer) => bearer,
    _ => {
      warn!("[Update sessions] Invalid bearer token");
      return Ok(HttpResponse::Unauthorized().finish());
    }
  };
  if api_key_repo.get_by_key(bearer.token()).await.is_err() {
    warn!("[Update sessions] Invalid api key");
    return Ok(HttpResponse::Unauthorized().finish());
  };

  let devices = device_repo.get_by_addrs(&mac_addrs).await?;

  let mut user_ids = devices
    .iter()
    .map(|device| device.user_id)
    .collect::<Vec<_>>();
  user_ids.sort_unstable();
  user_ids.dedup();
  user_session_repo
    .update_sessions(&user_ids)
    .await
    .map_err(|e| {
      warn!("[Update sessions] Could not update user sessions");
      e
    })?;

  let devices = devices
    .into_iter()
    .map(|device| (device.user_id, device.address))
    .collect::<Vec<_>>();
  session_repo.update_sessions(&devices).await.map_err(|e| {
    warn!("[Update sessions] Could not update sessions");
    e
  })?;

  Ok(HttpResponse::Ok().finish())
}

pub fn init(config: &mut ServiceConfig) {
  config.service(
    web::resource("/sessions")
      .route(web::post().to(update_sessions))
      .route(web::put().to(update_sessions)),
  );
}
