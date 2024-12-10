// File: src/api/governance_api.rs

use crate::clarity::interaction::ClarityInteractor;
use serde::{Deserialize, Serialize};
use warp::Filter;
use tokio::sync::Arc;

#[derive(Clone)]
pub struct GovernanceAPI {
    clarity: Arc<ClarityInteractor>,
}

impl GovernanceAPI {
    pub fn new(clarity: Arc<ClarityInteractor>) -> Self {
        Self { clarity }
    }

    pub fn routes(&self) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let api = warp::path("governance");

        // Route for staking BTCZ
        let stake = {
            let clarity = self.clarity.clone();
            warp::path("stake")
                .and(warp::post())
                .and(warp::body::json())
                .and_then(move |amount: u128| {
                    let clarity = clarity.clone();
                    async move {
                        match clarity.stake_btcz(amount).await {
                            Ok(response) => Ok::<_, warp::Rejection>(warp::reply::json(&response)),
                            Err(e) => Err(warp::reject::custom(e)),
                        }
                    }
                })
        };

        // Route for unstaking BTCZ
        let unstake = {
            let clarity = self.clarity.clone();
            warp::path("unstake")
                .and(warp::post())
                .and(warp::body::json())
                .and_then(move |amount: u128| {
                    let clarity = clarity.clone();
                    async move {
                        match clarity.unstake_btcz(amount).await {
                            Ok(response) => Ok::<_, warp::Rejection>(warp::reply::json(&response)),
                            Err(e) => Err(warp::reject::custom(e)),
                        }
                    }
                })
        };

        api.and(stake.or(unstake))
    }
}