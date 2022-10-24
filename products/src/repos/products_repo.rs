use crate::db::get_connection;
use crate::models::products::{NewProduct, Product, ProductDto};
use crate::schema::products::dsl::*;
use diesel::{result::Error, PgConnection, QueryDsl, RunQueryDsl};

pub async fn create_product(product: ProductDto) -> Result<Product, Error> {
    let mut db: PgConnection = get_connection();

    let new_product: NewProduct = NewProduct {
        name: &product.name,
        description: &product.description,
    };
    let inserted_product = diesel::insert_into(products)
        .values(&new_product)
        .get_result::<Product>(&mut db);

    return inserted_product;
}
pub async fn get_product(product_id: i32) -> Result<Product, Error> {
    let mut db: PgConnection = get_connection();
    let product = products.find(product_id).first(&mut db);
    return product;
}
pub async fn get_all_products() -> Result<Vec<Product>, Error> {
    let mut db: PgConnection = get_connection();
    let results = products.load::<Product>(&mut db);
    return results;
}
pub async fn delete_product(product_id: i32) -> Result<usize, Error> {
    let mut connection: PgConnection = get_connection();
    let result = diesel::delete(products.find(product_id)).execute(&mut connection);
    return result;
}
pub async fn update_product(product_id: i32, user: Product) -> Result<Product, Error> {
    let mut db: PgConnection = get_connection();

    let updated_product = diesel::update(products.find(product_id))
        .set(&user)
        .get_result::<Product>(&mut db);

    return updated_product;
}
