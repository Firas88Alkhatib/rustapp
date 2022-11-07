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
mod utils;

use config::urls::get_database_url;
use db::get_connection_pool;
use repos::{categories_repo::CategoriesRepo, products_repo::ProductsRepo, Repositories};
use routes::{get_categories_routes, get_products_routes};
use utils::stdout_utils::{print_solita, printc};

#[actix_web::main]
async fn main() -> Result<()> {
    print_solita();
    let database_url = get_database_url();
    let connection_pool = get_connection_pool(database_url.clone());
    HttpServer::new(move || {
        let products_routes = get_products_routes();
        let categories_routes = get_categories_routes();
        let repositories = Data::new(Repositories {
            products_repo: ProductsRepo::new(connection_pool.clone()),
            categories_repo: CategoriesRepo::new(connection_pool.clone()),
        });
        printc("Starting PRODUCTS service server", (239, 62, 66));
        App::new()
            .app_data(repositories)
            .service(products_routes)
            .service(categories_routes)
    })
    .bind(("0.0.0.0", 5001))?
    .run()
    .await
}
