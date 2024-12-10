// File: src/validator/node_api.rs

use warp::{Filter, Rejection, Reply};
use std::sync::Arc;
use crate::validator::node_registration::ValidatorRegistry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RegisterNodeRequest {
    address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeactivateNodeRequest {
    address: String,
}

pub fn node_api_routes(
    registry: Arc<ValidatorRegistry>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let register_node = warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |request: RegisterNodeRequest| {
            let registry = registry.clone();
            async move {
                match registry.register_node(request.address) {
                    Ok(_) => Ok(warp::reply::json(&{"status": "success", "message": "Node registered successfully"})),
                    Err(e) => Ok(warp::reply::json(&{"status": "error", "message": e})),
                }
            }
        });

    let list_nodes = warp::path("list")
        .and(warp::get())
        .and_then(move || {
            let registry = registry.clone();
            async move {
                match registry.list_nodes() {
                    Ok(nodes) => Ok(warp::reply::json(&{"status": "success", "nodes": nodes})),
                    Err(e) => Ok(warp::reply::json(&{"status": "error", "message": e})),
                }
            }
        });

    let deactivate_node = warp::path("deactivate")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |request: DeactivateNodeRequest| {
            let registry = registry.clone();
            async move {
                match registry.deactivate_node(&request.address) {
                    Ok(_) => Ok(warp::reply::json(&{"status": "success", "message": "Node deactivated successfully"})),
                    Err(e) => Ok(warp::reply::json(&{"status": "error", "message": e})),
                }
            }
        });

    warp::path("nodes").and(register_node.or(list_nodes).or(deactivate_node))
}
