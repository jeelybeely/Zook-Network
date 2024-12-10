// File: src/governance/validator_policies.rs

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::validator::node_registration::ValidatorRegistry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorPolicy {
    pub minimum_stake: u64,
    pub activity_threshold: u64, // Minimum activity percentage required
}

pub struct GovernanceValidatorPolicies {
    pub policy: Arc<Mutex<ValidatorPolicy>>,
}

impl GovernanceValidatorPolicies {
    pub fn new(minimum_stake: u64, activity_threshold: u64) -> Self {
        Self {
            policy: Arc::new(Mutex::new(ValidatorPolicy {
                minimum_stake,
                activity_threshold,
            })),
        }
    }

    pub fn update_policy(&self, minimum_stake: Option<u64>, activity_threshold: Option<u64>) -> Result<(), String> {
        let mut policy = self.policy.lock().map_err(|_| "Mutex lock failed")?;

        if let Some(stake) = minimum_stake {
            policy.minimum_stake = stake;
        }
        if let Some(threshold) = activity_threshold {
            policy.activity_threshold = threshold;
        }

        println!("Updated validator policy: {:?}", *policy);
        Ok(())
    }

    pub fn apply_policy(&self, registry: &ValidatorRegistry) -> Result<(), String> {
        let policy = self.policy.lock().map_err(|_| "Mutex lock failed")?;
        let nodes = registry.nodes.lock().map_err(|_| "Mutex lock failed")?;

        for (address, node) in nodes.iter() {
            // Example: Apply policy rules (e.g., deactivate nodes below threshold)
            println!("Applying policy to node {}: {:?}", address, node);
        }

        Ok(())
    }

    pub fn get_policy(&self) -> Result<ValidatorPolicy, String> {
        let policy = self.policy.lock().map_err(|_| "Mutex lock failed")?;
        Ok(policy.clone())
    }
}
