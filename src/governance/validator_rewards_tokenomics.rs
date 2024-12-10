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

pub struct ValidatorRewardsTokenomics {
    pub reward_rate: u64, // Reward rate per cycle in gBTCZ
    pub validator_records: Arc<Mutex<HashMap<String, ValidatorRewardRecord>>>,
}

impl ValidatorRewardsTokenomics {
    pub fn new(reward_rate: u64) -> Self {
        Self {
            reward_rate,
            validator_records: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_validator(&self, address: String) -> Result<(), String> {
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

    pub fn distribute_rewards(&self) -> Result<(), String> {
        let mut records = self.validator_records.lock().map_err(|_| "Mutex lock failed")?;
        let now = Utc::now();

        for record in records.values_mut() {
            record.rewards_earned += self.reward_rate;
            record.last_reward_time = now;
        }

        Ok(())
    }

    pub fn adjust_reward_rate(&mut self, new_rate: u64) -> Result<(), String> {
        self.reward_rate = new_rate;
        Ok(())
    }

    pub fn get_validator_rewards(&self, address: &str) -> Option<ValidatorRewardRecord> {
        let records = self.validator_records.lock().ok()?;
        records.get(address).cloned()
    }
}
