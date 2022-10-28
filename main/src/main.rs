mod api;
mod app;
mod db;
mod error_handle;
mod models;
mod repos;
mod routes;
mod schema;
mod services;
mod tests;
mod config;

#[macro_use]
extern crate diesel;
// use crate::routes::{get_protected_routes, get_public_routes};
// use actix_web::{web::Data, App, HttpServer};
// use std::{env, io::Result as IOResult};

use crate::app::init_app;
use actix_web::HttpServer;
// use crate::repos::products_repo::ProductsRepo;
// use crate::repos::users_repo::UsersRepo;

// TODO use dependency injection instead of repos direct import
// implement a struct for database and http client
// fix the tests after authentication implementation

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        init_app()
        // let products_service_url = env::var("PRODUCTS_URL").unwrap_or(String::from("http://localhost:5001/products"));
        // let database_url = env::var("DATABASE_URL").unwrap_or(String::from("postgres://postgres:postgres@0.0.0.0/rustapp"));

        // let users_repo = Data::new(UsersRepo::new(database_url));
        // let products_repo = Data::new(ProductsRepo::new(products_service_url));

        // let public_routes = get_public_routes();
        // let protected_routes = get_protected_routes();
        // App::new()
        //     .app_data(users_repo)
        //     .app_data(products_repo)
        //     .service(public_routes)
        //     .service(protected_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
