use crate::services::authentication_service::{create_jwt, verify_password, Claims};

use actix_web::{
    error::ErrorUnauthorized,
    post,
    web::{Data, Json},
    Error as ActixError,
};
use serde::{Deserialize, Serialize};

use crate::repos::Repositories;

#[derive(Serialize, Deserialize)]
pub struct LoginDto {
    username: String,
    password: String,
}

#[post("")]
pub async fn login(repos: Data<Repositories>, login_dto: Json<LoginDto>) -> Result<String, ActixError> {
    let login = login_dto.into_inner();
    let db_user = repos.users_repo.get_user_by_username(login.username).await?;
    let is_valid = verify_password(db_user.password, login.password).expect("Failed to decode password hash ");
    if !is_valid {
        return Err(ErrorUnauthorized("invalid credentials"));
    }

    let permissions = db_user.roles.iter().map(|role| role.name.clone()).collect();
    let claims = Claims::new(db_user.username, permissions);
    let token = create_jwt(claims);
    return token;
}
