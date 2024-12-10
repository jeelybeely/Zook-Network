// File: src/governance/cross_layer_governance.rs

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::bridge::state_anchoring::{StateAnchoring, L2StateSummary};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceProposal {
    pub proposal_id: u64,
    pub description: String,
    pub param: String,
    pub value: u64,
    pub approved: bool,
}

pub struct CrossLayerGovernance {
    pub proposals: Arc<Mutex<Vec<GovernanceProposal>>>,
    pub state_anchoring: Arc<StateAnchoring>,
}

impl CrossLayerGovernance {
    pub fn new(state_anchoring: Arc<StateAnchoring>) -> Self {
        Self {
            proposals: Arc::new(Mutex::new(Vec::new())),
            state_anchoring,
        }
    }

    pub fn submit_proposal(&self, description: String, param: String, value: u64) -> Result<u64, String> {
        let mut proposals = self.proposals.lock().map_err(|_| "Mutex lock failed")?;
        let proposal_id = (proposals.len() as u64) + 1;
        let proposal = GovernanceProposal {
            proposal_id,
            description,
            param,
            value,
            approved: false,
        };
        proposals.push(proposal);
        Ok(proposal_id)
    }

    pub fn approve_proposal(&self, proposal_id: u64) -> Result<(), String> {
        let mut proposals = self.proposals.lock().map_err(|_| "Mutex lock failed")?;
        let proposal = proposals.iter_mut().find(|p| p.proposal_id == proposal_id).ok_or("Proposal not found")?;
        proposal.approved = true;
        self.apply_proposal(proposal.clone())?;
        Ok(())
    }

    fn apply_proposal(&self, proposal: GovernanceProposal) -> Result<(), String> {
        match proposal.param.as_str() {
            "anchoring_frequency" => {
                // Example: Modify how frequently states are anchored
                println!("Applying anchoring frequency change to {} seconds", proposal.value);
                Ok(())
            }
            "validator_rewards" => {
                println!("Updating validator reward rate to {}", proposal.value);
                Ok(())
            }
            _ => Err("Unknown governance parameter".to_string()),
        }
    }

    pub fn list_proposals(&self) -> Result<Vec<GovernanceProposal>, String> {
        let proposals = self.proposals.lock().map_err(|_| "Mutex lock failed")?;
        Ok(proposals.clone())
    }

    pub fn validate_l1_proposal(&self, state_summary: &L2StateSummary, proposal_id: u64) -> Result<(), String> {
        let proposals = self.proposals.lock().map_err(|_| "Mutex lock failed")?;
        let proposal = proposals.iter().find(|p| p.proposal_id == proposal_id && p.approved).ok_or("Invalid or unapproved proposal")?;

        println!("Validating proposal {} on L1 with state root: {}", proposal_id, state_summary.state_root);
        Ok(())
    }
}
