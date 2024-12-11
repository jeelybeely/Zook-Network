# File: scripts/deploy_clarity_contracts.sh

# Deploy governance contract
clarinet deploy contracts/governance.clar governance
if [ $? -ne 0 ]; then
  echo "Failed to deploy governance contract"
  exit 1
fi

# Deploy bridge contract
clarinet deploy contracts/bridge.clar bridge
if [ $? -ne 0 ]; then
  echo "Failed to deploy bridge contract"
  exit 1
fi

# Deploy validator rewards contract
clarinet deploy contracts/validator_rewards.clar validator-rewards
if [ $? -ne 0 ]; then
  echo "Failed to deploy validator rewards contract"
  exit 1
fi

echo "All Clarity contracts deployed successfully!"
