pub mod products_repo;
use products_repo::ProductRepo;

pub struct Repositories {
    pub products_repo: ProductRepo,
}
