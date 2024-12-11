use crate::governance::validator_rewards::ValidatorRewards;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::Filter;

#[derive(Serialize)]
struct ComplianceLogResponse {
    success: bool,
    logs: Option<Vec<ComplianceLogEntry>>,
    message: String,
}

#[derive(Serialize)]
struct ComplianceSummaryResponse {
    success: bool,
    summary: Option<Vec<(String, usize)>>, // Validator address and compliance count
    message: String,
}

#[derive(Clone)]
pub struct GovernanceAPI {
    validator_rewards: Arc<ValidatorRewards>,
}

impl GovernanceAPI {
    pub fn new(validator_rewards: Arc<ValidatorRewards>) -> Self {
        Self { validator_rewards }
    }

    pub fn routes(&self) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let api = warp::path("governance");

        // Fetch compliance logs for a specific validator
        let compliance_logs = {
            let validator_rewards = self.validator_rewards.clone();
            warp::path("compliance-logs")
                .and(warp::get())
                .and(warp::query::<String>())
                .and_then(move |validator_address: String| {
                    let validator_rewards = validator_rewards.clone();
                    async move {
                        let logs = validator_rewards.get_compliance_logs(&validator_address);
                        if let Some(logs) = logs {
                            Ok::<_, warp::Rejection>(warp::reply::json(&ComplianceLogResponse {
                                success: true,
                                logs: Some(logs),
                                message: "Compliance logs retrieved successfully".to_string(),
                            }))
                        } else {
                            Ok(warp::reply::json(&ComplianceLogResponse {
                                success: false,
                                logs: None,
                                message: "No logs found for the specified validator".to_string(),
                            }))
                        }
                    }
                })
        };

        // Fetch compliance summary for all validators
        let compliance_summary = {
            let validator_rewards = self.validator_rewards.clone();
            warp::path("compliance-summary")
                .and(warp::get())
                .and_then(move || {
                    let validator_rewards = validator_rewards.clone();
                    async move {
                        let summary = validator_rewards.get_compliance_summary();
                        Ok::<_, warp::Rejection>(warp::reply::json(&ComplianceSummaryResponse {
                            success: true,
                            summary: Some(summary),
                            message: "Compliance summary retrieved successfully".to_string(),
                        }))
                    }
                })
        };

        api.and(compliance_logs.or(compliance_summary))
    }
}
