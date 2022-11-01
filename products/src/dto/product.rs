use crate::models::category::Category;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ProductRequestDto {
    pub name: String,
    pub description: String,
    pub categories: Vec<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct ProductResponseDto {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub categories: Vec<Category>,
}
