// File: src/api/router.rs

use warp::{Filter, Rejection};
use std::sync::Arc;
use crate::auth::security::{secure_governance_routes, secure_validator_routes, with_auth};

pub fn api_routes(
    token: Arc<String>,
    governance_routes: impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone,
    validator_routes: impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone,
    bridge_routes: impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone,
) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::path("api")
        .and(
            secure_governance_routes(token.clone(), governance_routes)
                .or(secure_validator_routes(token.clone(), validator_routes))
                .or(with_auth(token).and(bridge_routes))
        )
}
