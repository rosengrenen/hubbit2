use actix_web::{
  web::{self, ServiceConfig},
  HttpResponse,
};
use actix_web_httpauth::headers::authorization::{Bearer, Scheme};
use sqlx::{Pool, Postgres};

use crate::repositories::{
  ApiKeyRepository, MacAddressRepository, SessionRepository, UserSessionRepository,
};

async fn update_sessions(
  mut mac_addrs: web::Json<Vec<String>>,
  req: web::HttpRequest,
  pool: web::Data<Pool<Postgres>>,
) -> HttpResponse {
  let api_key_repo = ApiKeyRepository::new(&pool);
  let mac_addr_repo = MacAddressRepository::new(&pool);
  let session_repo = SessionRepository::new(&pool);
  let user_session_repo = UserSessionRepository::new(&pool);

  mac_addrs.sort_unstable();
  mac_addrs.dedup();

  let bearer = Bearer::parse(
    req
      .headers()
      .get("Authorization")
      .expect("couldnt get auth header"),
  )
  .expect("couldnt parse bearer");
  api_key_repo
    .get_by_key(bearer.token())
    .await
    .expect("could not find api key");

  let mac_addrs = mac_addr_repo.get_by_addrs(&mac_addrs).await.unwrap();

  let mut user_ids = mac_addrs
    .iter()
    .map(|mac_addr| mac_addr.user_id)
    .collect::<Vec<_>>();
  user_ids.sort_unstable();
  user_ids.dedup();
  user_session_repo.update_sessions(&user_ids).await.unwrap();

  let devices = mac_addrs
    .into_iter()
    .map(|mac_addr| (mac_addr.user_id, mac_addr.address))
    .collect::<Vec<_>>();
  session_repo.update_sessions(&devices).await.unwrap();

  HttpResponse::Ok().finish()
}

pub fn init(config: &mut ServiceConfig) {
  config.service(web::resource("/sessions").route(web::get().to(update_sessions)));
}
