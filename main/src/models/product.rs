use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
pub struct ProductDto {
    pub name: String,
    pub description: String,
}
