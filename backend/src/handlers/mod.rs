mod auth;
mod graphql;
mod session;

use actix_web::{
  web::{self, ServiceConfig},
  HttpResponse,
};

async fn health() -> HttpResponse {
  HttpResponse::Ok().finish()
}

pub fn init(config: &mut ServiceConfig) {
  config.service(web::resource("/").route(web::get().to(health)));
  auth::init(config);
  graphql::init(config);
  session::init(config);
}
