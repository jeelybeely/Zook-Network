use crate::api::anchoring_api::AnchoringAPI;
use crate::bridge::cross_layer_sync::{AnchoredState, CrossLayerSync};
use crate::bridge::merkle::MerkleTree;
use chrono::Utc;
use serde_json::json;
use std::sync::Arc;
use warp::Filter;

#[tokio::test]
async fn test_anchor_state() {
    let cross_layer_sync = Arc::new(CrossLayerSync::new());
    let api = AnchoringAPI::new(cross_layer_sync.clone());
    let filter = api.routes();

    let merkle_tree = MerkleTree::new(vec!["tx1".into(), "tx2".into()]);
    let merkle_proof = merkle_tree.get_proof("tx1");

    let anchored_state = AnchoredState {
        block_height: 1,
        state_root: merkle_tree.get_root(),
        merkle_proof: merkle_proof.clone(),
        timestamp: Utc::now(),
        validator_compliance: true,
    };

    let response = warp::test::request()
        .method("POST")
        .path("/anchoring/anchor")
        .json(&anchored_state)
        .reply(&filter)
        .await;

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
    assert!(body["success"].as_bool().unwrap());
    assert_eq!(body["message"], "State anchored successfully");
}

#[tokio::test]
async fn test_get_latest_state() {
    let cross_layer_sync = Arc::new(CrossLayerSync::new());
    let api = AnchoringAPI::new(cross_layer_sync.clone());
    let filter = api.routes();

    let merkle_tree = MerkleTree::new(vec!["tx1".into(), "tx2".into()]);
    let merkle_proof = merkle_tree.get_proof("tx1");

    let anchored_state = AnchoredState {
        block_height: 1,
        state_root: merkle_tree.get_root(),
        merkle_proof: merkle_proof.clone(),
        timestamp: Utc::now(),
        validator_compliance: true,
    };

    cross_layer_sync
        .anchor_state(anchored_state.clone(), true, merkle_proof.clone())
        .unwrap();

    let response = warp::test::request()
        .method("GET")
        .path("/anchoring/latest")
        .reply(&filter)
        .await;

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
    assert!(body["success"].as_bool().unwrap());
    assert_eq!(body["state"]["state_root"], anchored_state.state_root);
}
