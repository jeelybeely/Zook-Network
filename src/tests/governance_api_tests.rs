use crate::api::governance_api::GovernanceAPI;
use crate::clarity::mock_clarity_interactor::MockClarityInteractor;
use warp::Filter;
use serde_json::json;

#[tokio::test]
async fn test_submit_proposal() {
    let clarity = MockClarityInteractor::new();
    let api = GovernanceAPI::new(clarity.into());
    let filter = api.routes();

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
    let response_body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
    assert!(response_body["success"].as_bool().unwrap());
    assert_eq!(response_body["message"], "Proposal 1 submitted");
}

#[tokio::test]
async fn test_vote_on_proposal() {
    let clarity = MockClarityInteractor::new();
    let api = GovernanceAPI::new(clarity.into());
    let filter = api.routes();

    let response = warp::test::request()
        .method("POST")
        .path("/governance/vote")
        .json(&1)
        .reply(&filter)
        .await;

    assert_eq!(response.status(), 200);
    let response_body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
    assert!(response_body["success"].as_bool().unwrap());
    assert_eq!(response_body["message"], "Voted on proposal 1");
}
