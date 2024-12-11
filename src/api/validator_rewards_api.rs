use crate::clarity::interaction::ClarityInteractor;
use serde::{Deserialize, Serialize};
use warp::Filter;
use tokio::sync::Arc;

#[derive(Serialize)]
struct RewardResponse {
    success: bool,
    message: String,
    rewards_distributed: Option<u64>,
}

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
                            Err(e) => Err(warp::reject::custom(e)),
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
                            Err(e) => Err(warp::reject::custom(e)),
                        }
                    }
                })
        };

        // Combine validator rewards endpoints
        api.and(distribute_rewards.or(query_rewards))
    }
}
