// File: src/main.rs

use warp::Filter;
use std::sync::Arc;
use crate::api::router::api_routes;
use crate::auth::security;
use crate::validator::node_api::node_api_routes;
use crate::validator::node_registration::ValidatorRegistry;

#[tokio::main]
async fn main() {
    // Authentication token for secure routes
    let auth_token = Arc::new("secure_token".to_string());

    // Initialize Validator Registry
    let validator_registry = Arc::new(ValidatorRegistry::new());

    // Placeholder routes for governance and bridge
    let governance_routes = warp::path("governance")
        .map(|| warp::reply::json(&{"message": "Governance routes"}));

    let bridge_routes = warp::path("bridge")
        .map(|| warp::reply::json(&{"message": "Bridge routes"}));

    // Integrate validator node routes
    let validator_routes = node_api_routes(validator_registry.clone());

    // Combine all routes under /api
    let routes = api_routes(auth_token, governance_routes, validator_routes, bridge_routes)
        .with(warp::log("api"));

    // Start the server
    println!("Server running at http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
