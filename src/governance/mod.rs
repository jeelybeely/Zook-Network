// File: src/governance/mod.rs

pub mod rewards;
pub mod token;

use rewards::RewardManager;

pub struct GovernanceModule {
    pub reward_manager: RewardManager,
}

impl GovernanceModule {
    pub fn new() -> Self {
        Self {
            reward_manager: RewardManager::new(),
        }
    }

    pub fn start_rewards(&mut self) {
        self.reward_manager.start_rewards();
    }

    pub fn distribute_rewards(&mut self) {
        self.reward_manager.distribute_rewards();
    }

    pub fn start_unstaking(&self) {
        self.reward_manager.start_unstaking();
    }
}