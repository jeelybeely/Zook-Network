use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Represents an API key with permissions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApiKey {
    pub key: String,
    pub permissions: HashSet<String>,
}

/// Represents a request to mint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintRequest {
    pub address: String,
    pub amount: u64,
}

/// Represents a request to burn
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnRequest {
    pub address: String,
    pub amount: u64,
    pub transaction_hash: String,
}
