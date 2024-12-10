// File: src/tests/validator_rewards_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use warp::test::request;
    use tokio::sync::Arc;
    use crate::governance::{api::governance_routes, GovernanceModule};

    #[tokio::test]
    async fn test_register_validator() {
        let governance = Arc::new(GovernanceModule::new(10));
        let routes = governance_routes(governance.clone());

        let request = serde_json::json!({
            "address": "validator1",
            "locked_btcz": 1_000_000,
        });

        let response = request()
            .method("POST")
            .path("/governance/register-validator")
            .json(&request)
            .reply(&routes)
            .await;

        assert_eq!(response.status(), 200);
        assert!(response.body().contains("Validator registered successfully"));
    }

    #[tokio::test]
    async fn test_distribute_rewards() {
        let governance = Arc::new(GovernanceModule::new(10));
        let routes = governance_routes(governance.clone());

        // Register a validator
        governance.register_validator("validator1".to_string(), 1_000_000).unwrap();

        let response = request()
            .method("POST")
            .path("/governance/distribute-rewards")
            .reply(&routes)
            .await;

        assert_eq!(response.status(), 200);
        assert!(response.body().contains("Rewards distributed successfully"));
    }

    #[tokio::test]
    async fn test_slash_validator() {
        let governance = Arc::new(GovernanceModule::new(10));
        let routes = governance_routes(governance.clone());

        // Register a validator
        governance.register_validator("validator1".to_string(), 1_000_000).unwrap();

        let request = serde_json::json!({
            "address": "validator1",
            "penalty": 500_000,
        });

        let response = request()
            .method("POST")
            .path("/governance/slash-validator")
            .json(&request)
            .reply(&routes)
            .await;

        assert_eq!(response.status(), 200);
        assert!(response.body().contains("Validator slashed successfully"));

        // Verify the updated locked BTCZ
        let validators = governance.rewards.validators.lock().unwrap();
        let validator = validators.get("validator1").unwrap();
        assert_eq!(validator.locked_btcz, 500_000);
    }
}
