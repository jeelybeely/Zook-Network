// File: src/bridge/cross_layer_sync.rs

use std::collections::HashMap; // FIX: Added the missing HashMap import
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::bridge::state_anchoring::L2StateSummary;
use crate::bridge::merkle::MerkleTree;
use crate::validator::node_registration::ValidatorRegistry;
use crate::governance::validator_policies::GovernanceValidatorPolicies;
use crate::bridge::btcz_integration::{BTCZIntegration, BTCZAnchorPayload};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchoredState {
    pub block_height: u64,
    pub state_root: String,
    pub merkle_proof: Vec<String>, 
    pub timestamp: DateTime<Utc>,
    pub validator_compliance: bool, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockEvent {
    pub tx_id: String,
    pub amount: u64,
    pub from_address: String,
    pub to_address: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnEvent {
    pub tx_id: String,
    pub amount: u64,
    pub from_address: String,
    pub to_address: String,
    pub timestamp: DateTime<Utc>,
}

pub struct CrossLayerSync {
    pub anchored_states: Arc<Mutex<Vec<AnchoredState>>>,
    pub btcz_integration: Arc<BTCZIntegration>, // Integration with BTCZ
    pub lock_events: Arc<Mutex<Vec<LockEvent>>>,
    pub burn_events: Arc<Mutex<Vec<BurnEvent>>>,
}

impl CrossLayerSync {
    pub fn new(btcz_integration: Arc<BTCZIntegration>) -> Self {
        Self {
            anchored_states: Arc::new(Mutex::new(Vec::new())),
            btcz_integration,
            lock_events: Arc::new(Mutex::new(Vec::new())),
            burn_events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn record_lock_event(&self, event: LockEvent) -> Result<(), String> {
        let mut events = self.lock_events.lock().map_err(|_| "Mutex lock failed")?;
        events.push(event);
        println!("Lock event recorded.");
        Ok(())
    }

    pub fn record_burn_event(&self, event: BurnEvent) -> Result<(), String> {
        let mut events = self.burn_events.lock().map_err(|_| "Mutex lock failed")?;
        events.push(event);
        println!("Burn event recorded.");
        Ok(())
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
            state_root: state_summary.state_root.clone(),
            merkle_proof: merkle_proof.clone(),
            timestamp: state_summary.timestamp,
            validator_compliance: compliance,
        };
        states.push(anchored_state);

        // Send to BTCZ
        let payload = BTCZAnchorPayload {
            block_height: state_summary.block_height,
            state_root: state_summary.state_root.clone(),
            merkle_proof,
            validator_compliance: compliance,
            timestamp: Utc::now(),
        };

        tokio::spawn({
            let integration = self.btcz_integration.clone();
            async move {
                if let Err(e) = integration.send_anchor(payload).await {
                    eprintln!("Failed to synchronize state with BTCZ: {}", e);
                }
            }
        });

        println!("State anchored: {:?}", states.last());
        Ok(())
    }

    pub fn validate_state(&self, state_root: &str) -> bool {
        let states = self.anchored_states.lock().map(|guard| guard.clone()).unwrap_or_default();
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
            state_root: String::from_utf8_lossy(&merkle_tree.get_root()).to_string(),
            total_transactions,
            timestamp: Utc::now(),
            compliance,
        }
    }

    pub fn generate_merkle_proof(
        merkle_tree: &MerkleTree,
        transaction_hash: &str,
    ) -> Vec<String> {
        merkle_tree
            .get_proof(&transaction_hash.as_bytes().to_vec())
            .into_iter()
            .map(|hash| String::from_utf8_lossy(&hash).to_string())
            .collect()
    }

    fn check_validator_compliance(
        validator_registry: &ValidatorRegistry,
        governance_policies: &GovernanceValidatorPolicies,
    ) -> bool {
        let policy = governance_policies.get_policy().unwrap_or_default();
        let nodes = match validator_registry.nodes.lock() {
            Ok(guard) => guard.clone(),
            Err(_) => HashMap::new(),
        };

        nodes.iter().all(|(_, node)| {
            // Example compliance logic (can be expanded as needed)
            node.active
        })
    }
}
