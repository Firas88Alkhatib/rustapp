#[cfg(test)]
mod products_endpoints_tests {
    use crate::app;
    use crate::models::product::{Product, ProductDto};
    use crate::tests::utils::{add_test_product, gen_random_string, get_pool, get_test_jwt};
    use actix_web::{http::header, test};

    #[actix_web::test]
    async fn test_create_product() {
        let name = String::from("testname") + &gen_random_string();
        let description = String::from("testdesc") + &gen_random_string();
        let product = ProductDto { name, description };

        let app = test::init_service(app::init_app(get_pool())).await;

        let url = "/products";
        let un_auth_req = test::TestRequest::post().uri(&url).set_json(&product).to_request();
        let un_auth_resp = test::call_service(&app, un_auth_req).await;
        assert_eq!(un_auth_resp.status(), 401, "should return Unauthorized 401");

        let req = test::TestRequest::post()
            .uri(&url)
            .insert_header((header::AUTHORIZATION, get_test_jwt(false)))
            .set_json(&product)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 201, "should return Created 201 ");
        let result: Product = test::read_body_json(resp).await;
        assert_eq!(result.name, product.name, "Product names should be identicals");
        assert_eq!(result.description, product.description, "Product descriptions should be identicals");
    }

    #[actix_web::test]
    async fn test_get_single_product() {
        let product = add_test_product().await;
        let app = test::init_service(app::init_app(get_pool())).await;

        let url = format!("/products/{}", product.id);
        let un_auth_req = test::TestRequest::get().uri(&url).to_request();
        let un_auth_resp = test::call_service(&app, un_auth_req).await;
        assert_eq!(un_auth_resp.status(), 401, "should return Unauthorized 401");

        let req = test::TestRequest::get()
            .uri(&url)
            .insert_header((header::AUTHORIZATION, get_test_jwt(false)))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200, "should return 200 status code ");
        let result: Product = test::read_body_json(resp).await;
        assert_eq!(result.name, product.name, "Product names should be identicals");
        assert_eq!(result.description, product.description, "Product descriptions should be identicals");
    }
    #[actix_web::test]
    async fn test_get_all_product() {
        let product1 = add_test_product().await;
        let product2 = add_test_product().await;
        let app = test::init_service(app::init_app(get_pool())).await;

        let url = "/products";
        let un_auth_req = test::TestRequest::get().uri(&url).to_request();
        let un_auth_resp = test::call_service(&app, un_auth_req).await;
        assert_eq!(un_auth_resp.status(), 401, "should return Unauthorized 401");

        let req = test::TestRequest::get()
            .uri(&url)
            .insert_header((header::AUTHORIZATION, get_test_jwt(false)))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200, "should return 200 status code ");
        let result: Vec<Product> = test::read_body_json(resp).await;

        let result_product1 = result
            .iter()
            .find(|product| product.name == product1.name && product.description == product1.description);
        let result_product2 = result
            .iter()
            .find(|product| product.name == product2.name && product.description == product2.description);

        assert!(result_product1.is_some(), "Should find inserted product 1");
        assert!(result_product2.is_some(), "Should find inserted product 2");
    }
    #[actix_web::test]
    async fn test_update_product() {
        let product = add_test_product().await;
        let updated_name = String::from("name") + &gen_random_string();
        let updated_product = Product {
            id: product.id,
            name: updated_name.clone(),
            description: product.description.clone(),
        };

        let app = test::init_service(app::init_app(get_pool())).await;

        let url = format!("/products/{}", product.id);
        let un_auth_req = test::TestRequest::put().uri(&url).set_json(&updated_product).to_request();
        let un_auth_resp = test::call_service(&app, un_auth_req).await;
        assert_eq!(un_auth_resp.status(), 401, "should return Unauthorized 401");

        let req = test::TestRequest::put()
            .uri(&url)
            .insert_header((header::AUTHORIZATION, get_test_jwt(false)))
            .set_json(&updated_product)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200, "should return Created 201 ");
        let result: Product = test::read_body_json(resp).await;
        assert_eq!(result.name, updated_name, "Product name should be updated");
        assert_eq!(result.description, product.description, "Product description should not be updated");
    }
    #[actix_web::test]
    async fn test_delete_product() {
        let product = add_test_product().await;
        let app = test::init_service(app::init_app(get_pool())).await;

        let url = format!("/products/{}", product.id);
        let un_auth_req = test::TestRequest::delete().uri(&url).to_request();
        let un_auth_resp = test::call_service(&app, un_auth_req).await;
        assert_eq!(un_auth_resp.status(), 401, "should return Unauthorized 401");

        let req = test::TestRequest::delete()
            .uri(&url)
            .insert_header((header::AUTHORIZATION, get_test_jwt(false)))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 204, "should return NoContent 204 ");
    }
}
