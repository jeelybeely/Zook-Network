// File: src/governance/api.rs

use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use super::governance_scalability::{GovernanceState, ParameterChange};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProposalRequest {
    creator: String,
    description: String,
    param: String,
    value: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VoteRequest {
    proposal_id: u64,
    support: bool,
    voting_power: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExecuteRequest {
    proposal_id: u64,
}

pub fn governance_routes(state: Arc<GovernanceState>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let create_proposal = warp::path("governance")
        .and(warp::path("propose"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |request: ProposalRequest| {
            let state = state.clone();
            async move {
                match state.create_proposal(
                    request.creator,
                    request.description,
                    Some(ParameterChange {
                        param: request.param,
                        value: request.value,
                    }),
                ) {
                    Ok(proposal_id) => Ok(warp::reply::json(&{
                        "status": "success",
                        "proposal_id": proposal_id,
                    })),
                    Err(e) => Ok(warp::reply::json(&{
                        "status": "error",
                        "message": e,
                    })),
                }
            }
        });

    let vote_proposal = warp::path("governance")
        .and(warp::path("vote"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |request: VoteRequest| {
            let state = state.clone();
            async move {
                match state.vote_on_proposal(request.proposal_id, request.support, request.voting_power) {
                    Ok(_) => Ok(warp::reply::json(&{"status": "success", "message": "Vote recorded"})),
                    Err(e) => Ok(warp::reply::json(&{"status": "error", "message": e})),
                }
            }
        });

    let execute_proposal = warp::path("governance")
        .and(warp::path("execute"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |request: ExecuteRequest| {
            let state = state.clone();
            async move {
                match state.execute_proposal(request.proposal_id) {
                    Ok(_) => Ok(warp::reply::json(&{"status": "success", "message": "Proposal executed"})),
                    Err(e) => Ok(warp::reply::json(&{"status": "error", "message": e})),
                }
            }
        });

    create_proposal.or(vote_proposal).or(execute_proposal)
}
