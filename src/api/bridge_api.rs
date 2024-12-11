use crate::clarity::interaction::ClarityInteractor;
use serde::{Deserialize, Serialize};
use warp::Filter;
use tokio::sync::Arc;

#[derive(Deserialize)]
struct LockRequest {
    amount: u64,
    sender_address: String,
}

#[derive(Serialize)]
struct LockResponse {
    success: bool,
    transaction_id: Option<String>,
    message: String,
}

#[derive(Deserialize)]
struct BurnRequest {
    amount: u64,
    sender_address: String,
}

#[derive(Serialize)]
struct BurnResponse {
    success: bool,
    transaction_id: Option<String>,
    message: String,
}

#[derive(Clone)]
pub struct BridgeAPI {
    clarity: Arc<ClarityInteractor>,
}

impl BridgeAPI {
    pub fn new(clarity: Arc<ClarityInteractor>) -> Self {
        Self { clarity }
    }

    pub fn routes(&self) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let api = warp::path("bridge");

        // Lock tokens endpoint
        let lock = {
            let clarity = self.clarity.clone();
            warp::path("lock")
                .and(warp::post())
                .and(warp::body::json())
                .and_then(move |req: LockRequest| {
                    let clarity = clarity.clone();
                    async move {
                        match clarity.lock_tokens(req.amount, req.sender_address.clone()).await {
                            Ok(tx_id) => Ok::<_, warp::Rejection>(warp::reply::json(&LockResponse {
                                success: true,
                                transaction_id: Some(tx_id),
                                message: "Tokens locked successfully".to_string(),
                            })),
                            Err(e) => Err(warp::reject::custom(e)),
                        }
                    }
                })
        };

        // Burn tokens endpoint
        let burn = {
            let clarity = self.clarity.clone();
            warp::path("burn")
                .and(warp::post())
                .and(warp::body::json())
                .and_then(move |req: BurnRequest| {
                    let clarity = clarity.clone();
                    async move {
                        match clarity.burn_tokens(req.amount, req.sender_address.clone()).await {
                            Ok(tx_id) => Ok::<_, warp::Rejection>(warp::reply::json(&BurnResponse {
                                success: true,
                                transaction_id: Some(tx_id),
                                message: "Tokens burned successfully".to_string(),
                            })),
                            Err(e) => Err(warp::reject::custom(e)),
                        }
                    }
                })
        };

        // Combine bridge endpoints
        api.and(lock.or(burn))
    }
}
