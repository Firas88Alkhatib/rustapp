pub mod categories_repo;
pub mod products_repo;
use categories_repo::CategoriesRepo;
use products_repo::ProductsRepo;

pub struct Repositories {
    pub products_repo: ProductsRepo,
    pub categories_repo: CategoriesRepo,
}
