use actix_web::web::ServiceConfig;

mod sessions;

pub fn init(cfg: &mut ServiceConfig) {
  sessions::init(cfg);
}
