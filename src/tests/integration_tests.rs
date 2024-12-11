// File: tests/integration_tests.rs

use warp::http::StatusCode;
use warp::test::request;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::api::governance_api::governance_routes;
use crate::api::validator_api::validator_routes;
use crate::api::security::{ApiKey, RateLimiter};
use crate::governance::cross_layer_governance::CrossLayerGovernance;
use crate::validator::node_registration::ValidatorRegistry;
use crate::governance::validator_rewards::ValidatorRewards;
use std::time::Duration;
use tempfile::tempdir;

#[tokio::test]
async fn test_governance_submit_and_list_proposals() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("proposals.json");

    let governance = Arc::new(CrossLayerGovernance::new(
        Arc::new(crate::bridge::state_anchoring::StateAnchoring::default()),
        Arc::new(ValidatorRegistry::new(1_000_000, temp_dir.path().join("validators.json"))),
        Arc::new(crate::governance::validator_policies::GovernanceValidatorPolicies::default()),
        storage_path.clone(),
    ));

    let api_keys = Arc::new(HashMap::from([(
        "valid-api-key".to_string(),
        ApiKey {
            key: "valid-api-key".to_string(),
            permissions: vec!["submit_proposal".to_string()],
        },
    )]));

    let rate_limiter = Arc::new(RateLimiter::new(10, Duration::from_secs(60)));
    let routes = governance_routes(governance.clone(), api_keys.clone(), rate_limiter.clone());

    let resp = request()
        .method("POST")
        .path("/governance/submit")
        .header("Authorization", "valid-api-key")
        .json(&serde_json::json!({
            "description": "Increase staking rewards",
            "param": "validator_rewards",
            "value": 10,
        }))
        .reply(&routes)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);

    let resp_body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
    assert!(resp_body["success"].as_bool().unwrap());
    assert!(resp_body["proposal_id"].as_u64().is_some());

    let resp = request()
        .method("GET")
        .path("/governance/list")
        .header("Authorization", "valid-api-key")
        .reply(&routes)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);
    let resp_body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
    assert!(resp_body["success"].as_bool().unwrap());
    assert!(resp_body["proposals"].as_array().unwrap().len() > 0);
}

#[tokio::test]
async fn test_validator_register_and_compliance_logs() {
    let temp_dir = tempdir().unwrap();

    let registry = Arc::new(ValidatorRegistry::new(1_000_000, temp_dir.path().join("validators.json")));
    let rewards = Arc::new(ValidatorRewards::new(10, temp_dir.path().join("rewards.json")));

    let api_keys = Arc::new(HashMap::from([(
        "valid-api-key".to_string(),
        ApiKey {
            key: "valid-api-key".to_string(),
            permissions: vec!["register_node".to_string()],
        },
    )]));

    let rate_limiter = Arc::new(RateLimiter::new(10, Duration::from_secs(60)));
    let routes = validator_routes(registry.clone(), rewards.clone(), api_keys.clone(), rate_limiter.clone());

    let resp = request()
        .method("POST")
        .path("/validator/register")
        .header("Authorization", "valid-api-key")
        .json(&serde_json::json!({
            "address": "validator1",
            "staked_btcz": 1_500_000,
        }))
        .reply(&routes)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);

    let resp_body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
    assert!(resp_body["success"].as_bool().unwrap());

    let resp = request()
        .method("GET")
        .path("/validator/compliance-logs")
        .header("Authorization", "valid-api-key")
        .query("address=validator1")
        .reply(&routes)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);
    let resp_body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
    assert!(resp_body["success"].as_bool().unwrap());
    assert!(resp_body["logs"].as_array().unwrap().is_empty());
}
