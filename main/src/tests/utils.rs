use crate::models::user::{User, UserDto};
use crate::repos::users_repo::UsersRepo;
use crate::services::authentication_service::{create_jwt, Claims};
use rand::Rng;
use std::env;

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
    let database_url = env::var("DATABASE_URL").unwrap_or(String::from("postgres://postgres:postgres@0.0.0.0/rustapp"));
    let users_repo = UsersRepo::new(database_url);
    let created_user = users_repo.create_user(user).await.expect("unable to seed test user");
    return created_user;
}
