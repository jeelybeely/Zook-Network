use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::validator::node_registration::ValidatorRegistry;
use crate::api::security::{with_auth, ApiKey};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct NodeRegisterRequest {
    address: String,
    staked_btcz: u64,
}

pub fn node_api_routes(
    registry: Arc<ValidatorRegistry>,
    api_keys: Arc<std::collections::HashMap<String, ApiKey>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let register_registry = registry.clone();
    let list_registry = registry.clone();
    let deactivate_registry = registry.clone();

    let register_node = warp::post()
        .and(warp::path("node"))
        .and(warp::path("register"))
        .and(warp::body::json())
        .and(with_auth(api_keys.clone()))
        .and_then(move |request: NodeRegisterRequest, _auth: ApiKey| {
            let registry = register_registry.clone();
            async move {
                match registry.register_node(request.address, request.staked_btcz) {
                    Ok(_) => Ok::<_, warp::Rejection>(warp::reply::json(&serde_json::json!({
                        "status": "success",
                        "message": "Node registered successfully",
                    }))),
                    Err(e) => Ok::<_, warp::Rejection>(warp::reply::json(&serde_json::json!({
                        "status": "error",
                        "message": e,
                    }))),
                }
            }
        });

    let list_nodes = warp::get()
        .and(warp::path("node"))
        .and(warp::path("list"))
        .and_then(move || {
            let registry = list_registry.clone();
            async move {
                match registry.list_nodes() {
                    Ok(nodes) => Ok::<_, warp::Rejection>(warp::reply::json(&serde_json::json!({
                        "status": "success",
                        "nodes": nodes,
                    }))),
                    Err(e) => Ok::<_, warp::Rejection>(warp::reply::json(&serde_json::json!({
                        "status": "error",
                        "message": e,
                    }))),
                }
            }
        });

    let deactivate_node = warp::post()
        .and(warp::path("node"))
        .and(warp::path("deactivate"))
        .and(warp::body::json())
        .and(with_auth(api_keys.clone()))
        .and_then(move |request: NodeRegisterRequest, _auth: ApiKey| {
            let registry = deactivate_registry.clone();
            async move {
                match registry.deactivate_node(&request.address) {
                    Ok(_) => Ok::<_, warp::Rejection>(warp::reply::json(&serde_json::json!({
                        "status": "success",
                        "message": "Node deactivated successfully",
                    }))),
                    Err(e) => Ok::<_, warp::Rejection>(warp::reply::json(&serde_json::json!({
                        "status": "error",
                        "message": e,
                    }))),
                }
            }
        });

    register_node.or(list_nodes).or(deactivate_node)
}
