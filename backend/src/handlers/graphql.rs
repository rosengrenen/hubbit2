use actix_session::Session;
use actix_web::{
  web::{self, ServiceConfig},
  Error, HttpResponse,
};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};

use crate::{config::Config, schema::HubbitSchema};

async fn playground() -> Result<HttpResponse, Error> {
  Ok(
    HttpResponse::Ok()
      .content_type("text/html; charset=utf-8")
      .body(playground_source(
        GraphQLPlaygroundConfig::new("/api/graphql").subscription_endpoint("/api/graphql"),
      )),
  )
}

async fn graphql(
  session: Session,
  gql_request: Request,
  schema: web::Data<HubbitSchema>,
  config: web::Data<Config>,
) -> Response {
  let mut request = gql_request.into_inner();
  if let Ok(Some(access_token)) = session.get::<String>("gamma_access_token") {
    if let Ok(user) = crate::utils::gamma::get_current_user(&config, &access_token).await {
      request = request.data(user);
    }
  };
  schema.execute(request).await.into()
}

pub fn init(config: &mut ServiceConfig) {
  config.service(
    web::resource("/graphql")
      .route(web::get().to(playground))
      .route(web::post().to(graphql)),
  );
}
