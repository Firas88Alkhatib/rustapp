use crate::api::categories::{create_category, delete_category, get_all_categories, get_category, update_category};
use crate::api::products::{create_product, delete_product, get_all_products, get_product, update_product};
use actix_web::{web::scope, Scope};

pub fn get_products_routes() -> Scope {
    // sure return and let can be omitted but it is more readable this way
    let users_scope = scope("/products")
        .service(create_product)
        .service(get_all_products)
        .service(get_product)
        .service(update_product)
        .service(delete_product);
    return users_scope;
}

pub fn get_categories_routes() -> Scope {
    let users_scope = scope("/categories")
        .service(create_category)
        .service(get_all_categories)
        .service(get_category)
        .service(update_category)
        .service(delete_category);
    return users_scope;
}
