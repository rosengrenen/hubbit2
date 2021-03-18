use actix_web::{
  web::{self, ServiceConfig},
  Error, HttpMessage, HttpRequest, HttpResponse,
};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};
use sqlx::{Pool, Postgres};

use crate::schema::{Context, Schema};

async fn playground() -> Result<HttpResponse, Error> {
  playground_handler("/graphql", None).await
}

async fn graphiql() -> Result<HttpResponse, Error> {
  graphiql_handler("/graphql", None).await
}

async fn graphql(
  req: HttpRequest,
  payload: web::Payload,
  schema: web::Data<Schema>,
  pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, Error> {
  let context = Context {
    pool: pool.into_inner(),
    headers: req.headers().clone(),
    cookies: req.cookies().map_or(vec![], |v| v.clone()),
  };
  graphql_handler(&schema, &context, req, payload).await
}

pub fn init(config: &mut ServiceConfig) {
  config
    .service(
      web::resource("/graphql")
        .route(web::get().to(playground))
        .route(web::post().to(graphql)),
    )
    .service(web::resource("/graphiql").route(web::get().to(graphiql)));
}
