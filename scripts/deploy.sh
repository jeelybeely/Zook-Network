# File: scripts/deploy.sh

#!/bin/bash

# Configuration
NETWORK="testnet"
DEPLOYER_KEY="your-private-key-here"
CONTRACTS_DIR="clarity"
API_URL="http://localhost:3030"

# Contracts to deploy
CONTRACTS=(
    "zbtcz.clar"
    "governance.clar"
)

# Function to deploy a contract
deploy_contract() {
    local contract_path=$1
    local contract_name=$(basename "$contract_path" .clar)
    echo "Deploying $contract_name..."
    clarinet contract deploy --network=$NETWORK --key=$DEPLOYER_KEY $contract_path

    if [ $? -ne 0 ]; then
        echo "Failed to deploy $contract_name. Exiting."
        exit 1
    fi
    echo "$contract_name deployed successfully."
}

# Deploy contracts
for contract in "${CONTRACTS[@]}"; do
    deploy_contract "$CONTRACTS_DIR/$contract"
done

# Initialize governance module
echo "Initializing governance module..."
curl -X POST "$API_URL/governance/init" -H "Content-Type: application/json" -d '{}' > /dev/null
if [ $? -ne 0 ]; then
    echo "Failed to initialize governance module. Exiting."
    exit 1
fi

# Initialize bridge module
echo "Initializing bridge module..."
curl -X POST "$API_URL/bridge/init" -H "Content-Type: application/json" -d '{"merkle_root": "", "validators": []}' > /dev/null
if [ $? -ne 0 ]; then
    echo "Failed to initialize bridge module. Exiting."
    exit 1
fi

echo "All components deployed and initialized successfully!"
