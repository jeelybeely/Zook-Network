#!/bin/bash

# Zook Network Testing Script
# This script automates testing of the Zook Network, including API endpoints, RPC commands, and contract functionality.

# Variables
API_ENDPOINT="http://127.0.0.1:3030"
BTCZ_RPC_ENDPOINT="http://127.0.0.1:8232"
LOG_FILE="./testing.log"
CONTRACTS_DIR="/path/to/zook-network/clarity_contracts"

# Helper Functions
log() {
    echo "[$(date)] $1" | tee -a "$LOG_FILE"
}

error_exit() {
    log "ERROR: $1"
    exit 1
}

# Step 1: Test BTCZ Node Connection
log "Testing BTCZ Node connection..."
bitcoinz-cli -rpcconnect=127.0.0.1 -rpcport=8232 getblockchaininfo > /dev/null 2>&1 || error_exit "BTCZ node is not running or unreachable."
log "BTCZ node connection successful."

# Step 2: Test API Endpoints
log "Testing Zook Network API endpoints..."
API_TESTS=(
    "/health"
    "/api/bridge/status"
    "/api/governance/proposals"
)

for endpoint in "${API_TESTS[@]}"; do
    response=$(curl -s -o /dev/null -w "%{http_code}" "$API_ENDPOINT$endpoint")
    if [ "$response" -ne 200 ]; then
        error_exit "API test failed for endpoint $endpoint (HTTP $response)."
    fi
    log "API test passed for endpoint $endpoint."
done

# Step 3: Test Clarity Contracts
log "Testing Clarity contracts..."
cd "$CONTRACTS_DIR" || error_exit "Failed to navigate to contracts directory."
for contract in $(find . -name "*.clar"); do
    log "Running tests for $contract..."
    clarity-cli test "$contract" || error_exit "Test failed for $contract."
done
log "All Clarity contract tests passed successfully."

# Step 4: Test RPC Commands
log "Testing Zook Network RPC commands..."
RPC_COMMANDS=(
    "sendanchor '{\"block_height\":100,\"state_root\":\"example\",\"merkle_proof\":[\"hash1\",\"hash2\"]}'"
    "processstateanchor \"merkle_root_example\" '[\"tx1\",\"tx2\"]'"
)

for cmd in "${RPC_COMMANDS[@]}"; do
    result=$(bitcoinz-cli -rpcconnect=127.0.0.1 -rpcport=8232 $cmd 2>&1)
    if echo "$result" | grep -q "error"; then
        error_exit "RPC command test failed: $cmd ($result)."
    fi
    log "RPC command test passed: $cmd."
done

# Step 5: End-to-End Workflow Test
log "Running end-to-end workflow test..."
log "Simulating token bridging: Lock BTCZ and mint zBTCZ..."
# Simulated commands for bridging workflow (to be replaced with real test data)
bitcoinz-cli lockbtcztokens 100 || error_exit "Failed to lock BTCZ tokens."
curl -X POST "$API_ENDPOINT/api/bridge/mint" -H "Content-Type: application/json" -d '{"amount": 100}' || error_exit "Failed to mint zBTCZ."
log "Token bridging test passed."

log "Testing completed successfully. All systems are operational."
