// File: src/governance/api.rs

use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use super::validator_rewards::{ValidatorRewards};
use super::GovernanceModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RegisterValidatorRequest {
    address: String,
    locked_btcz: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SlashValidatorRequest {
    address: String,
    penalty: u64,
}

pub fn governance_routes(governance: Arc<GovernanceModule>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let register_validator = warp::path("governance")
        .and(warp::path("register-validator"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |request: RegisterValidatorRequest| {
            let governance = governance.clone();
            async move {
                match governance.register_validator(request.address, request.locked_btcz) {
                    Ok(_) => Ok(warp::reply::json(&{"status": "success", "message": "Validator registered successfully"})),
                    Err(e) => Ok(warp::reply::json(&{"status": "error", "message": e})),
                }
            }
        });

    let distribute_rewards = warp::path("governance")
        .and(warp::path("distribute-rewards"))
        .and(warp::post())
        .and_then(move || {
            let governance = governance.clone();
            async move {
                match governance.distribute_rewards() {
                    Ok(_) => Ok(warp::reply::json(&{"status": "success", "message": "Rewards distributed successfully"})),
                    Err(e) => Ok(warp::reply::json(&{"status": "error", "message": e})),
                }
            }
        });

    let slash_validator = warp::path("governance")
        .and(warp::path("slash-validator"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |request: SlashValidatorRequest| {
            let governance = governance.clone();
            async move {
                match governance.slash_validator(request.address, request.penalty) {
                    Ok(_) => Ok(warp::reply::json(&{"status": "success", "message": "Validator slashed successfully"})),
                    Err(e) => Ok(warp::reply::json(&{"status": "error", "message": e})),
                }
            }
        });

    register_validator.or(distribute_rewards).or(slash_validator)
}
