use crate::api::users::{create_user, delete_user, get_all_users, get_user, update_user};
use crate::api::products::get_all_products;
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
    let products_scope = scope("/products").service(get_all_products);
    return products_scope;
}
