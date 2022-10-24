use crate::models::product::*;
use crate::repos::products_repo;
use actix_web::{get, web::Json};

#[get("")]
pub async fn get_all_products() -> Json<Vec<Product>> {
    let result_users = products_repo::get_all_products().await;
    return Json(result_users);
}
