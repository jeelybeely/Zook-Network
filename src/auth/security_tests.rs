// File: src/auth/security_tests.rs

#[cfg(test)]
mod tests {
    use warp::test::request;
    use warp::Filter;
    use std::sync::Arc;
    use crate::auth::security::{secure_filter, verify_token};

    #[tokio::test]
    async fn test_valid_authentication() {
        let filter = secure_filter("secure_token".to_string());

        let response = request()
            .method("GET")
            .header("Authorization", "Bearer secure_token")
            .filter(&filter)
            .await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_authentication() {
        let filter = secure_filter("secure_token".to_string());

        let response = request()
            .method("GET")
            .header("Authorization", "Bearer invalid_token")
            .filter(&filter)
            .await;

        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_missing_authentication() {
        let filter = secure_filter("secure_token".to_string());

        let response = request()
            .method("GET")
            .filter(&filter)
            .await;

        assert!(response.is_err());
    }
}
