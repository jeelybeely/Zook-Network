# Clarity Contracts - Tokens

The `tokens` module under `clarity_contracts` implements the token standards and operations for the Zook Network. This includes the creation and management of the native `zBTCZ` and `gBTCZ` tokens, which are integral to the platform.

## **Purpose**
The `tokens` contracts provide the following functionalities:

1. **zBTCZ Token Management**:
   - Implements the wrapped version of BTCZ (`zBTCZ`), used for transactions and cross-layer operations.

2. **gBTCZ Governance Token**:
   - Implements the `gBTCZ` token, used for voting and governance within the Zook Network.

3. **Utility Functions**:
   - Includes helper functions for token transfers, approvals, and balance management.

## **Key Contracts**
- **`zbtcz.clar`**:
  - Implements the core functionality of the `zBTCZ` token.
- **`gbtcz.clar`**:
  - Manages the `gBTCZ` governance token.

## **How It Works**
1. **zBTCZ Transactions**:
   - Users can transfer `zBTCZ` between accounts using the `zbtcz.clar` contract.

2. **gBTCZ Governance**:
   - Voting power is determined by the balance of `gBTCZ` held by users, managed through `gbtcz.clar`.

3. **Integration with Bridge and Governance**:
   - The `zBTCZ` and `gBTCZ` tokens integrate with the `bridge` and `governance` modules to enable seamless cross-layer operations and decision-making.

## **Custom Token Creation**
Custom token creation is **not supported** on the Zook Network. This restriction ensures the platform focuses on enhancing BTCZ’s ecosystem while maintaining security and governance integrity. Only predefined tokens such as `zBTCZ` and `gBTCZ` are supported.

## **Usage**
- **Deploying Contracts**:
  - Use the `scripts/deploy_contracts.sh` script to deploy the token contracts.
- **Token Transfers**:
  - Call the `transfer` function in `zbtcz.clar` or `gbtcz.clar` with sender, recipient, and amount details.

## **Tips for Developers**
- **Testing**:
  - Validate all token operations with unit tests to ensure correctness.
- **Extensibility**:
  - Ensure any future token additions align with the platform’s governance and security standards.
- **Security**:
  - Ensure token contracts are resistant to common vulnerabilities such as reentrancy or overflow errors.

For additional details, refer to the `README.md` in the root `clarity_contracts` directory.


