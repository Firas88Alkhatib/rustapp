use crate::error_handle::RepositoryError;
use crate::models::user::{User, UserDto};
use crate::repos::users_repo::UsersRepo;
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use actix_web_grants::proc_macro::has_roles;

#[post("")]
#[has_roles("ADMIN")]
pub async fn create_user(users_repo: Data<UsersRepo>, user: Json<UserDto>) -> Result<Json<User>, RepositoryError> {
    let new_user = users_repo.create_user(user.into_inner()).await?;
    Ok(Json(new_user))
}

#[get("")]
#[has_roles("ADMIN")]
pub async fn get_all_users(users_repo: Data<UsersRepo>) -> Result<Json<Vec<User>>, RepositoryError> {
    let result_users = users_repo.get_all_users().await?;
    return Ok(Json(result_users));
}

#[get("/{id}")]
#[has_roles("ADMIN")]
pub async fn get_user(users_repo: Data<UsersRepo>, path: Path<i32>) -> Result<Json<User>, RepositoryError> {
    let user_id: i32 = path.into_inner();
    let user = users_repo.get_user(user_id).await?;
    return Ok(Json(user));
}

#[put("/{id}")]
#[has_roles("ADMIN")]
pub async fn update_user(users_repo: Data<UsersRepo>, path: Path<i32>, user: Json<User>) -> Result<Json<User>, RepositoryError> {
    let updated_user = users_repo.update_user(path.into_inner(), user.into_inner()).await?;
    return Ok(Json(updated_user));
}

#[delete("/{id}")]
#[has_roles("ADMIN")]
pub async fn delete_user(users_repo: Data<UsersRepo>, path: Path<i32>) -> Result<HttpResponse, RepositoryError> {
    let user_id: i32 = path.into_inner();
    users_repo.delete_user(user_id).await?;
    return Ok(HttpResponse::NoContent().finish());
}
