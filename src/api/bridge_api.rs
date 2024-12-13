// File: src/api/bridge_api.rs

use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::bridge::bridge_logic::BridgeLedger;
use crate::api::security::{with_auth, with_rate_limit, ApiKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintRequest {
    pub address: String,
    pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnRequest {
    pub address: String,
    pub amount: u64,
    pub transaction_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeResponse {
    pub success: bool,
    pub message: String,
}

pub fn bridge_routes(
    bridge_ledger: Arc<Mutex<BridgeLedger>>,
    api_keys: Arc<std::collections::HashMap<String, ApiKey>>,
    rate_limiter: Arc<crate::api::security::RateLimiter>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let mint = warp::post()
        .and(warp::path("bridge"))
        .and(warp::path("mint"))
        .and(warp::body::json())
        .and(with_auth(api_keys.clone()))
        .and(with_rate_limit(rate_limiter.clone()))
        .and(with_bridge_ledger(bridge_ledger.clone()))
        .and_then(|request: MintRequest, auth: ApiKey, ledger: Arc<Mutex<BridgeLedger>>| {
            async move { mint_handler(request, auth, ledger).await }
        });

    let burn = warp::post()
        .and(warp::path("bridge"))
        .and(warp::path("burn"))
        .and(warp::body::json())
        .and(with_auth(api_keys.clone()))
        .and(with_rate_limit(rate_limiter.clone()))
        .and(with_bridge_ledger(bridge_ledger.clone()))
        .and_then(|request: BurnRequest, auth: ApiKey, ledger: Arc<Mutex<BridgeLedger>>| {
            async move { burn_handler(request, auth, ledger).await }
        });

    mint.or(burn)
}

fn with_bridge_ledger(
    bridge_ledger: Arc<Mutex<BridgeLedger>>,
) -> impl Filter<Extract = (Arc<Mutex<BridgeLedger>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || bridge_ledger.clone())
}

async fn mint_handler(
    request: MintRequest,
    _auth: ApiKey,
    bridge_ledger: Arc<Mutex<BridgeLedger>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut ledger = bridge_ledger.lock().map_err(|_| warp::reject::reject())?;
    match ledger.lock_btcz(request.address, request.amount) {
        Ok(_) => Ok(warp::reply::json(&BridgeResponse {
            success: true,
            message: "zBTCZ minted successfully".to_string(),
        })),
        Err(err) => Ok(warp::reply::json(&BridgeResponse {
            success: false,
            message: err,
        })),
    }
}

async fn burn_handler(
    request: BurnRequest,
    _auth: ApiKey,
    bridge_ledger: Arc<Mutex<BridgeLedger>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut ledger = bridge_ledger.lock().map_err(|_| warp::reject::reject())?;
    match ledger.burn_zbtcz(request.address, request.amount, request.transaction_hash) {
        Ok(_) => Ok(warp::reply::json(&BridgeResponse {
            success: true,
            message: "zBTCZ burned successfully".to_string(),
        })),
        Err(err) => Ok(warp::reply::json(&BridgeResponse {
            success: false,
            message: err,
        })),
    }
}
