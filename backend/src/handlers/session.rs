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
    ApiKeyRepository, MacAddressRepository, SessionRepository, UserSessionRepository,
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
  let mac_addr_repo = MacAddressRepository::new(pool.clone());
  let session_repo = SessionRepository::new(pool.clone());
  let user_session_repo = UserSessionRepository::new(pool);

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

  let mac_addrs = mac_addr_repo.get_by_addrs(&mac_addrs).await?;

  let mut user_ids = mac_addrs
    .iter()
    .map(|mac_addr| mac_addr.user_id)
    .collect::<Vec<_>>();
  user_ids.sort_unstable();
  user_ids.dedup();
  user_session_repo
    .update_sessions(&user_ids, config.session_lifetime_s)
    .await?;

  let devices = mac_addrs
    .into_iter()
    .map(|mac_addr| (mac_addr.user_id, mac_addr.address))
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
