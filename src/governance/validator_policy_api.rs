// File: src/governance/validator_policy_api.rs

use warp::{Filter, Rejection, Reply};
use std::sync::Arc;
use crate::governance::validator_policies::GovernanceValidatorPolicies;
use crate::validator::node_registration::ValidatorRegistry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdatePolicyRequest {
    minimum_stake: Option<u64>,
    activity_threshold: Option<u64>,
}

pub fn policy_api_routes(
    policies: Arc<GovernanceValidatorPolicies>,
    registry: Arc<ValidatorRegistry>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let update_policy = warp::path("update")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |request: UpdatePolicyRequest| {
            let policies = policies.clone();
            let registry = registry.clone();
            async move {
                match policies.integrate_governance_proposal(
                    &registry,
                    request.minimum_stake,
                    request.activity_threshold,
                ) {
                    Ok(_) => Ok(warp::reply::json(&{"status": "success", "message": "Policy updated successfully"})),
                    Err(e) => Ok(warp::reply::json(&{"status": "error", "message": e})),
                }
            }
        });

    let get_policy = warp::path("get")
        .and(warp::get())
        .and_then(move || {
            let policies = policies.clone();
            async move {
                match policies.get_policy() {
                    Ok(policy) => Ok(warp::reply::json(&{"status": "success", "policy": policy})),
                    Err(e) => Ok(warp::reply::json(&{"status": "error", "message": e})),
                }
            }
        });

    warp::path("policy").and(update_policy.or(get_policy))
}
