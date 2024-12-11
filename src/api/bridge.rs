// File: src/api/bridge.rs

use warp::{Filter, Rejection, Reply};
use std::sync::Arc;
use crate::bridge::bridge_operations::{BridgeOperations, LockTransaction, BurnTransaction};
use crate::bridge::merkle::MerkleTree;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LockRequest {
    tx_id: String,
    amount: u64,
    sender: String,
    receiver: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BurnRequest {
    tx_id: String,
    amount: u64,
    sender: String,
    receiver: String,
}

pub fn bridge_api_routes(
    bridge_operations: Arc<BridgeOperations>,
    merkle_tree: Arc<MerkleTree>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let lock_tokens = warp::path("lock")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |request: LockRequest| {
            let bridge_operations = bridge_operations.clone();
            async move {
                let tx = LockTransaction {
                    tx_id: request.tx_id,
                    amount: request.amount,
                    timestamp: chrono::Utc::now(),
                    sender: request.sender,
                    receiver: request.receiver,
                };
                match bridge_operations.lock_tokens(tx) {
                    Ok(_) => Ok(warp::reply::json(&{"status": "success", "message": "Tokens locked successfully"})),
                    Err(e) => Ok(warp::reply::json(&{"status": "error", "message": e})),
                }
            }
        });

    let burn_tokens = warp::path("burn")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |request: BurnRequest| {
            let bridge_operations = bridge_operations.clone();
            async move {
                let tx = BurnTransaction {
                    tx_id: request.tx_id,
                    amount: request.amount,
                    timestamp: chrono::Utc::now(),
                    sender: request.sender,
                    receiver: request.receiver,
                };
                match bridge_operations.burn_tokens(tx) {
                    Ok(_) => Ok(warp::reply::json(&{"status": "success", "message": "Tokens burned successfully"})),
                    Err(e) => Ok(warp::reply::json(&{"status": "error", "message": e})),
                }
            }
        });

    let validate_lock = warp::path("validate-lock")
        .and(warp::get())
        .and(warp::query::<String>())
        .and_then(move |tx_id: String| {
            let bridge_operations = bridge_operations.clone();
            let merkle_tree = merkle_tree.clone();
            async move {
                let valid = bridge_operations.validate_lock_proof(&merkle_tree, &tx_id);
                Ok::<_, Rejection>(warp::reply::json(&{"status": if valid {"success"} else {"error"}, "valid": valid}))
            }
        });

    let validate_burn = warp::path("validate-burn")
        .and(warp::get())
        .and(warp::query::<String>())
        .and_then(move |tx_id: String| {
            let bridge_operations = bridge_operations.clone();
            let merkle_tree = merkle_tree.clone();
            async move {
                let valid = bridge_operations.validate_burn_proof(&merkle_tree, &tx_id);
                Ok::<_, Rejection>(warp::reply::json(&{"status": if valid {"success"} else {"error"}, "valid": valid}))
            }
        });

    warp::path("bridge").and(lock_tokens.or(burn_tokens).or(validate_lock).or(validate_burn))
}
