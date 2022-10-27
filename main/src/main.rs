mod api;
mod db;
mod error_handle;
mod models;
mod repos;
mod routes;
mod schema;
mod services;
// mod tests;

#[macro_use]
extern crate diesel;
use crate::routes::{get_protected_routes, get_public_routes};
use actix_web::{App, HttpServer};
use std::io::Result as IOResult;

// TODO use dependency injection instead of repos direct import
// implement a struct for database and http client
// fix the tests after authentication implementation

#[actix_web::main]
async fn main() -> IOResult<()> {
    HttpServer::new(move || {
        let public_routes = get_public_routes();
        let protected_routes = get_protected_routes();
        App::new().service(public_routes).service(protected_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
