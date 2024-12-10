# Zook-Network
Zook Network. L2 for BitcoinZ. In-Progress

# Zook Network Deployment Documentation

## Overview
The Zook Network is designed as an L2 solution leveraging BTCZ as the finality layer and a custom governance token (`gBTCZ`) for incentivizing validators. This document outlines the deployment steps and operational guidelines for the system.

---

## Deployment Pipeline

### Prerequisites
- Ensure `clarinet` is installed for deploying smart contracts.
- Confirm the availability of a configured Stacks testnet endpoint.
- Replace placeholder values (`your-private-key-here`) with actual deployment keys.

### Deployment Steps
1. **Deploy Smart Contracts**:
   Run the `deploy.sh` script to deploy smart contracts and initialize the system:
   ```bash
   ./scripts/deploy.sh
   ```

   This script:
   - Deploys the `zbtcz.clar` and `governance.clar` contracts.
   - Initializes the governance module.
   - Sets up the bridge module for event synchronization.

2. **Verify API Endpoints**:
   After deployment, test API endpoints to ensure the system is functioning:
   ```bash
   curl -X GET http://localhost:3030/governance/parameters
   curl -X GET http://localhost:3030/bridge/events
   ```

---

## Component Details

### Governance Module
- **Purpose**: Manages proposals, voting, and parameter adjustments.
- **Endpoints**:
  - `/governance/propose`: Create a proposal.
  - `/governance/vote`: Vote on a proposal.
  - `/governance/execute`: Execute a passed proposal.

### Bridge Module
- **Purpose**: Synchronizes events and state between L1 and L2.
- **Endpoints**:
  - `/bridge/burn`: Validate and record burn transactions.
  - `/bridge/sync-event`: Synchronize cross-layer events.
  - `/bridge/events`: Retrieve event records.

---

## Pre-Production Checklist

### Functional Tests
- [ ] Validate smart contract deployment using `clarinet`.
- [ ] Verify API endpoints for governance and bridge modules.
- [ ] Simulate cross-layer events and governance proposals.

### Security Checks
- [ ] Review and apply slashing rules for inactive or malicious validators.
- [ ] Ensure all API inputs are sanitized to prevent injection attacks.

### Performance Validation
- [ ] Benchmark API response times under load.
- [ ] Simulate multiple validators and governance proposals.

### Monitoring Setup
- [ ] Enable logging for all API endpoints.
- [ ] Configure alerts for failed transactions or synchronization issues.

---

## Post-Deployment Monitoring
- Use logs to monitor API and event synchronization activity.
- Periodically validate the state consistency between BTCZ (L1) and Zook (L2).
- Audit governance proposals and validator performance.

---

## Troubleshooting
- **Failed Contract Deployment**: Verify the network configuration and deployer keys.
- **API Connectivity Issues**: Check the server logs for errors.
- **Event Sync Delays**: Validate the Merkle proofs and synchronization endpoints.

---

## Contact
For support, contact the Zook Network development team at [support@zooknetwork.io](mailto:support@zooknetwork.io).


