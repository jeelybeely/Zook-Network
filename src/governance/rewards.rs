// File: src/governance/rewards.rs

use std::time::{Duration, Instant};

pub struct RewardManager {
    activation_delay: Duration,
    distribution_frequency: Duration,
    unstaking_period: Duration,
    last_distribution: Option<Instant>,
}

impl RewardManager {
    pub fn new() -> Self {
        Self {
            activation_delay: Duration::from_secs(24 * 60 * 60), // 24 hours
            distribution_frequency: Duration::from_secs(8 * 60 * 60), // 8 hours
            unstaking_period: Duration::from_secs(48 * 60 * 60), // 48 hours
            last_distribution: None,
        }
    }

    pub fn start_rewards(&mut self) {
        self.last_distribution = Some(Instant::now() + self.activation_delay);
    }

    pub fn distribute_rewards(&mut self) {
        if let Some(last) = self.last_distribution {
            if last.elapsed() >= self.distribution_frequency {
                println!("Rewards distributed.");
                self.last_distribution = Some(Instant::now());
            } else {
                println!("Not yet time for distribution.");
            }
        } else {
            println!("Rewards not activated yet.");
        }
    }

    pub fn start_unstaking(&self) {
        println!("Unstaking started. Will complete after {:?}.", self.unstaking_period);
    }
}
