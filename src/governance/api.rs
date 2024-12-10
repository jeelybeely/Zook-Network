// File: src/governance/api.rs

use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use super::cross_layer_governance::{CrossLayerGovernance, GovernanceProposal};
use crate::bridge::state_anchoring::L2StateSummary;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SubmitProposalRequest {
    description: String,
    param: String,
    value: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApproveProposalRequest {
    proposal_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ValidateProposalRequest {
    proposal_id: u64,
    state_summary: L2StateSummary,
}

pub fn governance_routes(cross_governance: Arc<CrossLayerGovernance>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let submit_proposal = warp::path("governance")
        .and(warp::path("submit-proposal"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |request: SubmitProposalRequest| {
            let governance = cross_governance.clone();
            async move {
                match governance.submit_proposal(request.description, request.param, request.value) {
                    Ok(proposal_id) => Ok(warp::reply::json(&{"status": "success", "proposal_id": proposal_id})),
                    Err(e) => Ok(warp::reply::json(&{"status": "error", "message": e})),
                }
            }
        });

    let approve_proposal = warp::path("governance")
        .and(warp::path("approve-proposal"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |request: ApproveProposalRequest| {
            let governance = cross_governance.clone();
            async move {
                match governance.approve_proposal(request.proposal_id) {
                    Ok(_) => Ok(warp::reply::json(&{"status": "success", "message": "Proposal approved successfully"})),
                    Err(e) => Ok(warp::reply::json(&{"status": "error", "message": e})),
                }
            }
        });

    let validate_proposal = warp::path("governance")
        .and(warp::path("validate-proposal"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |request: ValidateProposalRequest| {
            let governance = cross_governance.clone();
            async move {
                match governance.validate_l1_proposal(&request.state_summary, request.proposal_id) {
                    Ok(_) => Ok(warp::reply::json(&{"status": "success", "message": "Proposal validated successfully"})),
                    Err(e) => Ok(warp::reply::json(&{"status": "error", "message": e})),
                }
            }
        });

    submit_proposal.or(approve_proposal).or(validate_proposal)
}
