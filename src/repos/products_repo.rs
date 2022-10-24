use reqwest;
use std::env;

use crate::models::product::Product;
// setting the BASE_URL as static will compile the value in the binary,
// and will not be able to change it in runtime via env variable
// static BASE_URL: &'static str = "http://localhost:5001/products";

pub async fn get_all_products() -> Vec<Product> {
    let base_url = env::var("PRODUCTS_URL").unwrap_or(String::from("http://localhost:5001/products"));
    let response = reqwest::get(base_url).await.unwrap();
    let products = response.json::<Vec<Product>>().await.unwrap();
    return products;
}
