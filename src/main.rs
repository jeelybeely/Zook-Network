// File: src/main.rs

use warp::Filter;
use std::sync::Arc;
use crate::api::router::api_routes;
use crate::auth::security;
use crate::validator::node_api::node_api_routes;
use crate::validator::node_registration::ValidatorRegistry;
use crate::governance::validator_policy_api::policy_api_routes;
use crate::governance::validator_policies::GovernanceValidatorPolicies;
use crate::bridge::bridge_api::bridge_api_routes;
use crate::bridge::bridge_operations::BridgeOperations;
use crate::bridge::merkle::MerkleTree;

#[tokio::main]
async fn main() {
    // Authentication token for secure routes
    let auth_token = Arc::new("secure_token".to_string());

    // Initialize Validator Registry
    let validator_registry = Arc::new(ValidatorRegistry::new());

    // Initialize Governance Validator Policies
    let governance_policies = Arc::new(GovernanceValidatorPolicies::new(1000, 80));

    // Initialize Bridge Operations
    let bridge_operations = Arc::new(BridgeOperations::new());
    let merkle_tree = Arc::new(MerkleTree::new());

    // Governance Routes
    let governance_routes = policy_api_routes(governance_policies.clone(), validator_registry.clone());

    // Validator Routes
    let validator_routes = node_api_routes(validator_registry.clone());

    // Bridge Routes
    let bridge_routes = bridge_api_routes(bridge_operations.clone(), merkle_tree.clone());

    // Combine all routes under /api
    let routes = api_routes(auth_token, governance_routes, validator_routes, bridge_routes)
        .with(warp::log("api"));

    // Start the server
    println!("Server running at http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
