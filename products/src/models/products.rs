use crate::models::categories::Category;
use crate::schema::products;
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[diesel(table_name = products)]
pub struct NewProduct<'a> {
    pub name: &'a String,
    pub description: &'a String,
}
#[derive(Deserialize, Serialize, Debug, Queryable, AsChangeset)]
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
}
#[derive(Deserialize, Serialize)]
pub struct ProductOutDto {
    id: i32,
    pub name: String,
    pub description: String,
    categories: Vec<Category>,
}
