pub mod products_repo;
pub mod users_repo;

use products_repo::ProductsRepo;
use users_repo::UsersRepo;
pub struct Repositories {
    products_repo: ProductsRepo,
    users_repo: UsersRepo,
}
