use crate::config::urls;
use crate::db::DbPool;
use crate::repos::{products_repo::ProductsRepo, users_repo::UsersRepo, Repositories};
use crate::routes::{get_protected_routes, get_public_routes};
use actix_web::{
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    error::Error,
    web::Data,
    App,
};

pub fn init_app(
    connection_pool: DbPool,
) -> App<impl ServiceFactory<ServiceRequest, Response = ServiceResponse<impl MessageBody>, Config = (), InitError = (), Error = Error>> {
    let products_service_url = urls::get_products_service_url();
    let repositories = Data::new(Repositories {
        users_repo: UsersRepo::new(connection_pool),
        products_repo: ProductsRepo::new(products_service_url),
    });

    let public_routes = get_public_routes();
    let protected_routes = get_protected_routes();
    App::new().app_data(repositories).service(public_routes).service(protected_routes)
}
