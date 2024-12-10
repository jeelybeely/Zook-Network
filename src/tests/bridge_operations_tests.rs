// File: src/tests/bridge_operations_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use warp::test::request;
    use tokio::sync::Arc;
    use crate::bridge::{api::burn_routes, bridge_finalization::{LockRequest, BurnRequest}, BridgeModule};
    use crate::bridge::validator::ValidatorState;

    #[tokio::test]
    async fn test_lock_btcz() {
        let validator_state = Arc::new(ValidatorState::new());
        let bridge_module = Arc::new(BridgeModule::new(vec![], validator_state.clone()));
        let routes = burn_routes(bridge_module.clone());

        let request = LockRequest {
            tx_id: "tx-lock123".to_string(),
            amount: 100,
            from_address: "user1".to_string(),
            to_address: "bridge1".to_string(),
        };

        let response = request()
            .method("POST")
            .path("/bridge/lock")
            .json(&request)
            .reply(&routes)
            .await;

        assert_eq!(response.status(), 200);
        assert!(response.body().contains("BTCZ locked successfully"));
    }

    #[tokio::test]
    async fn test_burn_zbtcz() {
        let validator_state = Arc::new(ValidatorState::new());
        let bridge_module = Arc::new(BridgeModule::new(vec![], validator_state.clone()));
        let routes = burn_routes(bridge_module.clone());

        let request = BurnRequest {
            tx_id: "tx-burn123".to_string(),
            amount: 50,
            from_address: "user1".to_string(),
            to_address: "bridge1".to_string(),
        };

        let response = request()
            .method("POST")
            .path("/bridge/burn")
            .json(&request)
            .reply(&routes)
            .await;

        assert_eq!(response.status(), 200);
        assert!(response.body().contains("zBTCZ burned successfully"));
    }

    #[tokio::test]
    async fn test_unlock_btcz() {
        let validator_state = Arc::new(ValidatorState::new());
        let bridge_module = Arc::new(BridgeModule::new(vec![], validator_state.clone()));
        let routes = burn_routes(bridge_module.clone());

        // Lock BTCZ to prepare for unlock test
        let lock_request = LockRequest {
            tx_id: "tx-unlock123".to_string(),
            amount: 100,
            from_address: "user1".to_string(),
            to_address: "bridge1".to_string(),
        };
        bridge_module.lock_btc(lock_request.clone()).unwrap();

        let response = request()
            .method("POST")
            .path("/bridge/unlock")
            .json(&"tx-unlock123")
            .reply(&routes)
            .await;

        assert_eq!(response.status(), 200);
        assert!(response.body().contains("BTCZ unlocked successfully"));
    }

    #[tokio::test]
    async fn test_mint_zbtcz() {
        let validator_state = Arc::new(ValidatorState::new());
        let bridge_module = Arc::new(BridgeModule::new(vec![], validator_state.clone()));
        let routes = burn_routes(bridge_module.clone());

        // Burn zBTCZ to prepare for mint test
        let burn_request = BurnRequest {
            tx_id: "tx-mint123".to_string(),
            amount: 50,
            from_address: "user1".to_string(),
            to_address: "bridge1".to_string(),
        };
        bridge_module.burn_zbtcz(burn_request.clone()).unwrap();

        let response = request()
            .method("POST")
            .path("/bridge/mint")
            .json(&"tx-mint123")
            .reply(&routes)
            .await;

        assert_eq!(response.status(), 200);
        assert!(response.body().contains("zBTCZ minted successfully"));
    }
}
