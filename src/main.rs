#[macro_use]
extern crate diesel;
mod api;
mod db;
mod models;
mod schema;

use actix_web::{App, HttpServer};
use api::user::{create_user, get_all_users, get_user, update_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_user)
            .service(create_user)
            .service(create_user)
            .service(get_all_users)
            .service(update_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
