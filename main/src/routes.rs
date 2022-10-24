use crate::api::products::{create_product, delete_product, get_all_products, get_product, update_product};
use crate::api::users::{create_user, delete_user, get_all_users, get_user, update_user};
use actix_web::{web::scope, Scope};

pub fn get_users_routes() -> Scope {
    // sure return and let can be omitted but it is more readable this way
    let users_scope = scope("/users")
        .service(create_user)
        .service(get_all_users)
        .service(get_user)
        .service(update_user)
        .service(delete_user);
    return users_scope;
}

pub fn get_products_routes() -> Scope {
    let products_scope = scope("/products")
        .service(create_product)
        .service(get_all_products)
        .service(get_product)
        .service(update_product)
        .service(delete_product);
    return products_scope;
}
