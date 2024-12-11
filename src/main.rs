use std::sync::{Arc, Mutex};
use warp::Filter;

mod api;
mod bridge;
mod clarity;
mod governance;
mod validator;

use api::anchoring_api::AnchoringAPI;
use api::bridge_api::BridgeAPI;
use api::governance_api::GovernanceAPI;
use api::validator_rewards_api::ValidatorRewardsAPI;
use bridge::bridge_logic::BridgeLedger;
use bridge::cross_layer_sync::CrossLayerSync;
use clarity::interaction::ClarityInteractor;
use validator::node_registration::ValidatorRegistry;
use governance::validator_policies::GovernanceValidatorPolicies;

#[tokio::main]
async fn main() {
    // Initialize components
    let clarity = Arc::new(ClarityInteractor::new());
    let validator_registry = Arc::new(ValidatorRegistry::new());
    let governance_policies = Arc::new(GovernanceValidatorPolicies::new());
    let cross_layer_sync = Arc::new(CrossLayerSync::new());
    let bridge_ledger = Arc::new(Mutex::new(BridgeLedger::new()));

    // Initialize APIs
    let governance_api = GovernanceAPI::new(clarity.clone());
    let bridge_api = BridgeAPI::new(bridge_ledger.clone());
    let validator_rewards_api = ValidatorRewardsAPI::new(clarity.clone());
    let anchoring_api = AnchoringAPI::new(cross_layer_sync.clone());

    // Combine routes
    let routes = governance_api
        .routes()
        .or(bridge_api.routes())
        .or(validator_rewards_api.routes())
        .or(anchoring_api.routes());

    // Start server
    println!("Starting server at http://127.0.0.1:3030");
    warp::serve(routes.with(warp::log("zook_api")))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
