#[macro_use]
extern crate diesel;
use actix_web::{App, HttpServer};
use std::io::Result;

mod api;
mod db;
mod models;
mod repos;
mod routes;
mod schema;

use crate::routes::{get_products_routes, get_users_routes};

mod tests;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        let users_routes = get_users_routes();
        let products_routes = get_products_routes();
        App::new().service(users_routes).service(products_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
