use crate::models::{MacAddress, Session};
use crate::DbPool;
use actix_web::{http::header::Header, web, HttpRequest, Responder};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use uuid::Uuid;

async fn add_sessions(
    req: HttpRequest,
    db_pool: web::Data<DbPool>,
    mac_addrs: web::Json<Vec<String>>,
) -> impl Responder {
    // Get Bearer token from header
    let auth = Authorization::<Bearer>::parse(&req).unwrap();
    let scheme = auth.into_scheme();
    let api_key = scheme.token();
    let api_key = api_key.clone().into_owned();

    // Check that the api_key is valid
    sqlx::query!(
        "
SELECT *
FROM api_keys
WHERE key = $1
        ",
        api_key
    )
    .fetch_one(db_pool.as_ref())
    .await
    .unwrap();

    // Get all registered mac addresses, basically filtering out
    // unregistered ones
    let mac_addrs: Vec<MacAddress> = sqlx::query_as!(
        MacAddress,
        "
SELECT *
FROM mac_addresses
WHERE mac = ANY($1)
        ",
        mac_addrs.as_slice()
    )
    .fetch_all(db_pool.as_ref())
    .await
    .unwrap();

    // User ids associated with the reported mac addresses
    let mut user_ids = mac_addrs
        .iter()
        .map(|mac_addr| mac_addr.user_id)
        .collect::<Vec<Uuid>>();
    user_ids.sort_unstable();
    user_ids.dedup();

    // Extend sessions that are active
    let active_sessions: Vec<Session> = sqlx::query_as!(
        Session,
        "
UPDATE sessions
SET end_time = NOW() + (5 * interval '1 minute')
WHERE user_id = ANY($1) AND end_time > NOW()
RETURNING *
        ",
        user_ids.as_slice()
    )
    .fetch_all(db_pool.as_ref())
    .await
    .unwrap();
    let active_user_ids = active_sessions
        .into_iter()
        .map(|Session { user_id, .. }| user_id)
        .collect::<Vec<Uuid>>();

    // Create new sessions for users that don't have an active session
    let inactive_user_ids = user_ids
        .into_iter()
        .filter(|user_id| !active_user_ids.contains(user_id))
        .collect::<Vec<Uuid>>();
    sqlx::query!(
        "
INSERT INTO sessions (user_id, start_time, end_time)
SELECT val, NOW(), NOW() + (5 * interval '1 minute')
FROM UNNEST($1::uuid[]) as val
            ",
        inactive_user_ids.as_slice(),
    )
    .fetch_all(db_pool.as_ref())
    .await
    .unwrap();
    "OK"
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/sessions").route(web::post().to(add_sessions)));
}
