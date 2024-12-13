// File: src/governance/mod.rs

pub mod governance_scalability;
pub mod validator_rewards;
pub mod validator_policies;

use validator_rewards::{ValidatorRewards};
use std::sync::Arc;

pub struct GovernanceModule {
    pub rewards: Arc<ValidatorRewards>,
}

impl GovernanceModule {
    pub fn new(reward_rate: u64) -> Self {
        Self {
            rewards: Arc::new(ValidatorRewards::new(reward_rate)),
        }
    }

    pub fn register_validator(&self, address: String, locked_btcz: u64) -> Result<(), String> {
        self.rewards.register_validator(address, locked_btcz)
    }

    pub fn distribute_rewards(&self) -> Result<(), String> {
        match self.rewards.distribute_rewards() {
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
