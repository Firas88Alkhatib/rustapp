use crate::error_handle::DatabaseError;
use crate::models::user::{User, UserDto};
use crate::repos::users_repo;
use actix_web::{delete, get, post, put, web::Json, web::Path, HttpResponse};
use diesel::result::Error;

#[post("")]
pub async fn create_user(user: Json<UserDto>) -> Result<Json<User>, DatabaseError> {
    let new_user = users_repo::create_user(user.into_inner()).await?;
    Ok(Json(new_user))
}

#[get("")]
pub async fn get_all_users() -> Result<Json<Vec<User>>, DatabaseError> {
    let result_users = users_repo::get_all_users().await?;
    return Ok(Json(result_users));
}

#[get("/{id}")]
pub async fn get_user(path: Path<i32>) -> Result<Json<User>, DatabaseError> {
    let user_id: i32 = path.into_inner();
    let user = users_repo::get_user(user_id).await?;
    return Ok(Json(user));
}

#[put("/{id}")]
pub async fn update_user(path: Path<i32>, user: Json<User>) -> Result<Json<User>, DatabaseError> {
    let updated_user = users_repo::update_user(path.into_inner(), user.into_inner()).await?;
    return Ok(Json(updated_user));
}

#[delete("/{id}")]
pub async fn delete_user(path: Path<i32>) -> Result<HttpResponse, DatabaseError> {
    let user_id: i32 = path.into_inner();
    let affected_rows = users_repo::delete_user(user_id).await?;
    match affected_rows {
        0 => Err(DatabaseError {
            error_type: Error::NotFound,
        }),
        _ => Ok(HttpResponse::NoContent().finish()),
    }
}
