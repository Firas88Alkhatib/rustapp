use crate::schema::products;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Queryable, AsChangeset)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = products)]
pub struct NewProduct<'a> {
    pub name: &'a String,
    pub description: &'a String,
}

#[derive(Deserialize, Serialize)]
pub struct ProductDto {
    pub name: String,
    pub description: String,
}
