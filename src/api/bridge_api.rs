use crate::bridge::bridge_logic::BridgeLedger;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use warp::Filter;

#[derive(Deserialize)]
struct LockRequest {
    amount: u64,
    sender_address: String,
}

#[derive(Serialize)]
struct LockResponse {
    success: bool,
    message: String,
    locked_balance: u64,
}

#[derive(Deserialize)]
struct BurnRequest {
    amount: u64,
    sender_address: String,
}

#[derive(Serialize)]
struct BurnResponse {
    success: bool,
    message: String,
    burned_balance: u64,
}

#[derive(Serialize)]
struct AuditResponse {
    success: bool,
    total_locked: u64,
    total_burned: u64,
}

#[derive(Clone)]
pub struct BridgeAPI {
    ledger: Arc<Mutex<BridgeLedger>>,
}

impl BridgeAPI {
    pub fn new(ledger: Arc<Mutex<BridgeLedger>>) -> Self {
        Self { ledger }
    }

    pub fn routes(&self) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let api = warp::path("bridge");

        // Lock tokens endpoint
        let lock_tokens = {
            let ledger = self.ledger.clone();
            warp::path("lock")
                .and(warp::post())
                .and(warp::body::json())
                .and_then(move |req: LockRequest| {
                    let ledger = ledger.clone();
                    async move {
                        let mut ledger = ledger.lock().unwrap();
                        match ledger.lock_tokens(req.sender_address.clone(), req.amount) {
                            Ok(_) => Ok::<_, warp::Rejection>(warp::reply::json(&LockResponse {
                                success: true,
                                message: "Tokens locked successfully".to_string(),
                                locked_balance: ledger.get_locked_balance(&req.sender_address),
                            })),
                            Err(e) => Ok(warp::reply::json(&LockResponse {
                                success: false,
                                message: e,
                                locked_balance: 0,
                            })),
                        }
                    }
                })
        };

        // Burn tokens endpoint
        let burn_tokens = {
            let ledger = self.ledger.clone();
            warp::path("burn")
                .and(warp::post())
                .and(warp::body::json())
                .and_then(move |req: BurnRequest| {
                    let ledger = ledger.clone();
                    async move {
                        let mut ledger = ledger.lock().unwrap();
                        match ledger.burn_tokens(req.sender_address.clone(), req.amount) {
                            Ok(_) => Ok::<_, warp::Rejection>(warp::reply::json(&BurnResponse {
                                success: true,
                                message: "Tokens burned successfully".to_string(),
                                burned_balance: ledger.get_burned_balance(&req.sender_address),
                            })),
                            Err(e) => Ok(warp::reply::json(&BurnResponse {
                                success: false,
                                message: e,
                                burned_balance: 0,
                            })),
                        }
                    }
                })
        };

        // Audit token flow endpoint
        let audit_tokens = {
            let ledger = self.ledger.clone();
            warp::path("audit")
                .and(warp::get())
                .and_then(move || {
                    let ledger = ledger.clone();
                    async move {
                        let ledger = ledger.lock().unwrap();
                        let (total_locked, total_burned) = ledger.audit_token_flow();
                        Ok::<_, warp::Rejection>(warp::reply::json(&AuditResponse {
                            success: true,
                            total_locked,
                            total_burned,
                        }))
                    }
                })
        };

        // Combine routes
        api.and(lock_tokens.or(burn_tokens).or(audit_tokens))
    }
}
