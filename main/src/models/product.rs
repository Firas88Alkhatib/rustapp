use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub categories: Vec<Category>,
}

#[derive(Deserialize, Serialize)]
pub struct ProductDto {
    pub name: String,
    pub description: String,
    pub categories: Vec<Category>,
}
