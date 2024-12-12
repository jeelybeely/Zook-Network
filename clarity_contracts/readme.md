# Clarity Contracts

This directory contains the Clarity smart contracts that power the Zook Network. Each module in this directory handles a specific aspect of the Layer 2 functionality, including bridging, governance, and token management.

## **Subdirectories**

### 1. `bridge`
- **Purpose**: Manages the interaction between BitcoinZ (BTCZ) and the Zook Network.
- **Key Contracts**:
  - Lock BTCZ and mint `zBTCZ` on L2.
  - Burn `zBTCZ` and release BTCZ on L1.

### 2. `governance`
- **Purpose**: Implements decentralized governance mechanisms for the Zook Network.
- **Key Contracts**:
  - Proposal submission, voting, and execution.
  - Dynamic adjustment of network parameters.

### 3. `tokens`
- **Purpose**: Manages token standards and operations on the Zook Network.
- **Key Contracts**:
  - `zBTCZ` token implementation.
  - Support for additional token standards as needed.

## **How to Use**

1. **Deploy Contracts**:
   - Use deployment scripts in the `scripts` directory to deploy individual modules.

2. **Interaction**:
   - Call contract functions via Clarity CLI or the provided API endpoints in `src/api`.

3. **Extending Functionality**:
   - Developers can add new contracts by creating modules under this directory.
   - Follow the existing modular structure to maintain compatibility.

## **Tips for Developers**
- Ensure all new contracts are tested thoroughly before deployment.
- Align new modules with the existing governance and token standards.
- Document any custom additions for ease of maintenance.

For detailed usage, refer to the individual subdirectory `README.md` files.


