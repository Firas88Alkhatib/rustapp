use crate::error_handle::DatabaseError;
use crate::models::products::{Product, ProductInDto};
use crate::repos::Repositories;
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use diesel::result::Error;

#[post("")]
pub async fn create_product(repos: Data<Repositories>, product: Json<ProductInDto>) -> Result<Json<Product>, DatabaseError> {
    let new_product = repos.products_repo.create_product(product.into_inner()).await?;
    Ok(Json(new_product))
}

#[get("")]
pub async fn get_all_products(repos: Data<Repositories>) -> Result<Json<Vec<Product>>, DatabaseError> {
    let result_products = repos.products_repo.get_all_products().await?;
    return Ok(Json(result_products));
}

#[get("/{id}")]
pub async fn get_product(repos: Data<Repositories>, path: Path<i32>) -> Result<Json<Product>, DatabaseError> {
    let product_id: i32 = path.into_inner();
    let product = repos.products_repo.get_product(product_id).await?;
    return Ok(Json(product));
}

#[put("/{id}")]
pub async fn update_product(repos: Data<Repositories>, path: Path<i32>, product: Json<Product>) -> Result<Json<Product>, DatabaseError> {
    let updated_product = repos.products_repo.update_product(path.into_inner(), product.into_inner()).await?;
    return Ok(Json(updated_product));
}

#[delete("/{id}")]
pub async fn delete_product(repos: Data<Repositories>, path: Path<i32>) -> Result<HttpResponse, DatabaseError> {
    let product_id: i32 = path.into_inner();
    let affected_rows = repos.products_repo.delete_product(product_id).await?;
    match affected_rows {
        0 => Err(DatabaseError {
            error_type: Error::NotFound,
        }),
        _ => Ok(HttpResponse::NoContent().finish()),
    }
}
