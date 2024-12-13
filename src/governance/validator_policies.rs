// Complete and updated validator_policies.rs

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::validator::node_registration::ValidatorRegistry;

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
            if let Some(node) = registry.get_node(node_id) {
                return node.staked_btcz >= policy.minimum_stake;
            }
        }
        false
    }
}
