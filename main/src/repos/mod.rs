pub mod products_repo;
pub mod users_repo;

use products_repo::ProductsRepo;
use users_repo::UsersRepo;
pub struct Repositories {
    pub products_repo: ProductsRepo,
    pub users_repo: UsersRepo,
}
