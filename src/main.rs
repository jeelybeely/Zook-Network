// File: src/main.rs

use warp::Filter;
use std::sync::Arc;

mod bridge;
mod governance;
mod api;

use bridge::validator::ValidatorState;
use bridge::BridgeModule;
use api::burn_routes;

#[tokio::main]
async fn main() {
    // Initialize Validator State
    let validator_state = Arc::new(ValidatorState::new());

    // Initialize Bridge Module
    let bridge_module = BridgeModule::new(vec![], validator_state.clone());

    // Setup Warp Filters for APIs
    let api_routes = burn_routes(validator_state.clone());

    // Start the Warp Server
    println!("Starting Zook Network Bridge API on http://localhost:3030");
    warp::serve(api_routes).run(([127, 0, 0, 1], 3030)).await;
}