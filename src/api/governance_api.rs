use crate::clarity::interaction::ClarityInteractor;
use serde::{Deserialize, Serialize};
use warp::Filter;
use tokio::sync::Arc;

#[derive(Deserialize)]
struct Proposal {
    proposal_id: u64,
    description: String,
    changes: Vec<(String, String)>,
}

#[derive(Serialize)]
struct Response {
    success: bool,
    message: String,
}

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

        // Proposal submission endpoint
        let submit_proposal = {
            let clarity = self.clarity.clone();
            warp::path("submit-proposal")
                .and(warp::post())
                .and(warp::body::json())
                .and_then(move |proposal: Proposal| {
                    let clarity = clarity.clone();
                    async move {
                        match clarity.submit_proposal(proposal.proposal_id, proposal.description, proposal.changes).await {
                            Ok(response) => Ok::<_, warp::Rejection>(warp::reply::json(&Response {
                                success: true,
                                message: format!("Proposal {} submitted", proposal.proposal_id),
                            })),
                            Err(e) => Err(warp::reject::custom(e)),
                        }
                    }
                })
        };

        // Voting endpoint
        let vote = {
            let clarity = self.clarity.clone();
            warp::path("vote")
                .and(warp::post())
                .and(warp::body::json())
                .and_then(move |proposal_id: u64| {
                    let clarity = clarity.clone();
                    async move {
                        match clarity.vote_on_proposal(proposal_id).await {
                            Ok(_) => Ok::<_, warp::Rejection>(warp::reply::json(&Response {
                                success: true,
                                message: format!("Voted on proposal {}", proposal_id),
                            })),
                            Err(e) => Err(warp::reject::custom(e)),
                        }
                    }
                })
        };

        // Combine all governance endpoints
        api.and(submit_proposal.or(vote))
    }
}
