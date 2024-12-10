// File: src/auth/security.rs

use warp::{Filter, Rejection};
use std::sync::Arc;

pub fn secure_filter(expected_token: String) -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::header::optional::<String>("Authorization")
        .and_then(move |auth_header: Option<String>| {
            let token = expected_token.clone();
            async move {
                match auth_header {
                    Some(header) if header == format!("Bearer {}", token) => Ok(()),
                    _ => Err(warp::reject::custom(Unauthorized)),
                }
            }
        })
}

#[derive(Debug)]
pub struct Unauthorized;

impl warp::reject::Reject for Unauthorized {}

pub fn with_auth(auth_token: Arc<String>) -> impl Filter<Extract = (), Error = Rejection> + Clone {
    secure_filter(auth_token.to_string())
}

pub fn authenticate_routes(token: Arc<String>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("secure")
        .and(warp::get())
        .and(with_auth(token.clone()))
        .map(|| warp::reply::json(&{"status": "success", "message": "Authenticated"}))
        .or(warp::path("test")
            .and(warp::get())
            .map(|| warp::reply::json(&{"status": "success", "message": "Test Endpoint Accessible"})))
}

/// Integrate with governance and validator APIs
pub fn secure_governance_routes(token: Arc<String>, inner_routes: impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    with_auth(token).and(inner_routes)
}

/// Integrate with validator APIs
pub fn secure_validator_routes(token: Arc<String>, inner_routes: impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    with_auth(token).and(inner_routes)
}

/// Example combined route for governance
pub fn combined_governance_routes(
    token: Arc<String>,
    governance_routes: impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone,
    validator_routes: impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("api")
        .and(
            secure_governance_routes(token.clone(), governance_routes)
                .or(secure_validator_routes(token, validator_routes))
        )
}
