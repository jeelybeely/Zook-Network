use crate::bridge::btcz_integration::{BTCZIntegration, BTCZAnchorPayload};
use crate::bridge::cross_layer_sync::{AnchoredState, CrossLayerSync};
use std::sync::Arc;
use chrono::Utc;
use warp::Filter;

#[tokio::test]
async fn test_btcz_sync_success() {
    // Mock BTCZ integration endpoint
    let btcz_endpoint = warp::path("btcz-anchor")
        .and(warp::post())
        .map(|| warp::reply::with_status("Success", warp::http::StatusCode::OK));

    let (addr, server) = warp::serve(btcz_endpoint).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let btcz_integration = Arc::new(BTCZIntegration::new(format!("http://{}", addr)));

    // Payload to anchor
    let payload = BTCZAnchorPayload {
        block_height: 1,
        state_root: "mock_state_root".to_string(),
        merkle_proof: vec!["mock_proof".to_string()],
        validator_compliance: true,
    };

    // Perform synchronization
    let result = btcz_integration.send_anchor(payload).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_btcz_sync_failure() {
    // Mock BTCZ integration endpoint with failure
    let btcz_endpoint = warp::path("btcz-anchor")
        .and(warp::post())
        .map(|| warp::reply::with_status("Failure", warp::http::StatusCode::INTERNAL_SERVER_ERROR));

    let (addr, server) = warp::serve(btcz_endpoint).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let btcz_integration = Arc::new(BTCZIntegration::new(format!("http://{}", addr)));

    // Payload to anchor
    let payload = BTCZAnchorPayload {
        block_height: 1,
        state_root: "mock_state_root".to_string(),
        merkle_proof: vec!["mock_proof".to_string()],
        validator_compliance: true,
    };

    // Perform synchronization
    let result = btcz_integration.send_anchor(payload).await;
    assert!(result.is_err());
}
