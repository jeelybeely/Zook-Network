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
