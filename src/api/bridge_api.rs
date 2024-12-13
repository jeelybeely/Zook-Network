use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::bridge::bridge_logic::BridgeLedger;
use crate::api::security::{with_auth, with_rate_limit, ApiKey, RateLimiter};

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
    bridge_ledger: Arc<BridgeLedger>,
    api_keys: Arc<std::collections::HashMap<String, ApiKey>>,
    rate_limiter: Arc<RateLimiter>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let mint = warp::post()
        .and(warp::path("bridge"))
        .and(warp::path("mint"))
        .and(warp::body::json())
        .and(with_auth(api_keys.clone()))
        .and(with_rate_limit(rate_limiter.clone()))
        .and(with_bridge_ledger(bridge_ledger.clone()))
        .and_then(mint_handler);

    let burn = warp::post()
        .and(warp::path("bridge"))
        .and(warp::path("burn"))
        .and(warp::body::json())
        .and(with_auth(api_keys.clone()))
        .and(with_rate_limit(rate_limiter.clone()))
        .and(with_bridge_ledger(bridge_ledger.clone()))
        .and_then(burn_handler);

    mint.or(burn)
}

fn with_bridge_ledger(
    bridge_ledger: Arc<BridgeLedger>,
) -> impl Filter<Extract = (Arc<BridgeLedger>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || bridge_ledger.clone())
}

async fn mint_handler(
    request: MintRequest,
    _auth: ApiKey,
    bridge_ledger: Arc<BridgeLedger>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match bridge_ledger.lock_btcz(request.address, request.amount) {
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
    bridge_ledger: Arc<BridgeLedger>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match bridge_ledger.burn_zbtcz(request.address, request.amount, request.transaction_hash) {
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