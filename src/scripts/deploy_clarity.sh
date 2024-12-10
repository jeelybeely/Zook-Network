# File: scripts/deploy_clarity.sh

#!/bin/bash

# Clarity contract deployment script

# Configurations
TESTNET_URL="http://localhost:20443"
DEPLOYER_KEY="your-private-key"

# Contracts to deploy
CONTRACTS=(
    "clarity/zbtcz.clar"
    "clarity/governance.clar"
    "clarity/bridge.clar"
)

# Deploy each contract
for CONTRACT in "${CONTRACTS[@]}"; do
    echo "Deploying $CONTRACT..."
    clarinet contract deploy --network=testnet --key=$DEPLOYER_KEY $CONTRACT
    if [ $? -ne 0 ]; then
        echo "Failed to deploy $CONTRACT"
        exit 1
    fi
    echo "$CONTRACT deployed successfully!"
done

echo "All contracts deployed successfully!"
