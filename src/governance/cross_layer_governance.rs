// File: src/governance/cross_layer_governance.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::bridge::state_anchoring::{StateAnchoring, L2StateSummary};
use crate::validator::node_registration::ValidatorRegistry;
use crate::governance::validator_policies::{GovernanceValidatorPolicies, VotingPowerAdjustment};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceProposal {
    pub proposal_id: u64,
    pub description: String,
    pub param: String,
    pub value: u64,
    pub approved: bool,
    pub voting_power: u64, // Adjusted based on compliance
}

pub struct CrossLayerGovernance {
    pub proposals: Arc<Mutex<HashMap<u64, GovernanceProposal>>>,
    pub state_anchoring: Arc<StateAnchoring>,
    pub validator_registry: Arc<ValidatorRegistry>,
    pub governance_policies: Arc<GovernanceValidatorPolicies>,
    pub storage_path: PathBuf, // Persistent storage path
}

impl CrossLayerGovernance {
    pub fn new(
        state_anchoring: Arc<StateAnchoring>,
        validator_registry: Arc<ValidatorRegistry>,
        governance_policies: Arc<GovernanceValidatorPolicies>,
        storage_path: PathBuf,
    ) -> Self {
        let proposals = if storage_path.exists() {
            match fs::read_to_string(&storage_path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
                Err(_) => HashMap::new(),
            }
        } else {
            HashMap::new()
        };

        Self {
            proposals: Arc::new(Mutex::new(proposals)),
            state_anchoring,
            validator_registry,
            governance_policies,
            storage_path,
        }
    }

    pub fn save_to_disk(&self) -> Result<(), String> {
        let proposals = self.proposals.lock().map_err(|_| "Mutex lock failed")?;
        let serialized = serde_json::to_string(&*proposals).map_err(|_| "Serialization failed")?;
        fs::write(&self.storage_path, serialized).map_err(|_| "Failed to write to disk")?;
        Ok(())
    }

    pub fn submit_proposal(
        &self,
        description: String,
        param: String,
        value: u64,
    ) -> Result<u64, String> {
        let mut proposals = self.proposals.lock().map_err(|_| "Mutex lock failed")?;
        let proposal_id = (proposals.len() as u64) + 1;
        let proposal = GovernanceProposal {
            proposal_id,
            description,
            param,
            value,
            approved: false,
            voting_power: 0, // Initialized to zero until approved
        };
        proposals.insert(proposal_id, proposal);
        self.save_to_disk()?;
        Ok(proposal_id)
    }

    pub fn approve_proposal(&self, proposal_id: u64) -> Result<(), String> {
        let mut proposals = self.proposals.lock().map_err(|_| "Mutex lock failed")?;
        let proposal = proposals
            .get_mut(&proposal_id)
            .ok_or("Proposal not found")?;

        proposal.approved = true;
        proposal.voting_power = self.calculate_voting_power()?; // Adjust voting power
        self.apply_proposal(proposal.clone())?;
        self.save_to_disk()?;
        Ok(())
    }

    fn calculate_voting_power(&self) -> Result<u64, String> {
        let nodes = self.validator_registry.nodes.lock().map_err(|_| "Mutex lock failed")?;
        let policies = self.governance_policies.get_policy().unwrap_or_default();

        let total_power: u64 = nodes
            .iter()
            .filter(|(_, node)| node.active) // Only consider active nodes
            .map(|(_, node)| policies.adjust_voting_power(node)) // Adjust power based on compliance
            .sum();

        Ok(total_power)
    }

    fn apply_proposal(&self, proposal: GovernanceProposal) -> Result<(), String> {
        match proposal.param.as_str() {
            "anchoring_frequency" => {
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
        Ok(proposals.values().cloned().collect())
    }

    pub fn validate_l1_proposal(&self, state_summary: &L2StateSummary, proposal_id: u64) -> Result<(), String> {
        let proposals = self.proposals.lock().map_err(|_| "Mutex lock failed")?;
        let proposal = proposals
            .get(&proposal_id)
            .filter(|p| p.approved)
            .ok_or("Invalid or unapproved proposal")?;

        println!("Validating proposal {} on L1 with state root: {}", proposal_id, state_summary.state_root);
        Ok(())
    }
}
