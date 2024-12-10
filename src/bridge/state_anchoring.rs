// File: src/bridge/state_anchoring.rs

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};
use crate::bridge::merkle::MerkleTree;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L2StateSummary {
    pub block_height: u64,
    pub state_root: String,
    pub total_transactions: u64,
    pub timestamp: DateTime<Utc>,
}

pub struct StateAnchoring {
    pub anchored_states: Arc<Mutex<Vec<L2StateSummary>>>,
}

impl StateAnchoring {
    pub fn new() -> Self {
        Self {
            anchored_states: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn anchor_state(&self, summary: L2StateSummary) -> Result<(), String> {
        let mut states = self.anchored_states.lock().map_err(|_| "Mutex lock failed")?;
        states.push(summary);
        Ok(())
    }

    pub fn get_latest_anchored_state(&self) -> Option<L2StateSummary> {
        let states = self.anchored_states.lock().ok()?;
        states.last().cloned()
    }

    pub fn validate_anchored_state(&self, state_root: &str) -> bool {
        let states = self.anchored_states.lock().ok()?;
        states.iter().any(|state| state.state_root == state_root)
    }
}

pub fn generate_state_summary(merkle_tree: &MerkleTree, block_height: u64, total_transactions: u64) -> L2StateSummary {
    L2StateSummary {
        block_height,
        state_root: merkle_tree.get_root(),
        total_transactions,
        timestamp: Utc::now(),
    }
}
