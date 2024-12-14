use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{Filter, Rejection, Reply};
use crate::models::{ApiKey, BurnRequest, MintRequest};
use crate::ledger::BridgeLedger;
use crate::errors::CustomError;

/// Struct wrapping the bridge routes functionality
pub struct BridgeAPI {
    api_keys: HashSet<ApiKey>,
    bridge_ledger: Arc<Mutex<BridgeLedger>>,
}

impl BridgeAPI {
    /// Creates a new BridgeAPI instance
    pub fn new(api_keys: HashSet<ApiKey>, bridge_ledger: Arc<Mutex<BridgeLedger>>) -> Self {
        Self {
            api_keys,
            bridge_ledger,
        }
    }

    /// Returns the Warp filter for bridge routes
    pub fn routes(&self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let api_keys = self.api_keys.clone();
        let bridge_ledger = self.bridge_ledger.clone();

        let with_ledger = warp::any().map(move || bridge_ledger.clone());

        let mint = warp::post()
            .and(warp::path!("bridge" / "mint"))
            .and(warp::body::json())
            .and(Self::with_auth(api_keys.clone()))
            .and(with_ledger.clone())
            .and_then(Self::handle_mint);

        let burn = warp::post()
            .and(warp::path!("bridge" / "burn"))
            .and(warp::body::json())
            .and(Self::with_auth(api_keys.clone()))
            .and(with_ledger.clone())
            .and_then(Self::handle_burn);

        mint.or(burn)
    }

    fn with_auth(
        api_keys: HashSet<ApiKey>,
    ) -> impl Filter<Extract = (ApiKey,), Error = Rejection> + Clone {
        warp::header::optional("authorization")
            .and_then(move |auth_header: Option<String>| {
                let keys = api_keys.clone();
                async move {
                    match auth_header {
                        Some(header) if keys.contains(&ApiKey {
                            key: header.clone(),
                            permissions: HashSet::new(),
                        }) => Ok(ApiKey {
                            key: header,
                            permissions: HashSet::new(),
                        }),
                        _ => Err(warp::reject::custom(CustomError("Unauthorized".to_string()))),
                    }
                }
            })
    }

    async fn handle_mint(
        request: MintRequest,
        _auth: ApiKey,
        bridge_ledger: Arc<Mutex<BridgeLedger>>,
    ) -> Result<impl Reply, Rejection> {
        let ledger = bridge_ledger.lock().await;

        ledger
            .lock_btcz(request.address.clone(), request.amount)
            .await
            .map_err(|e| warp::reject::custom(CustomError(e.to_string())))?;

        Ok(warp::reply::json(&"Mint successfully processed"))
    }

    async fn handle_burn(
        request: BurnRequest,
        _auth: ApiKey,
        bridge_ledger: Arc<Mutex<BridgeLedger>>,
    ) -> Result<impl Reply, Rejection> {
        let ledger = bridge_ledger.lock().await;

        ledger
            .burn_zbtcz(
                request.address.clone(),
                request.amount,
                request.transaction_hash.clone(),
            )
            .await
            .map_err(|e| warp::reject::custom(CustomError(e.to_string())))?;

        Ok(warp::reply::json(&"Burn successfully processed"))
    }
}
