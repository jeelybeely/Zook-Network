use crate::interaction::ClarityInteractor; // Adjusted path to match the file location
use serde::Serialize;
use warp::Filter;
use std::sync::Arc;

#[derive(Serialize)]
struct RewardResponse {
    success: bool,
    message: String,
    rewards_distributed: Option<u64>,
}

// Custom error type for Warp
#[derive(Debug)]
struct CustomError(String);

impl warp::reject::Reject for CustomError {}

#[derive(Clone)]
pub struct ValidatorRewardsAPI {
    clarity: Arc<ClarityInteractor>,
}

impl ValidatorRewardsAPI {
    pub fn new(clarity: Arc<ClarityInteractor>) -> Self {
        Self { clarity }
    }

    pub fn routes(&self) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let api = warp::path("validator-rewards");

        // Distribute rewards endpoint
        let distribute_rewards = {
            let clarity = self.clarity.clone();
            warp::path("distribute")
                .and(warp::post())
                .and_then(move || {
                    let clarity = clarity.clone();
                    async move {
                        match clarity.distribute_rewards().await {
                            Ok(total_rewards) => Ok::<_, warp::Rejection>(warp::reply::json(&RewardResponse {
                                success: true,
                                message: "Rewards distributed successfully".to_string(),
                                rewards_distributed: Some(total_rewards),
                            })),
                            Err(e) => Err(warp::reject::custom(CustomError(e))), // Use CustomError
                        }
                    }
                })
        };

        // Query rewards endpoint
        let query_rewards = {
            let clarity = self.clarity.clone();
            warp::path("query")
                .and(warp::get())
                .and_then(move || {
                    let clarity = clarity.clone();
                    async move {
                        match clarity.query_rewards().await {
                            Ok(total_rewards) => Ok::<_, warp::Rejection>(warp::reply::json(&RewardResponse {
                                success: true,
                                message: "Rewards queried successfully".to_string(),
                                rewards_distributed: Some(total_rewards),
                            })),
                            Err(e) => Err(warp::reject::custom(CustomError(e))), // Use CustomError
                        }
                    }
                })
        };

        // Combine validator rewards endpoints
        api.and(distribute_rewards.or(query_rewards))
    }
}
