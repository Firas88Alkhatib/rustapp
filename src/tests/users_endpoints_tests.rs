#[cfg(test)]
mod user_endpoints_tests {
    use crate::models::user::*;
    use crate::repos::users_repo;
    use crate::routes::get_users_routes;
    use crate::tests::utils::gen_random_string;
    use actix_web::{test, App};

    async fn add_test_user() -> User {
        let first_name = String::from("first name") + &gen_random_string();
        let last_name = String::from("last name") + &gen_random_string();
        let user = UserDto { first_name, last_name };
        let created_user = users_repo::create_user(user).await;
        return created_user;
    }

    #[actix_web::test]
    async fn test_create_user() {
        let first_name = String::from("first name") + &gen_random_string();
        let last_name = String::from("last name") + &gen_random_string();
        let user = UserDto {
            first_name: first_name.clone(),
            last_name: last_name.clone(),
        };

        let user_routes = get_users_routes();
        let app = test::init_service(App::new().service(user_routes)).await;

        let url = "/users";
        let req = test::TestRequest::post().uri(url).set_json(user).to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200, "should return 200 status code ");
        let result: User = test::read_body_json(resp).await;
        assert_eq!(result.first_name, first_name, "first names should be identicals");
        assert_eq!(result.last_name, last_name, "last names should be identicals");
    }
    #[actix_web::test]
    async fn test_get_all_users() {
        let user1 = add_test_user().await;
        let user2 = add_test_user().await;
        let user_routes = get_users_routes();
        let app = test::init_service(App::new().service(user_routes)).await;

        let url = "/users";
        let req = test::TestRequest::get().uri(url).to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200, "should return 200 status code ");
        let result: Vec<User> = test::read_body_json(resp).await;

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
        let user = add_test_user().await;
        let user_routes = get_users_routes();
        let app = test::init_service(App::new().service(user_routes)).await;

        let url = format!("/users/{}", user.id);
        let req = test::TestRequest::get().uri(&url).to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200, "should return 200 status code ");
        let result: User = test::read_body_json(resp).await;
        assert_eq!(result.first_name, user.first_name, "first names should be identicals");
        assert_eq!(result.last_name, user.last_name, "last names should be identicals");
    }
    #[actix_web::test]
    async fn test_update_user() {
        let user = add_test_user().await;
        let updated_first_name = String::from("first name") + &gen_random_string();

        let updated_user = User {
            id: user.id.clone(),
            first_name: updated_first_name.clone(),
            last_name: user.last_name.clone(),
        };

        let user_routes = get_users_routes();
        let app = test::init_service(App::new().service(user_routes)).await;

        let url = format!("/users/{}", user.id);
        let req = test::TestRequest::put().uri(&url).set_json(updated_user).to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200, "should return 200 status code ");
        let result: User = test::read_body_json(resp).await;
        assert_eq!(result.first_name, updated_first_name, "first name should be updated");
        assert_eq!(result.last_name, user.last_name, "last name should not be changed");
    }
    #[actix_web::test]
    async fn test_delete_user() {
        let user = add_test_user().await;

        let user_routes = get_users_routes();
        let app = test::init_service(App::new().service(user_routes)).await;

        let url = format!("/users/{}", user.id);
        let req = test::TestRequest::delete().uri(&url).to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 204, "should return NoContent 204 status code");
    }
}
