use std::env;

pub fn get_database_url() -> String {
    let database_url = env::var("DATABASE_URL").unwrap_or(String::from("postgres://postgres:postgres@0.0.0.0/rustapp"));
    return database_url;
}
pub fn get_products_service_url() -> String {
    let products_service_url = env::var("PRODUCTS_URL").unwrap_or(String::from("http://localhost:5001/products"));
    return products_service_url;
}
