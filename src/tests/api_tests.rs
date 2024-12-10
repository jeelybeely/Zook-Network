// File: src/tests/api_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use warp::test::request;
    use tokio::sync::Arc;
    use crate::clarity::interaction::ClarityInteractor;
    use crate::api::{governance_api::GovernanceAPI, bridge::BridgeAPI};

    #[tokio::test]
    async fn test_stake_btcz() {
        let clarity = Arc::new(ClarityInteractor::new("http://localhost:20443", &"ST123".into()));
        let governance_api = GovernanceAPI::new(clarity.clone());

        let response = request()
            .method("POST")
            .path("/governance/stake")
            .json(&1000u128)
            .reply(&governance_api.routes())
            .await;

        assert_eq!(response.status(), 200);
        assert!(response.body().contains("Staking successful"));
    }

    #[tokio::test]
    async fn test_lock_btcz() {
        let clarity = Arc::new(ClarityInteractor::new("http://localhost:20443", &"ST123".into()));
        let bridge_api = BridgeAPI::new(clarity.clone());

        let response = request()
            .method("POST")
            .path("/bridge/lock")
            .json(&(12345u128, 1000u128))
            .reply(&bridge_api.routes())
            .await;

        assert_eq!(response.status(), 200);
        assert!(response.body().contains("BTCZ locked"));
    }
}
