#[macro_use]
extern crate diesel;
use actix_web::{App, HttpServer};
use std::io::Result;

mod api;
mod db;
mod error_handle;
mod models;
mod repos;
mod routes;
mod schema;

use crate::routes::get_products_routes;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        let products_routes = get_products_routes();
        App::new().service(products_routes)
    })
    .bind(("0.0.0.0", 5001))?
    .run()
    .await
}
