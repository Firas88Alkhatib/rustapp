use crate::models::user::{User, UserDto};
use crate::repos::users_repo;
use actix_web::{delete, get, post, put, web::Json, web::Path, HttpResponse};

#[post("")]
pub async fn create_user(user: Json<UserDto>) -> Json<User> {
    let new_user = users_repo::create_user(user.into_inner()).await;
    return Json(new_user);
}

#[get("")]
pub async fn get_all_users() -> Json<Vec<User>> {
    let result_users = users_repo::get_all_users().await;
    return Json(result_users);
}

#[get("/{id}")]
pub async fn get_user(path: Path<i32>) -> Json<User> {
    let user_id: i32 = path.into_inner();
    let user = users_repo::get_user(user_id).await;
    return Json(user);
}

#[put("/{id}")]
pub async fn update_user(path: Path<i32>, user: Json<User>) -> Json<User> {
    let updated_user = users_repo::update_user(path.into_inner(), user.into_inner()).await;
    return Json(updated_user);
}

#[delete("/{id}")]
pub async fn delete_user(path: Path<i32>) -> HttpResponse {
    let user_id: i32 = path.into_inner();
    users_repo::delete_user(user_id).await;
    return HttpResponse::NoContent().finish();
}
