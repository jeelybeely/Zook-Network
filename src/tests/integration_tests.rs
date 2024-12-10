// File: src/tests/integration_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use warp::test::request;
    use tokio::sync::Arc;
    use crate::bridge::{api::burn_routes, validator::ValidatorState, event_sync::event_sync_routes};
    use crate::governance::{api::governance_routes, governance_scalability::GovernanceState};

    #[tokio::test]
    async fn test_cross_layer_burn_sync() {
        let validator_state = Arc::new(ValidatorState::new());
        let bridge_routes = burn_routes(validator_state.clone())
            .or(event_sync_routes(validator_state.clone()));

        // Simulate a burn transaction
        let burn_response = request()
            .method("POST")
            .path("/bridge/burn")
            .json(&{
                "tx_id": "burn123",
                "amount": 100,
                "timestamp": 1640995200
            })
            .reply(&bridge_routes)
            .await;

        assert_eq!(burn_response.status(), 200);
        assert!(burn_response.body().contains("Burn validated"));

        // Sync the burn event
        let sync_response = request()
            .method("POST")
            .path("/bridge/sync-event")
            .json(&{
                "event_type": "burn",
                "tx_id": "burn123",
                "amount": 100,
                "merkle_root": "abc123",
                "block_height": 500
            })
            .reply(&bridge_routes)
            .await;

        assert_eq!(sync_response.status(), 200);
        assert!(sync_response.body().contains("Event added"));
    }

    #[tokio::test]
    async fn test_governance_proposal_and_execution() {
        let governance_state = Arc::new(GovernanceState::new());
        let governance_routes = governance_routes(governance_state.clone());

        // Create a proposal
        let proposal_response = request()
            .method("POST")
            .path("/governance/propose")
            .json(&{
                "creator": "user1",
                "description": "Increase validator rewards",
                "param": "reward-rate",
                "value": 200
            })
            .reply(&governance_routes)
            .await;

        assert_eq!(proposal_response.status(), 200);
        assert!(proposal_response.body().contains("proposal_id"));

        // Vote on the proposal
        let vote_response = request()
            .method("POST")
            .path("/governance/vote")
            .json(&{
                "proposal_id": 1,
                "support": true,
                "voting_power": 50
            })
            .reply(&governance_routes)
            .await;

        assert_eq!(vote_response.status(), 200);
        assert!(vote_response.body().contains("Vote recorded"));

        // Execute the proposal
        let execute_response = request()
            .method("POST")
            .path("/governance/execute")
            .json(&{
                "proposal_id": 1
            })
            .reply(&governance_routes)
            .await;

        assert_eq!(execute_response.status(), 200);
        assert!(execute_response.body().contains("Proposal executed"));
    }
}
