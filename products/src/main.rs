#[macro_use]
extern crate diesel;
use actix_web::{web::Data, App, HttpServer};
use std::io::Result;

mod api;
mod config;
mod db;
mod error_handle;
mod models;
mod repos;
mod routes;
mod schema;

use crate::config::urls::get_database_url;
use crate::repos::products_repo::ProductRepo;
use crate::routes::get_products_routes;
#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(move || {
        let database_url = get_database_url();
        let products_routes = get_products_routes();
        let products_repo = Data::new(ProductRepo::new(database_url));
        App::new().app_data(products_repo).service(products_routes)
    })
    .bind(("0.0.0.0", 5001))?
    .run()
    .await
}
