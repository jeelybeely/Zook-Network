use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApiKey {
    pub key: String,
    pub permissions: HashSet<String>, // Define allowed actions or endpoints
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintRequest {
    pub address: String,
    pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnRequest {
    pub address: String,
    pub amount: u64,
    pub transaction_hash: String,
}
