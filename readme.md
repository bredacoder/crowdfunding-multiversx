# Crowdfunding Smart Contract

This is my **first smart contract** built using Rust and MultiversX. I followed the official [Crowdfunding Tutorial](https://docs.multiversx.com/developers/tutorials/crowdfunding-p1/) and learned a lot about how blockchain smart contracts work behind the scenes.

---

## �� Contract Overview

This smart contract implements a complete crowdfunding platform with the following features:

- **Campaign Setup**: Create a campaign with a funding target and deadline
- **Contribution System**: Accept EGLD donations from any user during the funding period
- **Status Tracking**: Track whether a campaign is active, successful, or failed
- **Fund Distribution**:
  - If successful (deadline passed & target reached): Owner can claim all funds
  - If failed (deadline passed & target not reached): Contributors can reclaim their donations
- **Contribution Tracking**: Individual contributions are tracked to enable refunds

### Key Contract Functions

- `init(target, deadline)`: Initialize the contract with target amount and deadline
- `fund()`: Contribute EGLD to the campaign
- `claim()`: Claim funds (owner) or get refunds (contributors)
- `status()`: Check current campaign status (FundingPeriod/Successful/Failed)
- `getCurrentFunds()`: Get total funds raised so far
- `getTarget()`: Get the campaign's funding target
- `getDeadline()`: Get the campaign's deadline
- `getDeposit(address)`: Check contribution amount for a specific address

---

## 🚀 Running and Testing the Contract

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [MultiversX SDK](https://docs.multiversx.com/sdk-and-tools/sdk-py/installing-sdk/)
- [mxpy](https://docs.multiversx.com/sdk-and-tools/sdk-py/installing-mxpy/)

### Building the Contract

```bash
# Clone the repository
git clone https://github.com/yourusername/crowdfunding-contract.git
cd crowdfunding-contract

# Build the contract
mxpy contract build
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test crowdfunding_blackbox_test
```

### Deploying to Devnet

```bash
# Deploy to devnet (replace path/to/wallet with your actual wallet path)
mxpy contract deploy --bytecode=output/crowdfunding.wasm \
  --pem=path/to/wallet.pem \
  --gas-limit=6000000 \
  --proxy=https://devnet-gateway.multiversx.com \
  --chain=D \
  --arguments 500000000000000000 $(( $(date +%s) + 3600 )) \
  --send
```

The deployment arguments are:
- `500000000000000000`: Target of 0.5 EGLD in atomic units
- `$(( $(date +%s) + 3600 ))`: Deadline set to 1 hour from now

### Interacting with the Contract

#### Fund the Campaign
```bash
mxpy contract call <CONTRACT_ADDRESS> \
  --pem=path/to/wallet.pem \
  --gas-limit=6000000 \
  --proxy=https://devnet-gateway.multiversx.com \
  --chain=D \
  --function=fund \
  --value=100000000000000000 \
  --send
```

#### Check Campaign Status
```bash
mxpy contract query <CONTRACT_ADDRESS> \
  --proxy=https://devnet-gateway.multiversx.com \
  --function=status
```

#### Claim Funds (after deadline)
```bash
mxpy contract call <CONTRACT_ADDRESS> \
  --pem=path/to/wallet.pem \
  --gas-limit=6000000 \
  --proxy=https://devnet-gateway.multiversx.com \
  --chain=D \
  --function=claim \
  --send
```

---

## 🧠 What I Learned
As a Web2 Full Stack Developer diving into blockchain and Rust, here's a breakdown of key concepts and insights I searched and explored while building my first smart contract — this is what I've learned so far:

### 🛡️ Smart Contract & Blockchain Concepts

- **Smart Contract** – A self-contained program that runs on the blockchain. Think of it as an immutable backend service that has its own storage and logic, and runs inside a virtual machine (VM).

- **Rust Attributes (macros)** – In MultiversX, special macros like `#[init]`, `#[view]`, and `#[endpoint]` are used to annotate your smart contract functions. They instruct the blockchain how and when those methods can be called:
  - `#[init]`: Marks the **constructor**, executed once at deployment to initialize contract state.
  - `#[endpoint]`: Marks a **public, externally-callable method** that **can change the contract's state**. Like a `POST` in HTTP – it consumes gas and changes data on-chain.
  - `#[view]`: Marks a **read-only public method**, callable for free. It doesn't change state – similar to an HTTP `GET`.

- **Storage Mapper** – Provides access to the contract's on-chain storage. It's like a strongly-typed key-value database built into the blockchain. You don't need to configure any DB, everything is handled internally.
  ```rust
  #[storage_mapper("target")]
  fn target(&self) -> SingleValueMapper<BigUint>;
  ```

- **Proxy** – A struct that mimics your smart contract's interface. It's auto-generated and allows you to call contract methods in integration tests or from other contracts.

- **Nonce** – A unique number that increments with each transaction per account. It prevents replay attacks and helps the network validate transaction order.

- **Virtual Machine (VM)** – The execution environment for smart contracts. It runs on validator nodes and isolates contract logic from the underlying blockchain protocol.

- **On-Chain** – Means that data and logic live directly on the blockchain, not on external servers. Every change is transparent, immutable, and globally verifiable.

- **Gas** – The cost of performing operations on-chain. It's like server cost in the cloud – but paid per action. Read operations (`#[view]`) are free, but write operations (`#[endpoint]`) cost gas.


> 🎉 I was amazed to see how Rust with MultiversX provides **everything built-in**: test runner, storage, contract deployment, simulation, and more – no need to worry about setting up databases, managing frameworks, or boilerplate.

## 📂 Project Structure
```
.
├── src/                        # Contract logic and proxy
│   ├── crowdfunding.rs         # Main contract implementation
│   └── crowdfunding_proxy.rs   # Auto-generated proxy for testing
├── output/                     # Compiled contract artifacts
│   └── crowdfunding.wasm       # WebAssembly binary of the contract
├── tests/                      # Blackbox tests for the contract
│   └── crowdfunding_blackbox_test.rs # Integration tests
├── sc-config.toml              # Proxy generator config
├── multiversx.json             # Contract metadata
```

---

## 🚀 Final Thoughts
This was my first experience building a smart contract, and I was surprised by how streamlined and powerful the MultiversX SDK is. As someone coming from Web2 Full Stack Development, I found the tooling and architecture around Rust + blockchain development to be clean, minimal, and enjoyable.

---

Feel free to reach out on **X (Twitter)** [@bredacoder_](https://x.com/bredacoder_) if you're learning too or want to share ideas!

