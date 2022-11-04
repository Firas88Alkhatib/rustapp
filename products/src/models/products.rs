use crate::models::categories::Category;
use crate::schema::products;
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[diesel(table_name = products)]
pub struct NewProduct<'a> {
    pub name: &'a String,
    pub description: &'a String,
}
#[derive(Identifiable, Deserialize, Serialize, Debug, Queryable, AsChangeset, Clone)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
pub struct ProductInDto {
    pub name: String,
    pub description: String,
    pub categories: Vec<Category>,
}
#[derive(Deserialize, Serialize, Debug, Queryable)]
pub struct ProductOutDto {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub categories: Vec<Category>,
}
