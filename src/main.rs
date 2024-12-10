// File: src/main.rs

mod governance;
mod bridge;
mod api;
mod rpc;

use governance::GovernanceModule;
use bridge::BridgeModule;
use api::governance_api::GovernanceAPI;
use api::bridge::BridgeAPI;
use rpc::server::RpcServer;

#[tokio::main]
async fn main() {
    println!("Starting the Zook Network...");

    // Initialize Governance Module
    let mut governance_module = GovernanceModule::new();
    governance_module.start_rewards();

    // Initialize Bridge Module
    let transactions = vec!["tx1".to_string(), "tx2".to_string()];
    let bridge_module = BridgeModule::new(transactions);

    // Initialize APIs
    let governance_api = GovernanceAPI::new(governance_module);
    let bridge_api = BridgeAPI::new(bridge_module);

    // Start the RPC Server
    let rpc_server = RpcServer::new(governance_api, bridge_api);
    rpc_server.start().await;
}