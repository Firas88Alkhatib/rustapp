#[cfg(test)]
mod authentication_tests {
    use crate::services::authentication_service::{create_jwt, decode_jwt, hash_password, verify_password, Claims};
    use chrono::{Duration, Utc};

    #[test]
    fn hash_verify_password_test() {
        let password = String::from("Te5t P@ssW0rd");
        let hashed = hash_password(password.clone()).expect("Should hash password");
        let is_password_verified = verify_password(hashed, password).expect("Should verify password");
        assert_eq!(is_password_verified, true);
    }
    #[test]
    fn jwt_encode_decode_test() {
        let username = String::from("testusername");
        let permissions = vec![String::from("ROLE_ADMIN"), String::from("ROLE_USER")];
        let claim = Claims::new(username.clone(), permissions.clone());
        let jwt = create_jwt(claim).expect("Should generate JWT token");
        let decoded_claim = decode_jwt(&jwt).expect("Should decode JWT token");

        let exp = (Utc::now() + Duration::hours(24)).timestamp();
        assert_eq!(username, decoded_claim.username);
        assert_eq!(permissions, decoded_claim.permissions);
        assert!(decoded_claim.exp <= exp);
    }
}
