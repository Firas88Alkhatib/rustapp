use crate::error_handle::DatabaseError;
use crate::models::categories::{Category, CategoryInDto};
use crate::repos::Repositories;
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use diesel::result::Error;

#[post("")]
pub async fn create_category(repos: Data<Repositories>, category: Json<CategoryInDto>) -> Result<Json<Category>, DatabaseError> {
    let new_category = repos.categories_repo.create_category(category.into_inner()).await?;
    Ok(Json(new_category))
}

#[get("")]
pub async fn get_all_categories(repos: Data<Repositories>) -> Result<Json<Vec<Category>>, DatabaseError> {
    let result_categories = repos.categories_repo.get_all_categories().await?;
    return Ok(Json(result_categories));
}

#[get("/{id}")]
pub async fn get_category(repos: Data<Repositories>, path: Path<i32>) -> Result<Json<Category>, DatabaseError> {
    let category_id: i32 = path.into_inner();
    let category = repos.categories_repo.get_category(category_id).await?;
    return Ok(Json(category));
}

#[put("/{id}")]
pub async fn update_category(
    repos: Data<Repositories>,
    path: Path<i32>,
    category: Json<Category>,
) -> Result<Json<Category>, DatabaseError> {
    let updated_category = repos
        .categories_repo
        .update_category(path.into_inner(), category.into_inner())
        .await?;
    return Ok(Json(updated_category));
}

#[delete("/{id}")]
pub async fn delete_category(repos: Data<Repositories>, path: Path<i32>) -> Result<HttpResponse, DatabaseError> {
    let category_id: i32 = path.into_inner();
    let affected_rows = repos.categories_repo.delete_category(category_id).await?;
    match affected_rows {
        0 => Err(DatabaseError {
            error_type: Error::NotFound,
        }),
        _ => Ok(HttpResponse::NoContent().finish()),
    }
}
