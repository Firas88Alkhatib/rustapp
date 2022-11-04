use crate::error_handle::RepositoryError;
use crate::models::user::{UpdateUserDto, User, UserDto, UserOutDto};
use crate::repos::Repositories;
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use actix_web_grants::proc_macro::has_roles;

#[post("")]
#[has_roles("ADMIN")]
pub async fn create_user(repos: Data<Repositories>, user: Json<UserDto>) -> Result<Json<UserOutDto>, RepositoryError> {
    let new_user = repos.users_repo.create_user(user.into_inner()).await?;
    Ok(Json(new_user))
}

#[get("")]
#[has_roles("ADMIN")]
pub async fn get_all_users(repos: Data<Repositories>) -> Result<Json<Vec<UserOutDto>>, RepositoryError> {
    let result_users = repos.users_repo.get_all_users().await?;
    return Ok(Json(result_users));
}

#[get("/{id}")]
#[has_roles("ADMIN")]
pub async fn get_user(repos: Data<Repositories>, path: Path<i32>) -> Result<Json<UserOutDto>, RepositoryError> {
    let user_id: i32 = path.into_inner();
    let user = repos.users_repo.get_user(user_id).await?;
    return Ok(Json(user));
}

#[put("/{id}")]
#[has_roles("ADMIN")]
pub async fn update_user(repos: Data<Repositories>, path: Path<i32>, user: Json<UpdateUserDto>) -> Result<Json<User>, RepositoryError> {
    let user_id = path.into_inner();
    let db_user = repos.users_repo.get_user(user_id).await?;
    let user_to_update = User {
        id: user.id,
        username: user.username.clone(),
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        password: db_user.password,
    };
    let updated_user = repos.users_repo.update_user(user_id, user_to_update).await?;
    return Ok(Json(updated_user));
}

#[delete("/{id}")]
#[has_roles("ADMIN")]
pub async fn delete_user(repos: Data<Repositories>, path: Path<i32>) -> Result<HttpResponse, RepositoryError> {
    let user_id: i32 = path.into_inner();
    repos.users_repo.delete_user(user_id).await?;
    return Ok(HttpResponse::NoContent().finish());
}
