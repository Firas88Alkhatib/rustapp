#[cfg(test)]
mod user_endpoints_tests {

    use crate::app;
    use crate::models::role::Role;
    use crate::models::user::UserDto;
    use crate::tests::utils::{add_test_user, gen_random_string, get_pool, get_test_jwt, Roles};
    use actix_web::{http::header, test};
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    struct TestUser {
        pub id: i32,
        pub username: String,
        pub roles: Vec<Role>,
        pub first_name: String,
        pub last_name: String,
    }

    #[actix_web::test]
    async fn test_create_user() {
        let username = String::from("username") + &gen_random_string();
        let password = String::from("password") + &gen_random_string();
        let first_name = String::from("first name") + &gen_random_string();
        let last_name = String::from("last name") + &gen_random_string();
        let roles = vec![Role {
            id: 2,
            name: String::from("ROLE_USER"),
        }];
        let test_user = UserDto {
            username: username.clone(),
            password: password.clone(),
            first_name: first_name.clone(),
            last_name: last_name.clone(),
            roles: roles.clone(),
        };

        let app = test::init_service(app::init_app(get_pool())).await;

        let url = "/users";
        let un_auth_req = test::TestRequest::post().uri(url).set_json(&test_user).to_request();
        let un_auth_resp = test::call_service(&app, un_auth_req).await;
        assert_eq!(un_auth_resp.status(), 401, "should return Unauthorized 401");
        let forbidden_req = test::TestRequest::post()
            .uri(url)
            .insert_header((header::AUTHORIZATION, get_test_jwt(false)))
            .set_json(&test_user)
            .to_request();
        let forbidden_resp = test::call_service(&app, forbidden_req).await;
        assert_eq!(forbidden_resp.status(), 403, "should return Forbidden 401");

        let req = test::TestRequest::post()
            .uri(url)
            .insert_header((header::AUTHORIZATION, get_test_jwt(true)))
            .set_json(test_user)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200, "should return 200 status code ");
        let result: TestUser = test::read_body_json(resp).await;
        assert_eq!(result.username, username, "usernames should be identicals");
        assert_eq!(result.first_name, first_name, "first names should be identicals");
        assert_eq!(result.last_name, last_name, "last names should be identicals");
        assert_eq!(result.roles[0].name, roles[0].name, "roles should be identicals");
    }
    #[actix_web::test]
    async fn test_get_all_users() {
        let user1 = add_test_user(Roles::User).await;
        let user2 = add_test_user(Roles::User).await;
        let app = test::init_service(app::init_app(get_pool())).await;

        let url = "/users";
        let un_auth_req = test::TestRequest::get().uri(url).to_request();
        let un_auth_resp = test::call_service(&app, un_auth_req).await;
        assert_eq!(un_auth_resp.status(), 401, "should return Unauthorized 401");
        let forbidden_req = test::TestRequest::get()
            .uri(url)
            .insert_header((header::AUTHORIZATION, get_test_jwt(false)))
            .to_request();
        let forbidden_resp = test::call_service(&app, forbidden_req).await;
        assert_eq!(forbidden_resp.status(), 403, "should return Forbidden 401");

        let req = test::TestRequest::get()
            .uri(url)
            .insert_header((header::AUTHORIZATION, get_test_jwt(true)))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200, "should return 200 status code ");
        let result: Vec<TestUser> = test::read_body_json(resp).await;

        let result_user1 = result
            .iter()
            .find(|usr| usr.first_name == user1.first_name && usr.last_name == user1.last_name);
        let result_user2 = result
            .iter()
            .find(|usr| usr.first_name == user2.first_name && usr.last_name == user2.last_name);

        assert!(result_user1.is_some(), "Should find inserted user 1");
        assert!(result_user2.is_some(), "Should find inserted user 2");
    }

    #[actix_web::test]
    async fn test_get_single_user() {
        let user = add_test_user(Roles::User).await;
        let app = test::init_service(app::init_app(get_pool())).await;

        let url = format!("/users/{}", user.id);
        let un_auth_req = test::TestRequest::get().uri(&url).to_request();
        let un_auth_resp = test::call_service(&app, un_auth_req).await;
        assert_eq!(un_auth_resp.status(), 401, "should return Unauthorized 401");
        let forbidden_req = test::TestRequest::get()
            .uri(&url)
            .insert_header((header::AUTHORIZATION, get_test_jwt(false)))
            .to_request();
        let forbidden_resp = test::call_service(&app, forbidden_req).await;
        assert_eq!(forbidden_resp.status(), 403, "should return Forbidden 401");

        let req = test::TestRequest::get()
            .uri(&url)
            .insert_header((header::AUTHORIZATION, get_test_jwt(true)))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200, "should return 200 status code ");
        let result: TestUser = test::read_body_json(resp).await;
        assert_eq!(result.first_name, user.first_name, "first names should be identicals");
        assert_eq!(result.last_name, user.last_name, "last names should be identicals");
    }
    #[actix_web::test]
    async fn test_update_user() {
        let test_user = add_test_user(Roles::User).await;
        let updated_first_name = String::from("first name") + &gen_random_string();

        let updated_user = TestUser {
            id: test_user.id.clone(),
            username: test_user.username.clone(),
            first_name: updated_first_name.clone(),
            last_name: test_user.last_name.clone(),
            roles: test_user.roles.clone(),
        };

        let app = test::init_service(app::init_app(get_pool())).await;

        let url = format!("/users/{}", test_user.id);
        let un_auth_req = test::TestRequest::put().uri(&url).set_json(&updated_user).to_request();
        let un_auth_resp = test::call_service(&app, un_auth_req).await;
        assert_eq!(un_auth_resp.status(), 401, "should return Unauthorized 401");
        let forbidden_req = test::TestRequest::put()
            .uri(&url)
            .insert_header((header::AUTHORIZATION, get_test_jwt(false)))
            .set_json(&updated_user)
            .to_request();
        let forbidden_resp = test::call_service(&app, forbidden_req).await;
        assert_eq!(forbidden_resp.status(), 403, "should return Forbidden 401");

        let req = test::TestRequest::put()
            .uri(&url)
            .insert_header((header::AUTHORIZATION, get_test_jwt(true)))
            .set_json(updated_user)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200, "should return 200 status code ");
        let result: TestUser = test::read_body_json(resp).await;
        assert_eq!(result.first_name, updated_first_name, "first name should be updated");
        assert_eq!(result.last_name, test_user.last_name, "last name should not be changed");
    }
    #[actix_web::test]
    async fn test_delete_user() {
        let test_user = add_test_user(Roles::User).await;

        let app = test::init_service(app::init_app(get_pool())).await;

        let url = format!("/users/{}", test_user.id);
        let un_auth_req = test::TestRequest::delete().uri(&url).to_request();
        let un_auth_resp = test::call_service(&app, un_auth_req).await;
        assert_eq!(un_auth_resp.status(), 401, "should return Unauthorized 401");
        let forbidden_req = test::TestRequest::delete()
            .uri(&url)
            .insert_header((header::AUTHORIZATION, get_test_jwt(false)))
            .to_request();
        let forbidden_resp = test::call_service(&app, forbidden_req).await;
        assert_eq!(forbidden_resp.status(), 403, "should return Forbidden 401");

        let req = test::TestRequest::delete()
            .uri(&url)
            .insert_header((header::AUTHORIZATION, get_test_jwt(true)))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 204, "should return NoContent 204 status code");
    }
}
