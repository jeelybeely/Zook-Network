// File: src/api/bridge.rs

use crate::clarity::interaction::ClarityInteractor;
use serde::{Deserialize, Serialize};
use warp::Filter;
use tokio::sync::Arc;

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

        // Route to lock BTCZ
        let lock = {
            let clarity = self.clarity.clone();
            warp::path("lock")
                .and(warp::post())
                .and(warp::body::json())
                .and_then(move |params: (u128, u128)| {
                    let clarity = clarity.clone();
                    async move {
                        let (tx_id, amount) = params;
                        match clarity.lock_btcz(tx_id, amount).await {
                            Ok(response) => Ok::<_, warp::Rejection>(warp::reply::json(&response)),
                            Err(e) => Err(warp::reject::custom(e)),
                        }
                    }
                })
        };

        // Route to unlock BTCZ
        let unlock = {
            let clarity = self.clarity.clone();
            warp::path("unlock")
                .and(warp::post())
                .and(warp::body::json())
                .and_then(move |tx_id: u128| {
                    let clarity = clarity.clone();
                    async move {
                        match clarity.unlock_btcz(tx_id).await {
                            Ok(response) => Ok::<_, warp::Rejection>(warp::reply::json(&response)),
                            Err(e) => Err(warp::reject::custom(e)),
                        }
                    }
                })
        };

        api.and(lock.or(unlock))
    }
}
