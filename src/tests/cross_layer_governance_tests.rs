// File: src/tests/cross_layer_governance_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use warp::test::request;
    use tokio::sync::Arc;
    use crate::governance::cross_layer_governance::{CrossLayerGovernance, GovernanceProposal};
    use crate::bridge::state_anchoring::{StateAnchoring, L2StateSummary};
    use chrono::Utc;

    #[tokio::test]
    async fn test_submit_proposal() {
        let state_anchoring = Arc::new(StateAnchoring::new());
        let governance = Arc::new(CrossLayerGovernance::new(state_anchoring.clone()));

        let request = serde_json::json!({
            "description": "Adjust anchoring frequency",
            "param": "anchoring_frequency",
            "value": 600
        });

        let response = request()
            .method("POST")
            .path("/governance/submit-proposal")
            .json(&request)
            .reply(&crate::governance::api::governance_routes(governance.clone()))
            .await;

        assert_eq!(response.status(), 200);
        assert!(response.body().contains("proposal_id"));
    }

    #[tokio::test]
    async fn test_approve_proposal() {
        let state_anchoring = Arc::new(StateAnchoring::new());
        let governance = Arc::new(CrossLayerGovernance::new(state_anchoring.clone()));

        let proposal_id = governance.submit_proposal(
            "Update validator rewards".to_string(),
            "validator_rewards".to_string(),
            20
        ).unwrap();

        let request = serde_json::json!({ "proposal_id": proposal_id });

        let response = request()
            .method("POST")
            .path("/governance/approve-proposal")
            .json(&request)
            .reply(&crate::governance::api::governance_routes(governance.clone()))
            .await;

        assert_eq!(response.status(), 200);
        assert!(response.body().contains("Proposal approved successfully"));
    }

    #[tokio::test]
    async fn test_validate_proposal() {
        let state_anchoring = Arc::new(StateAnchoring::new());
        let governance = Arc::new(CrossLayerGovernance::new(state_anchoring.clone()));

        let summary = L2StateSummary {
            block_height: 100,
            state_root: "sample_root".to_string(),
            total_transactions: 50,
            timestamp: Utc::now(),
        };
        state_anchoring.anchor_state(summary.clone()).unwrap();

        let proposal_id = governance.submit_proposal(
            "Increase validator penalties".to_string(),
            "validator_penalty".to_string(),
            50
        ).unwrap();
        governance.approve_proposal(proposal_id).unwrap();

        let request = serde_json::json!({
            "proposal_id": proposal_id,
            "state_summary": summary
        });

        let response = request()
            .method("POST")
            .path("/governance/validate-proposal")
            .json(&request)
            .reply(&crate::governance::api::governance_routes(governance.clone()))
            .await;

        assert_eq!(response.status(), 200);
        assert!(response.body().contains("Proposal validated successfully"));
    }
}
