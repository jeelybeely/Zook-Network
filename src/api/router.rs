// File: src/api/router.rs

use warp::Filter;
use crate::storage::{keys::KeyStore, storage::Wallet};
use ed25519_dalek::PublicKey;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
struct KeypairResponse {
    public_key: String,
}

#[derive(Serialize, Deserialize)]
struct BalanceResponse {
    zbtcz: u64,
    gbtcz: u64,
}

#[derive(Serialize, Deserialize)]
struct TransactionRequest {
    public_key: String,
    zbtcz: u64,
    gbtcz: u64,
    signature: String,
}

pub fn wallet_routes(
    keystore: Arc<KeyStore>,
    wallet: Arc<Wallet>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let keystore_filter = warp::any().map(move || Arc::clone(&keystore));
    let wallet_filter = warp::any().map(move || Arc::clone(&wallet));

    // Test route for debugging
    let test_route = warp::path("test")
        .and(warp::get())
        .map(|| warp::reply::json(&"Test route working"));

    // Generate a new keypair
    let generate_keypair = warp::path("generate_keypair")
        .and(warp::get())
        .and(keystore_filter.clone())
        .map(|keystore: Arc<KeyStore>| {
            let public_key = keystore.generate_keypair();
            warp::reply::json(&KeypairResponse {
                public_key: base64::encode(public_key.as_bytes()),
            })
        });

    // Get balance
    let get_balance = warp::path("balance")
        .and(warp::get())
        .and(warp::query::<String>())
        .and(wallet_filter.clone())
        .map(|public_key_base64: String, wallet: Arc<Wallet>| {
            let public_key_bytes = base64::decode(public_key_base64).unwrap();
            let public_key = PublicKey::from_bytes(&public_key_bytes).unwrap();
            if let Some((zbtcz, gbtcz)) = wallet.get_balance(&public_key) {
                warp::reply::json(&BalanceResponse { zbtcz, gbtcz })
            } else {
                warp::reply::json(&"User not found")
            }
        });

    // Deposit tokens
    let deposit_tokens = warp::path("deposit")
        .and(warp::post())
        .and(warp::body::json())
        .and(wallet_filter.clone())
        .map(|tx: TransactionRequest, wallet: Arc<Wallet>| {
            let public_key_bytes = base64::decode(tx.public_key).unwrap();
            let public_key = PublicKey::from_bytes(&public_key_bytes).unwrap();
            wallet.deposit(&public_key, tx.zbtcz, tx.gbtcz);
            warp::reply::json(&"Deposit successful")
        });

    // Withdraw tokens
    let withdraw_tokens = warp::path("withdraw")
        .and(warp::post())
        .and(warp::body::json())
        .and(wallet_filter.clone())
        .map(|tx: TransactionRequest, wallet: Arc<Wallet>| {
            let public_key_bytes = base64::decode(tx.public_key).unwrap();
            let public_key = PublicKey::from_bytes(&public_key_bytes).unwrap();
            let signature_bytes = base64::decode(tx.signature).unwrap();
            let signature = ed25519_dalek::Signature::from_bytes(&signature_bytes).unwrap();

            match wallet.withdraw(&public_key, tx.zbtcz, tx.gbtcz, &signature) {
                Ok(_) => warp::reply::json(&"Withdraw successful"),
                Err(e) => warp::reply::json(&e),
            }
        });

    warp::path("wallet")
        .and(
            generate_keypair
                .or(get_balance)
                .or(deposit_tokens)
                .or(withdraw_tokens),
        )
        .or(test_route) // Add test route for debugging
        .with(warp::log("api_requests")) // Add logging for debugging
}
