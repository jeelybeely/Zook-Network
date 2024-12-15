use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiKey {
    pub key: String,
    pub permissions: HashSet<String>, // HashSet to define unique permissions
}

impl Hash for ApiKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state); // Hash only the key (primary identifier)
        for perm in &self.permissions {
            perm.hash(state); // Hash each permission string
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnRequest {
    pub address: String,
    pub amount: u64,
    pub transaction_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintRequest {
    pub address: String,
    pub amount: u64,
}
