// File: src/governance/mod.rs

pub mod validator_rewards;
pub mod validator_policies;
pub mod cross_layer_governance;

use validator_rewards::{ValidatorRewards, RewardsDistribution};
use std::sync::Arc;
use std::path::PathBuf;
use crate::validator::node_registration::ValidatorNode;

pub struct GovernanceModule {
    pub rewards: Arc<ValidatorRewards>,
}

impl GovernanceModule {
    pub fn new(reward_rate: u64, storage_path: PathBuf) -> Self {
        Self {
            rewards: Arc::new(ValidatorRewards::new(reward_rate, storage_path)),
        }
    }

    pub fn register_validator(&self, address: String, locked_btcz: u64) -> Result<(), String> {
        self.rewards.register_validator(address, locked_btcz)
    }

    pub fn distribute_rewards(&self, validators: &[ValidatorNode]) -> Result<(), String> {
        match self.rewards.distribute_rewards(validators) {
            Ok(distribution) => {
                for (validator, reward) in distribution.validator_rewards {
                    println!("Distributed {} gBTCZ to validator: {}", reward, validator);
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn slash_validator(&self, address: String, penalty: u64) -> Result<(), String> {
        self.rewards.slash_validator(address, penalty)
    }
}
