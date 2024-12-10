// File: src/governance/api.rs

use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use super::validator_rewards_tokenomics::ValidatorRewardsTokenomics;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RewardAdjustmentRequest {
    param: String,
    value: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ValidatorQueryRequest {
    address: String,
}

pub fn validator_rewards_routes(tokenomics: Arc<ValidatorRewardsTokenomics>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let distribute_rewards = warp::path("validator-rewards")
        .and(warp::path("distribute"))
        .and(warp::post())
        .and_then(move || {
            let tokenomics = tokenomics.clone();
            async move {
                let response = tokenomics.distribute_rewards_api();
                Ok::<_, Rejection>(warp::reply::json(&{"status": "success", "message": response}))
            }
        });

    let apply_adjustment = warp::path("validator-rewards")
        .and(warp::path("adjust"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |request: RewardAdjustmentRequest| {
            let mut tokenomics = tokenomics.clone();
            async move {
                let response = tokenomics.apply_adjustment_api(&request.param, request.value);
                Ok::<_, Rejection>(warp::reply::json(&{"status": "success", "message": response}))
            }
        });

    let query_rewards = warp::path("validator-rewards")
        .and(warp::path("query"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |request: ValidatorQueryRequest| {
            let tokenomics = tokenomics.clone();
            async move {
                let response = tokenomics.get_rewards_api(&request.address);
                Ok::<_, Rejection>(warp::reply::json(&{"status": "success", "message": response}))
            }
        });

    distribute_rewards.or(apply_adjustment).or(query_rewards)
}
