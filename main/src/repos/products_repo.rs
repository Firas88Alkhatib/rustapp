use reqwest::Error;
use std::env;

use crate::models::product::{Product, ProductDto};
// setting the BASE_URL as static will compile the value in the binary,
// and will not be able to change it in runtime via env variable
// static BASE_URL: &'static str = "http://localhost:5001/products";

pub async fn create_product(product_dto: ProductDto) -> Result<Product, Error> {
    let base_url = env::var("PRODUCTS_URL").unwrap_or(String::from("http://localhost:5001/products"));
    let client = reqwest::Client::new();
    let response = client.post(base_url).json(&product_dto).send().await?;
    let product = response.json::<Product>().await?;
    return Ok(product);
}
pub async fn get_all_products() -> Result<Vec<Product>, Error> {
    let base_url = env::var("PRODUCTS_URL").unwrap_or(String::from("http://localhost:5001/products"));
    let response = reqwest::get(base_url).await?;
    let products = response.json::<Vec<Product>>().await?;
    return Ok(products);
}

pub async fn get_product(product_id: i32) -> Result<Product, Error> {
    let base_url = env::var("PRODUCTS_URL").unwrap_or(String::from("http://localhost:5001/products"));
    let url = format!("{}/{}", base_url, product_id);
    let response = reqwest::get(url).await?;
    let product = response.json::<Product>().await?;
    return Ok(product);
}
pub async fn update_product(product_id: i32, product: Product) -> Result<Product, Error> {
    let base_url = env::var("PRODUCTS_URL").unwrap_or(String::from("http://localhost:5001/products"));
    let url = format!("{}/{}", base_url, product_id);
    let client = reqwest::Client::new();
    let response = client.put(url).json(&product).send().await?;
    let product = response.json::<Product>().await?;
    return Ok(product);
}

pub async fn delete_product(product_id: i32) -> Result<(), Error> {
    let base_url = env::var("PRODUCTS_URL").unwrap_or(String::from("http://localhost:5001/products"));
    let url = format!("{}/{}", base_url, product_id);
    let client = reqwest::Client::new();
    let response = client.delete(url).send().await?;
    // let products = response.json::<Vec<Product>>().await?;
    return Ok(());
}
