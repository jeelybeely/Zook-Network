# File: scripts/deploy_clarity.sh

#!/bin/bash

# Deployment configurations
NETWORK="testnet"
DEPLOYER_KEY="your-private-key-here"
CONTRACTS_DIR="clarity"

# Clarity contracts to deploy
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

# Deploy all contracts
for contract in "${CONTRACTS[@]}"; do
    deploy_contract "$CONTRACTS_DIR/$contract"
done

# Initialize bridge module
echo "Initializing bridge components..."
curl -X POST "http://localhost:3030/bridge/init" -H "Content-Type: application/json" -d '{"merkle_root": "", "validators": []}'
if [ $? -ne 0 ]; then
    echo "Failed to initialize bridge components. Exiting."
    exit 1
fi

echo "All contracts deployed and bridge initialized successfully!"
