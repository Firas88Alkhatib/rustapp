use crate::config::urls::{get_database_url, get_products_service_url};
use crate::db::{get_connection_pool, DbPool};
use crate::models::product::{Product, ProductDto};
use crate::models::user::{User, UserDto};
use crate::repos::products_repo::ProductsRepo;
use crate::repos::users_repo::UsersRepo;
use crate::services::authentication_service::{create_jwt, Claims};
use rand::Rng;

#[allow(dead_code)]
pub fn gen_random_string() -> String {
    let mut rng = rand::thread_rng();
    let random_str = rng.gen::<u32>().to_string();
    return random_str;
}
#[allow(dead_code)]
pub fn get_test_jwt(admin: bool) -> String {
    let username = String::from("admin");
    let mut permissions = vec![String::from("ROLE_USER")];
    if admin {
        permissions.push(String::from("ROLE_ADMIN"))
    }
    let claim = Claims::new(username, permissions);
    let token = create_jwt(claim).expect("cannot generate test JWT token");
    return String::from("Bearer ") + &token;
}

pub fn get_pool() -> DbPool {
    get_connection_pool(get_database_url()).clone()
}
#[allow(dead_code)]
pub async fn add_test_user(permissions: Vec<&str>) -> User {
    let username = String::from("testuser") + &gen_random_string();
    let password = String::from("testpassword") + &gen_random_string();
    let first_name = String::from("first name") + &gen_random_string();
    let last_name = String::from("last name") + &gen_random_string();
    let roles = permissions.iter().map(|s| s.to_string()).collect();
    let user = UserDto {
        username,
        password,
        roles,
        first_name,
        last_name,
    };
    
    let users_repo = UsersRepo::new(get_pool());
    let created_user = users_repo.create_user(user).await.expect("unable to seed test user");
    return created_user;
}
#[allow(dead_code)]
pub async fn add_test_product() -> Product {
    let products_service_url = get_products_service_url();
    let products_repo = ProductsRepo::new(products_service_url);
    let name = String::from("testname") + &gen_random_string();
    let description = String::from("testdesc") + &gen_random_string();
    let product = ProductDto { name, description };
    let created_product = products_repo.create_product(product).await.expect("unable to create test product");
    return created_product;
}
