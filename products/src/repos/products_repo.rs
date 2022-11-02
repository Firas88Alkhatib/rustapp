use crate::models::products::{NewProduct, Product, ProductInDto};
use crate::schema::products::dsl::*;
use diesel::{result::Error, QueryDsl, RunQueryDsl};

use crate::db::{get_connection_pool, DbConnection, DbPool};

pub struct ProductsRepo {
    connection_pool: DbPool,
}

impl ProductsRepo {
    pub fn new(database_url: String) -> Self {
        let connection_pool = get_connection_pool(database_url);
        return Self { connection_pool };
    }
    fn get_connection(&self) -> DbConnection {
        let connection = self
            .connection_pool
            .get()
            .expect("Failed to get connection from the connection pool");
        return connection;
    }
    pub async fn create_product(&self, product: ProductInDto) -> Result<Product, Error> {
        let mut connection = self.get_connection();

        let new_product: NewProduct = NewProduct {
            name: &product.name,
            description: &product.description,
        };
        let inserted_product = diesel::insert_into(products)
            .values(&new_product)
            .get_result::<Product>(&mut connection);

        return inserted_product;
    }
    pub async fn get_product(&self, product_id: i32) -> Result<Product, Error> {
        let mut connection = self.get_connection();
        let product = products.find(product_id).first(&mut connection);
        return product;
    }
    pub async fn get_all_products(&self) -> Result<Vec<Product>, Error> {
        let mut connection = self.get_connection();
        let results = products.load::<Product>(&mut connection);
        return results;
    }
    pub async fn delete_product(&self, product_id: i32) -> Result<usize, Error> {
        let mut connection = self.get_connection();
        let result = diesel::delete(products.find(product_id)).execute(&mut connection);
        return result;
    }
    pub async fn update_product(&self, product_id: i32, product: Product) -> Result<Product, Error> {
        let mut connection = self.get_connection();

        let updated_product = diesel::update(products.find(product_id))
            .set(&product)
            .get_result::<Product>(&mut connection);

        return updated_product;
    }
}
