// File: src/tests/bridge_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use warp::test::request;
    use tokio::sync::Arc;
    use crate::bridge::{api::burn_routes, validator::ValidatorState};

    #[tokio::test]
    async fn test_burn_endpoint() {
        let validator_state = Arc::new(ValidatorState::new());
        let routes = burn_routes(validator_state.clone());

        let response = request()
            .method("POST")
            .path("/bridge/burn")
            .json(&{
                "tx_id": "tx123",
                "amount": 100,
                "timestamp": 1640995200
            })
            .reply(&routes)
            .await;

        assert_eq!(response.status(), 200);
        assert!(response.body().contains("Burn validated"));
    }

    #[tokio::test]
    async fn test_sync_event_endpoint() {
        let event_state = Arc::new(crate::bridge::event_sync::EventSyncState::new());
        let routes = crate::bridge::event_sync::event_sync_routes(event_state.clone());

        let response = request()
            .method("POST")
            .path("/bridge/sync-event")
            .json(&{
                "event_type": "burn",
                "tx_id": "tx456",
                "amount": 50,
                "merkle_root": "abc123",
                "block_height": 500
            })
            .reply(&routes)
            .await;

        assert_eq!(response.status(), 200);
        assert!(response.body().contains("Event added"));

        let events_response = request()
            .method("GET")
            .path("/bridge/events")
            .reply(&routes)
            .await;

        assert_eq!(events_response.status(), 200);
        assert!(events_response.body().contains("tx456"));
    }
}
