use crate::error_handle::RepositoryError;
use crate::models::product::*;
use crate::repos::products_repo;
use actix_web::{delete, get, post, put, web::Json, web::Path, HttpResponse};

#[post("")]
pub async fn create_product(product: Json<ProductDto>) -> Result<HttpResponse, RepositoryError> {
    let new_product = products_repo::create_product(product.into_inner()).await?;
    return Ok(HttpResponse::Created().json(new_product));
}

#[get("")]
pub async fn get_all_products() -> Result<Json<Vec<Product>>, RepositoryError> {
    let result_users = products_repo::get_all_products().await?;
    return Ok(Json(result_users));
}
#[get("/{id}")]
pub async fn get_product(path: Path<i32>) -> Result<Json<Product>, RepositoryError> {
    let product_id: i32 = path.into_inner();
    let product = products_repo::get_product(product_id).await?;
    return Ok(Json(product));
}
#[put("/{id}")]
pub async fn update_product(path: Path<i32>, product: Json<Product>) -> Result<Json<Product>, RepositoryError> {
    let product_id = path.into_inner();
    let updated_product = products_repo::update_product(product_id, product.into_inner()).await?;
    return Ok(Json(updated_product));
}

#[delete("/{id}")]
pub async fn delete_product(path: Path<i32>) -> Result<HttpResponse, RepositoryError> {
    let product_id: i32 = path.into_inner();
    products_repo::delete_product(product_id).await?;
    return Ok(HttpResponse::NoContent().finish());
}
