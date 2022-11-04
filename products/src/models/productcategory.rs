use crate::models::categories::Category;
use crate::models::products::Product;
use crate::schema::productscategories;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Debug)]
#[diesel(table_name = productscategories)]
pub struct NewProductCategory<'a> {
    pub product_id: &'a i32,
    pub category_id: &'a i32,
}
#[derive(Identifiable, Deserialize, Serialize, Debug, Queryable, AsChangeset, Associations)]
#[belongs_to(Product)]
#[belongs_to(Category)]
#[diesel(table_name = productscategories)]
pub struct ProductCategory {
    pub id: i32,
    pub product_id: i32,
    pub category_id: i32,
}
