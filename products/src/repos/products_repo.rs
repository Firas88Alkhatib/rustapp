use crate::models::categories::Category;
use crate::models::productcategory::{NewProductCategory, ProductCategory};
use crate::models::products::{NewProduct, Product, ProductInDto, ProductOutDto};
use crate::schema::categories::dsl::categories as DbCategories;
use crate::schema::products::dsl::products as DbProducts;
use crate::schema::productscategories::dsl::productscategories as DbProductsCategories;
use diesel::{result::Error, BelongingToDsl, QueryDsl, RunQueryDsl, Table};

use crate::db::{DbConnection, DbPool};

pub struct ProductsRepo {
    connection_pool: DbPool,
}

impl ProductsRepo {
    pub fn new(connection_pool: DbPool) -> Self {
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
        let inserted_product = diesel::insert_into(DbProducts)
            .values(&new_product)
            .get_result::<Product>(&mut connection)?;
        let new_product_categories: Vec<NewProductCategory> = product
            .categories
            .iter()
            .map(|category| NewProductCategory {
                category_id: &category.id,
                product_id: &inserted_product.id,
            })
            .collect();

        diesel::insert_into(DbProductsCategories)
            .values(&new_product_categories)
            .execute(&mut connection)?;
        return Ok(inserted_product);
    }
    pub async fn get_product(&self, product_id: i32) -> Result<ProductOutDto, Error> {
        let mut connection = self.get_connection();
        // get product
        let product: Product = DbProducts.find(product_id).first(&mut connection)?;
        // get product related categories using the join table
        let product_categories: Vec<Category> = ProductCategory::belonging_to(&product)
            .inner_join(DbCategories)
            .select(DbCategories::all_columns())
            .load::<Category>(&mut connection)?;

        // instanciate the out dto
        let results = ProductOutDto {
            id: product.id,
            name: product.name,
            description: product.description,
            categories: product_categories,
        };
        return Ok(results);
    }
    pub async fn get_all_products(&self) -> Result<Vec<ProductOutDto>, Error> {
        let mut connection = self.get_connection();
        let query_result: Vec<(Product, Category)> = DbProducts
            .inner_join(DbProductsCategories.inner_join(DbCategories))
            .select((DbProducts::all_columns(), DbCategories::all_columns()))
            .load::<(Product, Category)>(&mut connection)?;

        let result: Vec<ProductOutDto> = query_result.iter().fold(Vec::new(), |mut acc, (product, category)| {
            // "iterator::find" will stop processing as soon as it's closure returns true
            let acc_product = acc.iter_mut().find(|acc_product| acc_product.id == product.id);
            match acc_product {
                Some(item) => item.categories.push(category.clone()),
                None => acc.push(ProductOutDto {
                    id: product.id,
                    name: product.name.clone(),
                    description: product.description.clone(),
                    categories: vec![category.clone()],
                }),
            }
            return acc;
        });
        return Ok(result);
    }
    pub async fn delete_product(&self, product_id: i32) -> Result<usize, Error> {
        let mut connection = self.get_connection();
        let result = diesel::delete(DbProducts.find(product_id)).execute(&mut connection);
        return result;
    }
    pub async fn update_product(&self, product_id: i32, product: Product) -> Result<Product, Error> {
        let mut connection = self.get_connection();

        let updated_product = diesel::update(DbProducts.find(product_id))
            .set(&product)
            .get_result::<Product>(&mut connection);
        return updated_product;
    }
}
