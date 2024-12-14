// File: src/api/bridge_finalize.rs

use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct FinalizedState {
    pub finalized_merkle_roots: Arc<Mutex<Vec<FinalizationRecord>>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FinalizationRecord {
    pub block_height: u64,
    pub merkle_root: String,
}

impl FinalizedState {
    pub fn new() -> Self {
        Self {
            finalized_merkle_roots: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_finalization(&self, record: FinalizationRecord) -> Result<(), String> {
        let mut state = self.finalized_merkle_roots.lock().map_err(|_| "Mutex lock error")?;
        if state.iter().any(|r| r.block_height == record.block_height) {
            return Err("Finalization for this block already exists".to_string());
        }
        state.push(record);
        Ok(())
    }

    pub fn get_finalizations(&self) -> Vec<FinalizationRecord> {
        self.finalized_merkle_roots.lock().unwrap_or_else(|_| vec![])
    }

    pub fn validate_merkle_root(&self, block_height: u64, merkle_root: &str) -> bool {
        let state = self.finalized_merkle_roots.lock().unwrap_or_else(|_| vec![]);
        state.iter().any(|r| r.block_height == block_height && r.merkle_root == merkle_root)
    }

    pub fn synchronize_finalization(&self, records: Vec<FinalizationRecord>) -> Result<(), String> {
        let mut state = self.finalized_merkle_roots.lock().map_err(|_| "Mutex lock error")?;
        for record in records {
            if !state.iter().any(|r| r.block_height == record.block_height) {
                state.push(record);
            }
        }
        Ok(())
    }
}

pub fn bridge_finalize_routes(finalized_state: FinalizedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let finalize = warp::path("bridge")
        .and(warp::path("finalize"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |record: FinalizationRecord| {
            let state = finalized_state.clone();
            async move {
                match state.add_finalization(record) {
                    Ok(_) => Ok(warp::reply::json(&{"status": "success", "message": "Finalization added"})),
                    Err(e) => Ok(warp::reply::json(&{"status": "error", "message": e})),
                }
            }
        });

    let get_finalizations = warp::path("bridge")
        .and(warp::path("finalizations"))
        .and(warp::get())
        .map(move || {
            let state = finalized_state.get_finalizations();
            warp::reply::json(&state)
        });

    let validate_finalization = warp::path("bridge")
        .and(warp::path("validate"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |record: FinalizationRecord| {
            let state = finalized_state.clone();
            async move {
                if state.validate_merkle_root(record.block_height, &record.merkle_root) {
                    Ok(warp::reply::json(&{"status": "success", "message": "Merkle root validated"}))
                } else {
                    Ok(warp::reply::json(&{"status": "error", "message": "Invalid Merkle root"}))
                }
            }
        });

    let synchronize_finalizations = warp::path("bridge")
        .and(warp::path("synchronize"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |records: Vec<FinalizationRecord>| {
            let state = finalized_state.clone();
            async move {
                match state.synchronize_finalization(records) {
                    Ok(_) => Ok(warp::reply::json(&{"status": "success", "message": "Finalizations synchronized"})),
                    Err(e) => Ok(warp::reply::json(&{"status": "error", "message": e})),
                }
            }
        });

    finalize.or(get_finalizations).or(validate_finalization).or(synchronize_finalizations)
}
