// File: src/governance/validator_rewards_tokenomics.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorRewardRecord {
    pub address: String,
    pub rewards_earned: u64,
    pub last_reward_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardsDistribution {
    pub validator_rewards: HashMap<String, u64>, // Validator address -> Reward amount
}

pub struct ValidatorRewards {
    pub reward_rate: u64, // Reward rate per cycle in gBTCZ
    pub validator_records: Arc<Mutex<HashMap<String, ValidatorRewardRecord>>>,
}

impl ValidatorRewards {
    pub fn new(reward_rate: u64, _storage_path: std::path::PathBuf) -> Self {
        Self {
            reward_rate,
            validator_records: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_validator(&self, address: String, _locked_btcz: u64) -> Result<(), String> {
        let mut records = self.validator_records.lock().map_err(|_| "Mutex lock failed")?;
        if records.contains_key(&address) {
            return Err("Validator already registered".to_string());
        }

        records.insert(
            address.clone(),
            ValidatorRewardRecord {
                address,
                rewards_earned: 0,
                last_reward_time: Utc::now(),
            },
        );
        Ok(())
    }

    pub fn distribute_rewards(
        &self,
        _validators: &[crate::validator::node_registration::ValidatorNode],
    ) -> Result<RewardsDistribution, String> {
        let mut records = self.validator_records.lock().map_err(|_| "Mutex lock failed")?;
        let now = Utc::now();
        let mut distribution = HashMap::new();

        for (address, record) in records.iter_mut() {
            record.rewards_earned += self.reward_rate;
            record.last_reward_time = now;
            distribution.insert(address.clone(), self.reward_rate);
        }

        println!("Rewards distributed to validators: {:?}", distribution);
        Ok(RewardsDistribution {
            validator_rewards: distribution,
        })
    }

    pub fn slash_validator(&self, address: String, penalty: u64) -> Result<(), String> {
        let mut records = self.validator_records.lock().map_err(|_| "Mutex lock failed")?;
        if let Some(record) = records.get_mut(&address) {
            record.rewards_earned = record.rewards_earned.saturating_sub(penalty);
            println!(
                "Validator {} slashed by {} gBTCZ. Remaining rewards: {}",
                address, penalty, record.rewards_earned
            );
            Ok(())
        } else {
            Err("Validator not found".to_string())
        }
    }

    pub fn adjust_reward_rate(&mut self, new_rate: u64) -> Result<(), String> {
        self.reward_rate = new_rate;
        println!("Validator reward rate adjusted to: {}", new_rate);
        Ok(())
    }

    pub fn apply_governance_adjustment(&mut self, param: &str, value: u64) -> Result<(), String> {
        match param {
            "reward_rate" => self.adjust_reward_rate(value),
            _ => Err("Unsupported governance parameter".to_string()),
        }
    }

    pub fn get_validator_rewards(&self, address: &str) -> Option<ValidatorRewardRecord> {
        let records = self.validator_records.lock().ok()?;
        records.get(address).cloned()
    }
}
