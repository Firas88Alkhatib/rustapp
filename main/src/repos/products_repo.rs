use std::env;

use crate::error_handle::{RepositoryError, RepositoryErrorType};
use crate::models::product::{Product, ProductDto};
use reqwest::{Client, StatusCode};
// setting the BASE_URL as static will compile the value in the binary,
// and will not be able to change it in runtime via env variable
// static BASE_URL: &'static str = "http://localhost:5001/products";

pub async fn create_product(product_dto: ProductDto) -> Result<Product, RepositoryError> {
    let base_url = env::var("PRODUCTS_URL").unwrap_or(String::from("http://localhost:5001/products"));
    let client = Client::new();
    let response = client.post(base_url).json(&product_dto).send().await?;

    let product = response.json::<Product>().await?;
    return Ok(product);
}
pub async fn get_all_products() -> Result<Vec<Product>, RepositoryError> {
    let base_url = env::var("PRODUCTS_URL").unwrap_or(String::from("http://localhost:5001/products"));
    let client = Client::new();
    let response = client.get(base_url).send().await?;
    let products = response.json::<Vec<Product>>().await?;
    return Ok(products);
}

pub async fn get_product(product_id: i32) -> Result<Product, RepositoryError> {
    let base_url = env::var("PRODUCTS_URL").unwrap_or(String::from("http://localhost:5001/products"));
    let url = format!("{}/{}", base_url, product_id);
    let client = Client::new();
    let response = client.get(url).send().await?;
    if response.status() == StatusCode::NOT_FOUND {
        return Err(RepositoryError {
            error_type: RepositoryErrorType::NotFound,
        });
    }
    let product = response.json::<Product>().await?;
    return Ok(product);
}
pub async fn update_product(product_id: i32, product: Product) -> Result<Product, RepositoryError> {
    let base_url = env::var("PRODUCTS_URL").unwrap_or(String::from("http://localhost:5001/products"));
    let url = format!("{}/{}", base_url, product_id);
    let client = Client::new();
    let response = client.put(url).json(&product).send().await?;
    let product = response.json::<Product>().await?;
    return Ok(product);
}

pub async fn delete_product(product_id: i32) -> Result<(), RepositoryError> {
    let base_url = env::var("PRODUCTS_URL").unwrap_or(String::from("http://localhost:5001/products"));
    let url = format!("{}/{}", base_url, product_id);
    let client = Client::new();
    client.delete(url).send().await?;
    return Ok(());
}
