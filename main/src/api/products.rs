use crate::models::product::*;
use crate::repos::products_repo;
use actix_web::{delete, get, post, put, web::Json, web::Path, HttpResponse};

#[post("")]
pub async fn create_product(product: Json<ProductDto>) -> Json<Product> {
    let new_product = products_repo::create_product(product.into_inner()).await.unwrap();
    return Json(new_product);
}

#[get("")]
pub async fn get_all_products() -> Json<Vec<Product>> {
    let result_users = products_repo::get_all_products().await.unwrap();
    return Json(result_users);
}
#[get("/{id}")]
pub async fn get_product(path: Path<i32>) -> Json<Product> {
    let product_id: i32 = path.into_inner();
    let product = products_repo::get_product(product_id).await.unwrap();
    return Json(product);
}
#[put("/{id}")]
pub async fn update_product(path: Path<i32>, product: Json<Product>) -> Json<Product> {
    let product_id = path.into_inner();
    let updated_product = products_repo::update_product(product_id, product.into_inner())
        .await
        .unwrap();
    return Json(updated_product);
}

#[delete("/{id}")]
pub async fn delete_product(path: Path<i32>) -> HttpResponse {
    let product_id: i32 = path.into_inner();
    let result = products_repo::delete_product(product_id).await.unwrap();
    return HttpResponse::NoContent().finish();
}
