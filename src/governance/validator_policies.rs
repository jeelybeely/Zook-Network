use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::validator::node_registration::{ValidatorRegistry, ValidatorNode};

/// Defines the governance rules for validator policies
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidatorPolicy {
    pub minimum_stake: u64,
    pub activity_threshold: u64,    // Minimum activity percentage required
    pub compliance_threshold: u64, // Minimum compliance percentage for voting power
}

pub struct GovernanceValidatorPolicies {
    pub policy: Arc<Mutex<ValidatorPolicy>>,
}

impl GovernanceValidatorPolicies {
    /// Creates a new governance policy with default thresholds
    pub fn new(minimum_stake: u64, activity_threshold: u64, compliance_threshold: u64) -> Self {
        let policy = ValidatorPolicy {
            minimum_stake,
            activity_threshold,
            compliance_threshold,
        };

        Self {
            policy: Arc::new(Mutex::new(policy)),
        }
    }

    /// Retrieves the current governance policy
    pub fn get_policy(&self) -> Option<ValidatorPolicy> {
        self.policy.lock().ok().map(|policy| policy.clone())
    }

    /// Updates the governance policy thresholds
    pub fn update_policy(&self, minimum_stake: u64, activity_threshold: u64, compliance_threshold: u64) {
        if let Ok(mut policy) = self.policy.lock() {
            policy.minimum_stake = minimum_stake;
            policy.activity_threshold = activity_threshold;
            policy.compliance_threshold = compliance_threshold;
        }
    }

    /// Validates a node based on its stake and activity
    pub fn validate_node(&self, registry: &ValidatorRegistry, node_id: &str) -> bool {
        if let Some(policy) = self.get_policy() {
            if let Ok(node) = registry.get_node(node_id) {
                return node.staked_btcz >= policy.minimum_stake
                    && node.active
                    && node.activity_percentage >= policy.activity_threshold;
            }
        }
        false
    }

    /// Adjusts the voting power of a validator node based on compliance and stake
    pub fn adjust_voting_power(&self, node: &ValidatorNode) -> u64 {
        if let Some(policy) = self.get_policy() {
            let stake_factor = (node.staked_btcz as f64 / policy.minimum_stake as f64).min(1.0);

            let compliance_factor = if node.compliance_percentage > 0 {
                (node.compliance_percentage as f64 / policy.compliance_threshold as f64).min(1.0)
            } else {
                0.0
            };

            if node.active {
                // Combine stake and compliance factors to calculate voting power
                (stake_factor * compliance_factor * 100.0) as u64
            } else {
                0
            }
        } else {
            0
        }
    }
}

/// A helper struct for adjusting voting power explicitly
pub struct VotingPowerAdjustment;

impl VotingPowerAdjustment {
    /// Adjusts the voting power of a node based on a compliance threshold
    pub fn adjust(node: &ValidatorNode, threshold: u64) -> u64 {
        let compliance_percentage = node.compliance_percentage;

        if compliance_percentage >= threshold {
            node.voting_power
        } else {
            // Apply proportional reduction based on compliance
            node.voting_power * compliance_percentage / 100
        }
    }
}
