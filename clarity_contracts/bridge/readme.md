# Clarity Contracts - Bridge

The `bridge` module under `clarity_contracts` contains smart contracts responsible for enabling the seamless interaction between the BitcoinZ (BTCZ) blockchain (Layer 1) and the Zook Network (Layer 2).

## **Purpose**
The `bridge` contracts implement the following core functionalities:

1. **Token Locking and Minting**:
   - Locks BTCZ on the BitcoinZ blockchain.
   - Mints equivalent `zBTCZ` tokens on the Zook Network.

2. **Token Burning and Unlocking**:
   - Burns `zBTCZ` tokens on the Zook Network.
   - Unlocks BTCZ on the BitcoinZ blockchain.

3. **Cross-Layer State Synchronization**:
   - Manages state anchoring and Merkle proof validation to ensure consistency between BTCZ and Zook.

## **Key Contracts**
- **`lock.clar`**:
  - Handles locking BTCZ and initiating the minting process on Zook.
- **`burn.clar`**:
  - Manages token burning on Zook and triggering BTCZ unlocking on L1.
- **`state_sync.clar`**:
  - Validates state roots and Merkle proofs for cross-layer synchronization.

## **How It Works**
1. **BTCZ to zBTCZ Flow**:
   - BTCZ is locked on L1 using a BTCZ Core mechanism.
   - The `lock.clar` contract processes the event and mints `zBTCZ` on L2.

2. **zBTCZ to BTCZ Flow**:
   - `zBTCZ` tokens are burned on L2 using `burn.clar`.
   - The event triggers unlocking of BTCZ on L1.

3. **State Synchronization**:
   - The `state_sync.clar` contract ensures both layers remain consistent through state anchoring and Merkle proof validation.

## **Usage**
- **Deploying Contracts**:
  - Use the `scripts/deploy_contracts.sh` script to deploy the bridge contracts.
- **Interacting with Contracts**:
  - Call the contract functions using Clarity CLI or through API endpoints defined in `src/api/bridge`.

## **Tips for Developers**
- **Testing**:
  - Ensure cross-layer consistency by testing with real payloads from BTCZ Core.
- **Extensibility**:
  - Follow the modular design to add new bridging features or optimize state synchronization.
- **Security**:
  - Validate all Merkle proofs and state roots rigorously to prevent inconsistencies.

For additional details, refer to the `README.md` in the root `clarity_contracts` directory.


