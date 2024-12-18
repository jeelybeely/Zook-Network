use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;
use std::sync::Mutex as StdMutex;
use warp::{Filter, Rejection, Reply};
use crate::models::{ApiKey, BurnRequest, MintRequest};
use crate::ledger::BridgeLedger;
use crate::errors::CustomError;

// Adapter Layer for Mutex
#[derive(Clone)]
pub enum MutexAdapter<T: Clone> {
    Tokio(Arc<TokioMutex<T>>),
    Std(Arc<StdMutex<T>>),
}

impl<T: Clone> MutexAdapter<T> {
    pub fn new_tokio(inner: T) -> Self {
        Self::Tokio(Arc::new(TokioMutex::new(inner)))
    }

    pub fn new_std(inner: T) -> Self {
        Self::Std(Arc::new(StdMutex::new(inner)))
    }

    pub fn as_tokio(&self) -> Option<Arc<TokioMutex<T>>> {
        if let Self::Tokio(tokio_mutex) = self {
            Some(tokio_mutex.clone())
        } else {
            None
        }
    }
}

pub struct BridgeAPI {
    api_keys: HashSet<ApiKey>,
    bridge_ledger: MutexAdapter<BridgeLedger>,
}

impl BridgeAPI {
    pub fn new(api_keys: HashSet<ApiKey>, bridge_ledger: MutexAdapter<BridgeLedger>) -> Self {
        Self {
            api_keys,
            bridge_ledger,
        }
    }

    pub fn routes(&self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let api_keys = self.api_keys.clone();
        let bridge_ledger = self.bridge_ledger.clone();

        let with_ledger = warp::any().map(move || bridge_ledger.clone());

        let mint = warp::post()
            .and(warp::path!("bridge" / "mint"))
            .and(warp::body::json())
            .and(Self::with_auth(api_keys.clone()))
            .and(with_ledger.clone())
            .and_then(|req: MintRequest, auth: ApiKey, ledger: MutexAdapter<BridgeLedger>| async move {
                if let Some(tokio_ledger) = ledger.as_tokio() {
                    BridgeAPI::handle_mint(req, auth, tokio_ledger).await
                } else {
                    Err(warp::reject::custom(CustomError("Invalid Mutex type".to_string())))
                }
            });

        let burn = warp::post()
            .and(warp::path!("bridge" / "burn"))
            .and(warp::body::json())
            .and(Self::with_auth(api_keys.clone()))
            .and(with_ledger.clone())
            .and_then(|req: BurnRequest, auth: ApiKey, ledger: MutexAdapter<BridgeLedger>| async move {
                if let Some(tokio_ledger) = ledger.as_tokio() {
                    BridgeAPI::handle_burn(req, auth, tokio_ledger).await
                } else {
                    Err(warp::reject::custom(CustomError("Invalid Mutex type".to_string())))
                }
            });

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
                        }) => Ok::<_, Rejection>(ApiKey {
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
        bridge_ledger: Arc<TokioMutex<BridgeLedger>>,
    ) -> Result<impl Reply, Rejection> {
        let mut ledger = bridge_ledger.lock().await;

        ledger
            .lock_btcz(request.address.clone(), request.amount)
            .await
            .map_err(|e| warp::reject::custom(CustomError(e.to_string())))?;

        Ok(warp::reply::json(&"Mint successfully processed"))
    }

    async fn handle_burn(
        request: BurnRequest,
        _auth: ApiKey,
        bridge_ledger: Arc<TokioMutex<BridgeLedger>>,
    ) -> Result<impl Reply, Rejection> {
        let mut ledger = bridge_ledger.lock().await;

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
