#!/bin/bash

# Zook Network Setup Script
# This script automates the setup of the Zook Network, including node initialization, contract deployment, and bridge configuration.

# Variables
ZOOK_NETWORK_DIR="/path/to/zook-network"
CONTRACTS_DIR="$ZOOK_NETWORK_DIR/clarity_contracts"
SCRIPTS_DIR="$ZOOK_NETWORK_DIR/scripts"
LOG_FILE="$ZOOK_NETWORK_DIR/setup.log"

# BTCZ Node Variables (for state anchoring or bridging)
BTCZ_NODES=(
    "http://127.0.0.1:8232"
    "http://node2.btcz.com:8232"
    "http://node3.btcz.com:8232"
)
BTCZ_NODE_SELECTED=""

# Helper Functions
log() {
    echo "[$(date)] $1" | tee -a "$LOG_FILE"
}

error_exit() {
    log "ERROR: $1"
    exit 1
}

check_btcz_node() {
    for node in "${BTCZ_NODES[@]}"; do
        log "Checking BTCZ node at $node..."
        bitcoinz-cli -rpcconnect=$(echo "$node" | awk -F":" '{print $2}' | tr -d "/") -rpcport=$(echo "$node" | awk -F":" '{print $3}') getblockchaininfo > /dev/null 2>&1
        if [ $? -eq 0 ]; then
            BTCZ_NODE_SELECTED="$node"
            log "Connected to BTCZ node at $node."
            return 0
        fi
    done
    error_exit "No BTCZ nodes are available or reachable."
}

# Step 1: Initialize BTCZ Node
log "Initializing BTCZ Node..."
check_btcz_node

# Step 2: Deploy Clarity Contracts
log "Deploying Clarity Contracts..."
cd "$CONTRACTS_DIR" || error_exit "Failed to navigate to contracts directory."
for contract in $(find . -name "*.clar"); do
    log "Deploying $contract..."
    clarity-cli deploy "$contract" || error_exit "Deployment failed for $contract."
done
log "Clarity contracts deployed successfully."

# Step 3: Configure Bridge
log "Configuring bridge..."
cd "$SCRIPTS_DIR" || error_exit "Failed to navigate to scripts directory."
sh setup_bridge.sh || error_exit "Bridge setup script failed."
log "Bridge configured successfully."

# Step 4: Start Zook Network API
log "Starting Zook Network API..."
cd "$ZOOK_NETWORK_DIR/src/api" || error_exit "Failed to navigate to API directory."
nohup python3 api_server.py --node="$BTCZ_NODE_SELECTED" > "$ZOOK_NETWORK_DIR/api.log" 2>&1 &
log "Zook Network API started. Logs available at $ZOOK_NETWORK_DIR/api.log."

# Step 5: Verify Setup
log "Verifying setup..."
curl -s http://127.0.0.1:3030/health || error_exit "Zook Network API is not responding. Check logs for details."
log "Setup verification complete. Zook Network is operational."

# Additional Enhancements for Multi-Node Awareness
log "Starting periodic node health checks..."
while true; do
    for node in "${BTCZ_NODES[@]}"; do
        bitcoinz-cli -rpcconnect=$(echo "$node" | awk -F":" '{print $2}' | tr -d "/") -rpcport=$(echo "$node" | awk -F":" '{print $3}') getblockchaininfo > /dev/null 2>&1
        if [ $? -eq 0 ]; then
            log "BTCZ node at $node is healthy."
        else
            log "WARNING: BTCZ node at $node is unreachable."
        fi
    done
    sleep 300 # Check every 5 minutes
done &

log "Zook Network setup completed successfully."
