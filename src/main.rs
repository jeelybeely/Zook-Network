// File: src/main.rs

use warp::Filter;
use std::sync::Arc;

mod bridge;
mod governance;

use bridge::validator::ValidatorState;
use bridge::api::burn_routes;
use bridge::event_sync::event_sync_routes;
use governance::api::governance_routes;
use governance::governance_scalability::GovernanceState;

#[tokio::main]
async fn main() {
    // Initialize Validator State
    let validator_state = Arc::new(ValidatorState::new());

    // Initialize Governance State
    let governance_state = Arc::new(GovernanceState::new());

    // Setup Warp Filters for APIs
    let bridge_api = burn_routes(validator_state.clone())
        .or(event_sync_routes(validator_state.clone()));
    let governance_api = governance_routes(governance_state.clone());

    let api_routes = bridge_api.or(governance_api);

    // Start the Warp Server
    println!("Starting Zook Network API on http://localhost:3030");
    warp::serve(api_routes).run(([127, 0, 0, 1], 3030)).await;
}
