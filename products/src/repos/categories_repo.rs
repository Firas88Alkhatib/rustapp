use crate::models::categories::{Category, CategoryInDto, CategoryOutDto, NewCategory};
use crate::schema::categories::dsl::categories;
use diesel::{result::Error, QueryDsl, RunQueryDsl};

use crate::db::{DbConnection, DbPool};
pub struct CategoriesRepo {
    connection_pool: DbPool,
}

impl CategoriesRepo {
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
    pub async fn create_category(&self, category: CategoryInDto) -> Result<Category, Error> {
        let mut connection = self.get_connection();

        let new_category: NewCategory = NewCategory { name: &category.name };
        let inserted_category = diesel::insert_into(categories)
            .values(&new_category)
            .get_result::<Category>(&mut connection);

        return inserted_category;
    }
    pub async fn get_category(&self, category_id: i32) -> Result<Category, Error> {
        let mut connection = self.get_connection();
        let category = categories.find(category_id).first(&mut connection);
        return category;
    }
    pub async fn get_all_categories(&self) -> Result<Vec<Category>, Error> {
        let mut connection = self.get_connection();
        let results = categories.load::<Category>(&mut connection);
        return results;
    }
    pub async fn delete_category(&self, category_id: i32) -> Result<usize, Error> {
        let mut connection = self.get_connection();
        let result = diesel::delete(categories.find(category_id)).execute(&mut connection);
        return result;
    }
    pub async fn update_category(&self, category_id: i32, category: Category) -> Result<Category, Error> {
        let mut connection = self.get_connection();

        let updated_category = diesel::update(categories.find(category_id))
            .set(&category)
            .get_result::<Category>(&mut connection);

        return updated_category;
    }
}
