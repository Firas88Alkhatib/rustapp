use crate::models::products::Product;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CategoryRequestDto {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct CategoryResponseDto {
    pub name: String,
    pub categories: Vec<Product>,
}
