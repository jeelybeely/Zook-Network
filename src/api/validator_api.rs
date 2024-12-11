// File: src/api/validator_api.rs

use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;
use crate::validator::node_registration::ValidatorRegistry;
use crate::governance::validator_rewards::{ValidatorRewards, ComplianceLog};
use crate::api::security::{with_auth, with_rate_limit, ApiKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterNodeRequest {
    pub address: String,
    pub staked_btcz: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterNodeResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceLogsResponse {
    pub success: bool,
    pub logs: Vec<ComplianceLog>,
    pub message: String,
}

pub fn validator_routes(
    registry: Arc<ValidatorRegistry>,
    rewards: Arc<ValidatorRewards>,
    api_keys: Arc<HashMap<String, ApiKey>>,
    rate_limiter: Arc<crate::api::security::RateLimiter>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let register_node = warp::post()
        .and(warp::path("validator"))
        .and(warp::path("register"))
        .and(warp::body::json())
        .and(with_auth(api_keys.clone()))
        .and(with_rate_limit(rate_limiter.clone()))
        .and(with_registry(registry.clone()))
        .and_then(register_node_handler);

    let get_compliance_logs = warp::get()
        .and(warp::path("validator"))
        .and(warp::path("compliance-logs"))
        .and(warp::query::<String>())
        .and(with_auth(api_keys.clone()))
        .and(with_rate_limit(rate_limiter.clone()))
        .and(with_rewards(rewards.clone()))
        .and_then(get_compliance_logs_handler);

    register_node.or(get_compliance_logs)
}

fn with_registry(
    registry: Arc<ValidatorRegistry>,
) -> impl Filter<Extract = (Arc<ValidatorRegistry>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || registry.clone())
}

fn with_rewards(
    rewards: Arc<ValidatorRewards>,
) -> impl Filter<Extract = (Arc<ValidatorRewards>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || rewards.clone())
}

async fn register_node_handler(
    request: RegisterNodeRequest,
    _auth: ApiKey,
    registry: Arc<ValidatorRegistry>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match registry.register_node(request.address.clone(), request.staked_btcz) {
        Ok(_) => Ok(warp::reply::json(&RegisterNodeResponse {
            success: true,
            message: "Validator node registered successfully".to_string(),
        })),
        Err(err) => Ok(warp::reply::json(&RegisterNodeResponse {
            success: false,
            message: err,
        })),
    }
}

async fn get_compliance_logs_handler(
    address: String,
    _auth: ApiKey,
    rewards: Arc<ValidatorRewards>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match rewards.get_compliance_logs(&address) {
        Ok(logs) => Ok(warp::reply::json(&ComplianceLogsResponse {
            success: true,
            logs,
            message: "Compliance logs retrieved successfully".to_string(),
        })),
        Err(err) => Ok(warp::reply::json(&ComplianceLogsResponse {
            success: false,
            logs: Vec::new(),
            message: err,
        })),
    }
}
