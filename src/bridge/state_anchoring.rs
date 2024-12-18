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
    pub compliance: bool,
}

#[derive(Debug)]
pub struct StateAnchoring {
    pub anchored_states: Arc<Mutex<Vec<L2StateSummary>>>,
    pub anchoring_frequency: Mutex<u64>, // Add frequency state
}

impl StateAnchoring {
    pub fn new() -> Self {
        Self {
            anchored_states: Arc::new(Mutex::new(Vec::new())),
            anchoring_frequency: Mutex::new(60), // Default to 60 seconds
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
        if let Ok(states) = self.anchored_states.lock() {
            states.iter().any(|state| state.state_root == state_root)
        } else {
            false // Return false if the lock fails
        }
    }

    pub fn update_frequency(&self, new_frequency: u64) -> Result<(), String> {
        let mut frequency = self.anchoring_frequency.lock().map_err(|_| "Mutex lock failed")?;
        *frequency = new_frequency;
        println!("Anchoring frequency updated to {} seconds", new_frequency);
        Ok(())
    }

    pub fn get_anchoring_frequency(&self) -> Result<u64, String> {
        let frequency = self.anchoring_frequency.lock().map_err(|_| "Mutex lock failed")?;
        Ok(*frequency)
    }
}

pub fn generate_state_summary(
    merkle_tree: &MerkleTree,
    block_height: u64,
    total_transactions: u64,
    compliance: bool, // Added compliance as an argument
) -> L2StateSummary {
    L2StateSummary {
        block_height,
        state_root: String::from_utf8_lossy(&merkle_tree.get_root()).to_string(),
        total_transactions,
        timestamp: Utc::now(),
        compliance,
    }
}
