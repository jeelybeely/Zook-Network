use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BridgeLedger {
    pub locked_tokens: HashMap<String, u64>, // Address and amount locked
    pub burned_tokens: HashMap<String, u64>, // Address and amount burned
}

impl BridgeLedger {
    pub fn new() -> Self {
        Self {
            locked_tokens: HashMap::new(),
            burned_tokens: HashMap::new(),
        }
    }

    pub fn lock_tokens(&mut self, address: String, amount: u64) -> Result<(), String> {
        if amount == 0 {
            return Err("Amount must be greater than zero".to_string());
        }

        let entry = self.locked_tokens.entry(address.clone()).or_insert(0);
        *entry += amount;

        println!("Tokens locked: {} -> {}", address, *entry);
        Ok(())
    }

    pub fn burn_tokens(&mut self, address: String, amount: u64) -> Result<(), String> {
        let current_balance = self.locked_tokens.get(&address).cloned().unwrap_or(0);

        if amount > current_balance {
            return Err("Burn amount exceeds locked balance".to_string());
        }

        if amount == 0 {
            return Err("Amount must be greater than zero".to_string());
        }

        self.locked_tokens.insert(address.clone(), current_balance - amount);

        let burn_entry = self.burned_tokens.entry(address.clone()).or_insert(0);
        *burn_entry += amount;

        println!("Tokens burned: {} -> {}", address, *burn_entry);
        Ok(())
    }

    pub fn get_locked_balance(&self, address: &String) -> u64 {
        *self.locked_tokens.get(address).unwrap_or(&0)
    }

    pub fn get_burned_balance(&self, address: &String) -> u64 {
        *self.burned_tokens.get(address).unwrap_or(&0)
    }

    pub fn audit_token_flow(&self) -> (u64, u64) {
        let total_locked: u64 = self.locked_tokens.values().sum();
        let total_burned: u64 = self.burned_tokens.values().sum();
        (total_locked, total_burned)
    }
}
