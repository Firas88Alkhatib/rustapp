use crate::error_handle::{RepositoryError, RepositoryErrorType};
use crate::models::product::{Product, ProductDto};
use reqwest::{Client, StatusCode};

pub struct ProductsRepo {
    pub base_url: String,
    pub client: Client,
}

impl ProductsRepo {
    pub fn new(base_url: String) -> Self {
        let client = Client::new();
        return Self { base_url, client };
    }
    pub async fn create_product(&self, product_dto: ProductDto) -> Result<Product, RepositoryError> {
        let client = &self.client;
        let url = format!("{}", self.base_url);
        let response = client.post(url).json(&product_dto).send().await?;
        let product = response.json::<Product>().await?;
        return Ok(product);
    }
    pub async fn get_all_products(&self) -> Result<Vec<Product>, RepositoryError> {
        let client = &self.client;
        let url = format!("{}", self.base_url);
        let response = client.get(url).send().await?;
        let products = response.json::<Vec<Product>>().await?;
        return Ok(products);
    }
    pub async fn get_product(&self, product_id: i32) -> Result<Product, RepositoryError> {
        let client = &self.client;
        let url = format!("{}/{}", self.base_url, product_id);
        let response = client.get(url).send().await?;
        if response.status() == StatusCode::NOT_FOUND {
            return Err(RepositoryError {
                error_type: RepositoryErrorType::NotFound,
            });
        }
        let product = response.json::<Product>().await?;
        return Ok(product);
    }
    pub async fn update_product(&self, product_id: i32, product: Product) -> Result<Product, RepositoryError> {
        let client = &self.client;
        let url = format!("{}/{}", self.base_url, product_id);
        let response = client.put(url).json(&product).send().await?;
        let product = response.json::<Product>().await?;
        return Ok(product);
    }

    pub async fn delete_product(&self, product_id: i32) -> Result<(), RepositoryError> {
        let client = &self.client;
        let url = format!("{}/{}", self.base_url, product_id);
        client.delete(url).send().await?;
        return Ok(());
    }
}
