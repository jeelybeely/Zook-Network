use crate::bridge::cross_layer_sync::{AnchoredState, CrossLayerSync};
use crate::bridge::state_anchoring::L2StateSummary; // Import the correct type
use serde::Serialize;
use warp::{Filter, Rejection, Reply};
use std::sync::Arc;

#[derive(Serialize)]
struct AnchorResponse {
    success: bool,
    message: String,
}

#[derive(Serialize)]
struct StateResponse {
    success: bool,
    state: Option<AnchoredState>,
}

#[derive(Debug)]
struct APIError(String);

impl warp::reject::Reject for APIError {}

#[derive(Clone)]
pub struct AnchoringAPI {
    cross_layer_sync: Arc<CrossLayerSync>,
}

impl AnchoringAPI {
    pub fn new(cross_layer_sync: Arc<CrossLayerSync>) -> Self {
        Self { cross_layer_sync }
    }

    pub fn routes(&self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let api = warp::path("anchoring");

        // Endpoint to anchor a state
        let anchor_state = {
            let cross_layer_sync = self.cross_layer_sync.clone();
            warp::path("anchor")
                .and(warp::post())
                .and(warp::body::json())
                .and_then(move |state: AnchoredState| {
                    let cross_layer_sync = cross_layer_sync.clone();
                    async move {
                        let state_summary = L2StateSummary {
                            block_height: state.block_height,
                            state_root: state.state_root.clone(),
                            total_transactions: state.merkle_proof.len() as u64, // Example calculation
                            timestamp: state.timestamp,
                            compliance: state.validator_compliance,
                        };
                        match cross_layer_sync.anchor_state(
                            state_summary,
                            state.validator_compliance,
                            state.merkle_proof.clone(),
                        ) {
                            Ok(_) => Ok::<_, Rejection>(warp::reply::json(&AnchorResponse {
                                success: true,
                                message: "State anchored successfully".to_string(),
                            })),
                            Err(e) => Err(warp::reject::custom(APIError(e))),
                        }
                    }
                })
        };

        // Endpoint to fetch the latest state
        let get_latest_state = {
            let cross_layer_sync = self.cross_layer_sync.clone();
            warp::path("latest")
                .and(warp::get())
                .and_then(move || {
                    let cross_layer_sync = cross_layer_sync.clone();
                    async move {
                        let latest_state = cross_layer_sync.get_latest_state();
                        Ok::<_, Rejection>(warp::reply::json(&StateResponse {
                            success: latest_state.is_some(),
                            state: latest_state,
                        }))
                    }
                })
        };

        // Combine endpoints
        api.and(anchor_state.or(get_latest_state))
    }
}
