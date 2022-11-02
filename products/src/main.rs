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
use crate::repos::{categories_repo::CategoriesRepo, products_repo::ProductsRepo, Repositories};
use crate::routes::{get_categories_routes, get_products_routes};

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(move || {
        let database_url = get_database_url();
        let products_routes = get_products_routes();
        let categories_routes = get_categories_routes();
        let repositories = Data::new(Repositories {
            products_repo: ProductsRepo::new(database_url.clone()),
            categories_repo: CategoriesRepo::new(database_url.clone()),
        });

        App::new()
            .app_data(repositories)
            .service(products_routes)
            .service(categories_routes)
    })
    .bind(("0.0.0.0", 5001))?
    .run()
    .await
}
