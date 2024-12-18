use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use warp::Filter;

pub mod models;
pub mod ledger;
pub mod errors;

mod api;
mod bridge;
mod clarity;
mod governance;
mod validator;

#[path = "../interaction.rs"]
mod interaction;

use api::anchoring_api::AnchoringAPI;
use api::bridge_api::{BridgeAPI, MutexAdapter};
use api::governance_api::GovernanceAPI;
use api::validator_rewards_api::ValidatorRewardsAPI;
use bridge::bridge_logic::BridgeLedger;
use bridge::cross_layer_sync::CrossLayerSync;
use bridge::btcz_integration::BTCZIntegration;
use bridge::state_anchoring::StateAnchoring;
use clarity::ClarityInteractor;
use governance::validator_rewards_tokenomics::ValidatorRewards;
use governance::cross_layer_governance::CrossLayerGovernance;
use governance::validator_policies::GovernanceValidatorPolicies;
use api::security::RateLimiter;
use validator::node_registration::ValidatorRegistry;

#[tokio::main]
async fn main() {
    let api_url = "http://clarity_node_rpc_url".to_string();
    let sender = "SP3K2K92ZXAN5DZ96FEAB5A6NNG2MX74E9KPW0XJ7";
    let zbtcz_address = "SP3K2K92ZXAN5DZ96FEAB5A6NNG2MX74E9KPW0XJ7.zbtcz";
    let gbtcz_address = "SP3K2K92ZXAN5DZ96FEAB5A6NNG2MX74E9KPW0XJ7.gbtcz";
    let governance_address = "SP3K2K92ZXAN5DZ96FEAB5A6NNG2MX74E9KPW0XJ7.governance";

    let clarity = Arc::new(ClarityInteractor::new(
        &api_url,
        interaction::Principal::new(sender.to_string()),
        zbtcz_address,
        gbtcz_address,
        governance_address,
    ));

    let state_anchoring = Arc::new(StateAnchoring::new());
    let validator_registry = Arc::new(ValidatorRegistry::new(
        1_000_000,
        PathBuf::from("validator_registry.json"),
    ));

    let governance_policies = Arc::new(GovernanceValidatorPolicies::new(1_000_000, 80, 70));
    let btcz_integration = Arc::new(BTCZIntegration::new("http://btcz_node_rpc_url".to_string()));
    let cross_layer_sync = Arc::new(CrossLayerSync::new(btcz_integration.clone()));
    let bridge_ledger = MutexAdapter::new_tokio(BridgeLedger::new(
        state_anchoring.clone(),
        clarity.clone(),
        btcz_integration.clone(),
    ));

    let validator_rewards = Arc::new(std::sync::Mutex::new(ValidatorRewards::new(
        100,
        PathBuf::from("validator_rewards.json"),
    )));

    let governance_storage_path = PathBuf::from("governance_proposals.json");
    let cross_layer_governance = Arc::new(CrossLayerGovernance::new(
        state_anchoring.clone(),
        validator_registry.clone(),
        governance_policies.clone(),
        validator_rewards.clone(),
        governance_storage_path,
    ));

    let api_keys = Arc::new(HashMap::new());
    let rate_limiter = Arc::new(RateLimiter::new(100, std::time::Duration::from_secs(60)));

    let governance_api = GovernanceAPI::new(
        cross_layer_governance.clone(),
        api_keys.clone(),
        rate_limiter.clone(),
    );
    let bridge_api = BridgeAPI::new(HashSet::new(), bridge_ledger);
    let validator_rewards_api = ValidatorRewardsAPI::new(clarity.clone());
    let anchoring_api = AnchoringAPI::new(cross_layer_sync.clone());

    let routes = governance_api
        .routes()
        .or(bridge_api.routes())
        .or(validator_rewards_api.routes())
        .or(anchoring_api.routes());

    println!("Starting server at http://127.0.0.1:3030");
    warp::serve(routes.with(warp::log("zook_api")))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
