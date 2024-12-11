use crate::api::{bridge_api::BridgeAPI, governance_api::GovernanceAPI, validator_rewards_api::ValidatorRewardsAPI};
use crate::clarity::mock_clarity_interactor::MockClarityInteractor;
use warp::Filter;
use serde_json::json;

#[tokio::test]
async fn test_governance_workflow() {
    let clarity = MockClarityInteractor::new();
    let governance_api = GovernanceAPI::new(clarity.clone().into());
    let filter = governance_api.routes();

    // Submit a proposal
    let proposal = json!({
        "proposal_id": 1,
        "description": "Increase minimum stake",
        "changes": [
            ["minimum-stake", "200000"]
        ]
    });

    let response = warp::test::request()
        .method("POST")
        .path("/governance/submit-proposal")
        .json(&proposal)
        .reply(&filter)
        .await;

    assert_eq!(response.status(), 200);

    // Vote on the proposal
    let response = warp::test::request()
        .method("POST")
        .path("/governance/vote")
        .json(&1)
        .reply(&filter)
        .await;

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_bridge_operations() {
    let clarity = MockClarityInteractor::new();
    let bridge_api = BridgeAPI::new(clarity.clone().into());
    let filter = bridge_api.routes();

    // Lock tokens
    let lock_request = json!({
        "amount": 1000,
        "sender_address": "sender1"
    });

    let response = warp::test::request()
        .method("POST")
        .path("/bridge/lock")
        .json(&lock_request)
        .reply(&filter)
        .await;

    assert_eq!(response.status(), 200);

    // Burn tokens
    let burn_request = json!({
        "amount": 500,
        "sender_address": "sender1"
    });

    let response = warp::test::request()
        .method("POST")
        .path("/bridge/burn")
        .json(&burn_request)
        .reply(&filter)
        .await;

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_validator_rewards_distribution() {
    let clarity = MockClarityInteractor::new();
    let rewards_api = ValidatorRewardsAPI::new(clarity.into());
    let filter = rewards_api.routes();

    // Distribute rewards
    let response = warp::test::request()
        .method("POST")
        .path("/validator-rewards/distribute")
        .reply(&filter)
        .await;

    assert_eq!(response.status(), 200);

    // Query rewards
    let response = warp::test::request()
        .method("GET")
        .path("/validator-rewards/query")
        .reply(&filter)
        .await;

    assert_eq!(response.status(), 200);
}
