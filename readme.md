# Hash-Locked Envelopes with Vesting (Soroban Smart Contract)

## Project Description

Hash-Locked Envelopes is a decentralized smart contract solution built on the Stellar blockchain using Soroban SDK. This innovative platform enables secure, time-locked, and cryptographically protected asset transfers with optional vesting schedules. The system provides a transparent, trustless way to lock funds in "envelopes" that can only be opened by beneficiaries who possess the correct secret, creating a powerful tool for inheritance planning, scheduled payments, and conditional asset transfers.

## Project Vision

Our vision is to revolutionize digital inheritance and conditional payments by providing a secure, decentralized mechanism that ensures assets reach intended beneficiaries under specific conditions. We aim to:

- **Democratize Asset Protection**: Enable anyone to create cryptographically secure inheritance plans without expensive legal intermediaries
- **Enhance Privacy**: Use hash-locked secrets to maintain confidentiality while ensuring verifiable claims
- **Promote Financial Security**: Provide time-locked and vesting mechanisms for controlled, scheduled asset distribution
- **Build Trust Through Transparency**: Leverage blockchain technology to provide verifiable, tamper-proof envelope records
- **Enable Conditional Transfers**: Allow asset transfers contingent on secret knowledge and time conditions

## Key Features

### 1. Hash-Locked Security
- Each envelope is secured with a cryptographic hash of a secret
- Only beneficiaries with the correct secret preimage can claim funds
- Prevents unauthorized access while maintaining verifiable claims

### 2. Time-Lock Mechanism
- Optional unlock timestamps prevent premature claims
- Ensures funds are released only after specified dates
- Ideal for inheritance planning and scheduled payments

### 3. Vesting Schedules
- Support for multiple vesting slices with custom unlock times and percentages
- Enables gradual fund distribution over time (e.g., 25% quarterly over 4 quarters)
- Each vesting slice is defined by timestamp and basis points (parts per 10,000)

### 4. Owner Controls
- Owners can create multiple envelopes with different beneficiaries
- Revoke unclaimed envelopes before expiry
- Set expiry timestamps for automatic refunds

### 5. Refund Mechanism
- Owners can reclaim funds from expired, unclaimed envelopes
- Prevents funds from being locked indefinitely
- Ensures capital efficiency for unused allocations

### 6. Guardian Recovery System
- Multiple guardians can be designated for emergency recovery
- Guardian voting threshold ensures multi-party agreement
- Recovery delay provides security against hasty decisions

### 7. Transparent Tracking
- View envelope details including amount, beneficiary, and claim status
- Track claimed vs. remaining amounts for vesting envelopes
- Immutable records stored on the blockchain

## Smart Contract Functions

### Initialization
```rust
initialize(owner, guardians, recovery_threshold, recovery_delay)
```
Set up the contract with owner, guardian addresses, and recovery parameters.

### Create Envelope
```rust
create_envelope(envelope_id, beneficiary, amount, secret_hash, unlock_ts, vesting, expiry_ts)
```
Owner creates a new envelope with specified parameters.

### Claim Funds
```rust
claim(envelope_id, provided_secret_hash) -> i128
```
Beneficiary claims vested funds by providing the correct secret hash.

### Revoke Envelope
```rust
revoke_envelope(envelope_id)
```
Owner revokes an unclaimed envelope.

### Refund Owner
```rust
refund_owner(envelope_id) -> i128
```
Owner reclaims funds from expired, unclaimed envelope.

### View Envelope
```rust
get_envelope(envelope_id) -> Envelope
```
Query envelope details and status.

## Use Cases

### 1. Digital Inheritance
- Parents create envelopes for children with time-locks (e.g., unlock at age 21)
- Beneficiaries receive secret via secure channels (lawyer, family member)
- Vesting ensures gradual distribution rather than lump sum

### 2. Trust Funds
- Set up multi-year vesting schedules for beneficiaries
- Automatic distribution without ongoing manual intervention
- Guardian recovery for emergencies

### 3. Conditional Payments
- Business contracts with milestone-based payments
- Release funds only when secret conditions are met
- Time-locked payments for services

### 4. Estate Planning
- Distribute assets to multiple beneficiaries with different schedules
- No need for probate or legal intermediaries
- Transparent, verifiable execution of wishes

## Future Scope

### Short-term Enhancements (3-6 months)
- **Multiple Token Support**: Enable envelopes for various Stellar assets beyond native XLM
- **Batch Creation**: Allow creation of multiple envelopes in a single transaction
- **Enhanced Secret Management**: Integrate with secure secret sharing protocols (Shamir's Secret Sharing)
- **Email Notifications**: Off-chain service to notify beneficiaries when envelopes unlock
- **Mobile Wallet Integration**: Direct integration with Freighter and other Stellar wallets

### Mid-term Development (6-12 months)
- **Multi-Signature Envelopes**: Require multiple secrets from different parties for claims
- **Conditional Logic**: Add complex unlock conditions (e.g., oracle-based triggers)
- **Template Library**: Pre-built envelope configurations for common use cases
- **Legal Documentation Integration**: Attach encrypted legal documents to envelopes
- **Analytics Dashboard**: Comprehensive UI for managing and tracking multiple envelopes
- **Beneficiary Marketplace**: Anonymous marketplace for unclaimed envelope assistance

### Long-term Vision (12+ months)
- **Cross-Chain Compatibility**: Extend to other blockchain networks beyond Stellar
- **AI-Powered Estate Planning**: Intelligent suggestions for vesting schedules and distributions
- **Decentralized Identity Integration**: Link envelopes to DIDs for enhanced privacy
- **Insurance Integration**: Partner with insurance providers for automatic claims
- **Regulatory Compliance Tools**: Tax reporting and legal compliance features
- **Smart Will Templates**: Comprehensive will creation with multiple envelope types
- **Trustee Services**: Decentralized trustee network for dispute resolution

## Technical Improvements
- **Gas Optimization**: Continuous efficiency improvements for lower transaction costs
- **Zero-Knowledge Proofs**: Enhanced privacy for secret verification
- **Enhanced Security Audits**: Regular third-party audits and bug bounties
- **SDK Development**: Libraries for easy integration in frontend applications
- **Testing Framework**: Comprehensive test suite for all edge cases
- **Documentation Hub**: Detailed guides for creators and beneficiaries

## Security Features
- **No Standard Library (`#![no_std]`)**: Reduced attack surface
- **Authentication**: Built-in `require_auth()` for all sensitive operations
- **Persistent Storage**: Instance storage with extended TTL
- **Input Validation**: Comprehensive checks on all parameters
- **Overflow Protection**: Safe arithmetic operations throughout

## Getting Started

### Prerequisites
- Rust toolchain with `wasm32-unknown-unknown` target
- Soroban CLI installed
- Stellar testnet account (Freighter wallet recommended)

### Build & Deploy
```bash
# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Deploy to testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/dead_man_project.wasm \
  --source alice \
  --network testnet

# Initialize the contract
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- initialize \
  --owner <OWNER_ADDRESS> \
  --guardians '[]' \
  --recovery_threshold 0 \
  --recovery_delay 0
```

## Contributing
Contributions are welcome! Please open issues and pull requests on GitHub.

## License
MIT License

## Contact
For questions and support, please open an issue on GitHub.

---

**Built with ❤️ on Stellar blockchain using Soroban SDK**