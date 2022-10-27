use crate::api::authentication::login;
use crate::api::products::{create_product, delete_product, get_all_products, get_product, update_product};
use crate::api::users::{create_user, delete_user, get_all_users, get_user, update_user};
use actix_web::body::{BoxBody, EitherBody};
use actix_web::{web::scope, Scope};

use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web_httpauth::middleware::HttpAuthentication;


use crate::services::authentication_service::validator;

pub fn get_protected_routes() -> Scope<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<EitherBody<BoxBody>>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let auth = HttpAuthentication::bearer(validator);
    scope("")
        .wrap(auth)
        .service(get_users_routes())
        .service(get_products_routes())
}
pub fn get_public_routes() -> Scope {
    scope("/login").service(login)
}

pub fn get_users_routes() -> actix_web::Scope {
    scope("/users")
        .service(create_user)
        .service(get_all_users)
        .service(get_user)
        .service(update_user)
        .service(delete_user)
}

pub fn get_products_routes() -> Scope {
    scope("/products")
        .service(create_product)
        .service(get_all_products)
        .service(get_product)
        .service(update_product)
        .service(delete_product)
}
