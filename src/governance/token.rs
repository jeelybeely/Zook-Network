// File: src/governance/token.rs

use std::collections::HashMap;

/// Governance Token (gBTCZ)
#[derive(Debug, Clone)]
pub struct GovernanceToken {
    pub symbol: String,
    pub total_supply: u64,
    pub balances: HashMap<String, u64>,
    pub rewards_schedule: u64, // Fixed emission rate
}

impl GovernanceToken {
    /// Creates a new governance token instance
    pub fn new(symbol: &str, initial_supply: u64) -> Self {
        Self {
            symbol: symbol.to_string(),
            total_supply: initial_supply,
            balances: HashMap::new(),
            rewards_schedule: 0,
        }
    }

    /// Mints new tokens to the specified account
    pub fn mint(&mut self, account: &str, amount: u64) {
        if amount == 0 {
            panic!("Cannot mint zero tokens");
        }
        *self.balances.entry(account.to_string()).or_insert(0) += amount;
        self.total_supply += amount;
    }

    /// Burns tokens from the specified account
    pub fn burn(&mut self, account: &str, amount: u64) -> Result<(), String> {
        if amount == 0 {
            return Err("Cannot burn zero tokens".to_string());
        }

        let balance = self.balances.entry(account.to_string()).or_insert(0);
        if *balance < amount {
            return Err("Insufficient balance to burn".to_string());
        }
        *balance -= amount;
        self.total_supply -= amount;
        Ok(())
    }

    /// Retrieves the balance of a specific account
    pub fn get_balance(&self, account: &str) -> u64 {
        *self.balances.get(account).unwrap_or(&0)
    }

    /// Validates secure transactions for governance updates
    pub fn validate_governance_transaction(&self, account: &str, required_balance: u64) -> Result<(), String> {
        if self.get_balance(account) < required_balance {
            return Err("Insufficient gBTCZ balance for governance transaction".to_string());
        }
        Ok(())
    }
}
