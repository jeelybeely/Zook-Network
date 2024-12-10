// File: src/main.rs

use warp::Filter;
use std::sync::Arc;
use crate::api::router::api_routes;
use crate::auth::security;

#[tokio::main]
async fn main() {
    // Authentication token for secure routes
    let auth_token = Arc::new("secure_token".to_string());

    // Placeholder routes for governance, validator, and bridge
    let governance_routes = warp::path("governance")
        .map(|| warp::reply::json(&{"message": "Governance routes"}));

    let validator_routes = warp::path("validator")
        .map(|| warp::reply::json(&{"message": "Validator routes"}));

    let bridge_routes = warp::path("bridge")
        .map(|| warp::reply::json(&{"message": "Bridge routes"}));

    // Combine all routes under /api
    let routes = api_routes(auth_token, governance_routes, validator_routes, bridge_routes)
        .with(warp::log("api"));

    // Start the server
    println!("Server running at http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
