// File: src/api/governance_api.rs

use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;
use crate::governance::cross_layer_governance::{CrossLayerGovernance, GovernanceProposal};
use crate::api::security::{with_auth, with_rate_limit, ApiKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalRequest {
    pub description: String,
    pub param: String,
    pub value: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalResponse {
    pub success: bool,
    pub proposal_id: Option<u64>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalsListResponse {
    pub success: bool,
    pub proposals: Vec<GovernanceProposal>,
}

pub fn governance_routes(
    governance: Arc<CrossLayerGovernance>,
    api_keys: Arc<HashMap<String, ApiKey>>,
    rate_limiter: Arc<crate::api::security::RateLimiter>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let submit_proposal = warp::post()
        .and(warp::path("governance"))
        .and(warp::path("submit"))
        .and(warp::body::json())
        .and(with_auth(api_keys.clone()))
        .and(with_rate_limit(rate_limiter.clone()))
        .and(with_governance(governance.clone()))
        .and_then(submit_proposal_handler);

    let list_proposals = warp::get()
        .and(warp::path("governance"))
        .and(warp::path("list"))
        .and(with_rate_limit(rate_limiter.clone()))
        .and(with_governance(governance.clone()))
        .and_then(list_proposals_handler);

    submit_proposal.or(list_proposals)
}

fn with_governance(
    governance: Arc<CrossLayerGovernance>,
) -> impl Filter<Extract = (Arc<CrossLayerGovernance>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || governance.clone())
}

async fn submit_proposal_handler(
    request: ProposalRequest,
    _auth: ApiKey,
    governance: Arc<CrossLayerGovernance>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match governance.submit_proposal(request.description, request.param, request.value) {
        Ok(proposal_id) => Ok(warp::reply::json(&ProposalResponse {
            success: true,
            proposal_id: Some(proposal_id),
            message: "Proposal submitted successfully".to_string(),
        })),
        Err(err) => Ok(warp::reply::json(&ProposalResponse {
            success: false,
            proposal_id: None,
            message: err,
        })),
    }
}

async fn list_proposals_handler(
    governance: Arc<CrossLayerGovernance>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match governance.list_proposals() {
        Ok(proposals) => Ok(warp::reply::json(&ProposalsListResponse {
            success: true,
            proposals,
        })),
        Err(err) => Ok(warp::reply::json(&ProposalsListResponse {
            success: false,
            proposals: Vec::new(),
        })),
    }
}
