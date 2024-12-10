// File: src/bridge/api.rs

use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::validator::{BurnRecord, ValidatorState};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BurnRequest {
    tx_id: String,
    amount: u64,
    timestamp: u64,
}

pub fn burn_routes(validator_state: Arc<ValidatorState>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let validate_burn = warp::path("bridge")
        .and(warp::path("burn"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |burn_request: BurnRequest| {
            let state = validator_state.clone();
            async move {
                let record = BurnRecord {
                    tx_id: burn_request.tx_id,
                    amount: burn_request.amount,
                    timestamp: burn_request.timestamp,
                };

                match state.validate_burn(&record) {
                    Ok(_) => Ok(warp::reply::json(&{"status": "success", "message": "Burn validated"})),
                    Err(e) => Ok(warp::reply::json(&{"status": "error", "message": e})),
                }
            }
        });

    let get_processed_burns = warp::path("bridge")
        .and(warp::path("processed-burns"))
        .and(warp::get())
        .map(move || {
            let state = validator_state.clone();
            let burns = state.get_processed_burns();
            warp::reply::json(&burns)
        });

    validate_burn.or(get_processed_burns)
}
