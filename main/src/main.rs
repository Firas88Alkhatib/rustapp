mod api;
mod db;
mod error_handle;
mod models;
mod repos;
mod routes;
mod schema;
mod tests;

#[macro_use]
extern crate diesel;
use crate::routes::{get_products_routes, get_users_routes};
use actix_web::{App, HttpServer};
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        let users_routes = get_users_routes();
        let products_routes = get_products_routes();
        App::new().service(users_routes).service(products_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
