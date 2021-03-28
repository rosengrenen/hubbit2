use crate::models::GammaUser;
use actix_web::{
  web::{self, ServiceConfig},
  HttpResponse,
};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct GammaTokenRes {
  access_token: String,
}

// TODO: move to a better place
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  exp: usize,
  pub sub: String,
}

async fn gamma(auth_code: web::Json<String>) -> HttpResponse {
  let client = Client::new();
  let res = client
    .post(
      Url::parse(&format!(
        "http://localhost:8081/api/oauth/token?grant_type=authorization_code&code={}",
        auth_code
      ))
      .unwrap(),
    )
    .basic_auth("hubbit", Some("hubbit"))
    .send()
    .await
    .unwrap()
    .text()
    .await
    .unwrap();
  let res: GammaTokenRes = serde_json::from_str(&res).unwrap();
  println!("{:?}", res);

  let res = client
    .get(Url::parse("http://localhost:8081/api/users/me").unwrap())
    .bearer_auth(res.access_token)
    .send()
    .await
    .unwrap()
    .text()
    .await
    .unwrap();
  let res: GammaUser = serde_json::from_str(&res).unwrap();
  println!("{:?}", res);

  let a = Utc::now() + chrono::Duration::days(1);
  let claims = Claims {
    exp: a.timestamp() as usize,
    sub: res.id.to_string(),
  };
  let token = encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret("hubbit".as_ref()),
  )
  .unwrap();

  HttpResponse::Ok().body(token)
}

pub fn init(config: &mut ServiceConfig) {
  config.service(web::resource("/auth/gamma").route(web::post().to(gamma)));
}
