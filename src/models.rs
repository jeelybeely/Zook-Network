use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Struct representing an API Key
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiKey {
    pub key: String,
    pub permissions: HashSet<String>, // Define allowed actions or endpoints
}

impl std::hash::Hash for ApiKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state); // Use only `key` for hashing
    }
}

/// Struct for burn requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnRequest {
    pub address: String,
    pub amount: u64,
    pub transaction_hash: String,
}

/// Struct for mint requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintRequest {
    pub address: String,
    pub amount: u64,
}
