// Updated validator_policies.rs

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::validator::node_registration::ValidatorRegistry;
use crate::validator::node_registration::ValidatorNode;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidatorPolicy {
    pub minimum_stake: u64,
    pub activity_threshold: u64, // Minimum activity percentage required
}

pub struct GovernanceValidatorPolicies {
    pub policy: Arc<Mutex<ValidatorPolicy>>,
}

impl GovernanceValidatorPolicies {
    pub fn new(minimum_stake: u64, activity_threshold: u64) -> Self {
        let policy = ValidatorPolicy {
            minimum_stake,
            activity_threshold,
        };

        Self {
            policy: Arc::new(Mutex::new(policy)),
        }
    }

    pub fn get_policy(&self) -> Option<ValidatorPolicy> {
        self.policy.lock().ok().map(|policy| policy.clone())
    }

    pub fn update_policy(&self, minimum_stake: u64, activity_threshold: u64) {
        if let Ok(mut policy) = self.policy.lock() {
            policy.minimum_stake = minimum_stake;
            policy.activity_threshold = activity_threshold;
        }
    }

    pub fn validate_node(&self, registry: &ValidatorRegistry, node_id: &str) -> bool {
        if let Some(policy) = self.get_policy() {
            if let Ok(node) = registry.get_node(node_id) {
                return node.staked_btcz >= policy.minimum_stake && node.active;
            }
        }
        false
    }

    pub fn adjust_voting_power(&self, node: &ValidatorNode) -> u64 {
        if let Some(policy) = self.get_policy() {
            let stake_factor = (node.staked_btcz as f64 / policy.minimum_stake as f64).min(1.0);
            if node.active {
                (stake_factor * 100.0) as u64 // Example voting power calculation
            } else {
                0
            }
        } else {
            0
        }
    }
}
