use crate::models::products::Product;
use crate::schema::categories;
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[diesel(table_name = categories)]
pub struct NewCategory<'a> {
    pub name: &'a String,
}

#[derive(Identifiable, Deserialize, Serialize, Debug, Queryable, AsChangeset, Clone)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct CategoryInDto {
    pub name: String,
}
#[derive(Deserialize, Serialize)]
pub struct CategoryOutDto {
    id: i32,
    pub name: String,
    products: Vec<Product>,
}
