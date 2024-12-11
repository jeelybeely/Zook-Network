use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::bridge::state_anchoring::L2StateSummary;
use crate::bridge::merkle::MerkleTree;
use crate::validator::node_registration::ValidatorRegistry;
use crate::governance::validator_policies::GovernanceValidatorPolicies;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchoredState {
    pub block_height: u64,
    pub state_root: String,
    pub merkle_proof: Vec<String>, // Added Merkle proof
    pub timestamp: DateTime<Utc>,
    pub validator_compliance: bool, // Reflects whether validators met governance policies
}

pub struct CrossLayerSync {
    pub anchored_states: Arc<Mutex<Vec<AnchoredState>>>,
}

impl CrossLayerSync {
    pub fn new() -> Self {
        Self {
            anchored_states: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn anchor_state(
        &self,
        state_summary: L2StateSummary,
        compliance: bool,
        merkle_proof: Vec<String>,
    ) -> Result<(), String> {
        let mut states = self.anchored_states.lock().map_err(|_| "Mutex lock failed")?;
        let anchored_state = AnchoredState {
            block_height: state_summary.block_height,
            state_root: state_summary.state_root,
            merkle_proof,
            timestamp: state_summary.timestamp,
            validator_compliance: compliance,
        };
        states.push(anchored_state);
        println!("State anchored: {:?}", states.last());
        Ok(())
    }

    pub fn validate_state(&self, state_root: &str) -> bool {
        let states = self.anchored_states.lock().unwrap_or_default();
        states.iter().any(|state| state.state_root == state_root)
    }

    pub fn get_latest_state(&self) -> Option<AnchoredState> {
        let states = self.anchored_states.lock().ok()?;
        states.last().cloned()
    }

    pub fn generate_anchor_summary(
        merkle_tree: &MerkleTree,
        block_height: u64,
        total_transactions: u64,
        validator_registry: &ValidatorRegistry,
        governance_policies: &GovernanceValidatorPolicies,
    ) -> L2StateSummary {
        let compliance = Self::check_validator_compliance(validator_registry, governance_policies);

        L2StateSummary {
            block_height,
            state_root: merkle_tree.get_root(),
            total_transactions,
            timestamp: Utc::now(),
            compliance,
        }
    }

    pub fn generate_merkle_proof(
        merkle_tree: &MerkleTree,
        transaction_hash: &str,
    ) -> Vec<String> {
        merkle_tree.get_proof(transaction_hash)
    }

    fn check_validator_compliance(
        validator_registry: &ValidatorRegistry,
        governance_policies: &GovernanceValidatorPolicies,
    ) -> bool {
        let policy = governance_policies.get_policy().unwrap_or_default();
        let nodes = validator_registry.nodes.lock().unwrap_or_default();

        nodes.iter().all(|(_, node)| {
            // Example compliance logic (can be expanded as needed)
            node.active // Ensure node is active
        })
    }
}
