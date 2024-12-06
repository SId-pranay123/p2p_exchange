# P2P Exchange Smart Contract

This repository contains a Cosmos SDK-based smart contract for a peer-to-peer (P2P) cryptocurrency exchange. The contract allows users to create offers, initiate trades, confirm payments, release funds, and resolve disputes securely and transparently.

---

## Features

1. **Offer Creation:** Sellers can list cryptocurrency offers with specific parameters like price, amount, and payment methods.
2. **Trade Initiation:** Buyers can initiate trades based on available offers.
3. **Secure Payments:** Buyers can confirm payments and release funds to sellers via the smart contract.
4. **Dispute Resolution:** Both buyers and sellers can raise disputes, which admins can resolve.
5. **Escrow:** The smart contract securely holds funds until the trade is completed or disputed.

---

## Contract Files

### 1. `state.rs`

This file defines the following:
- Persistent storage schemas (offers, trades, and configuration).
- Data structures for offers, trades, and their statuses.

### 2. `msg.rs`

Defines:
- Messages for instantiation, execution, and query operations.
- Responses for query results like offers, trades, and configuration.

### 3. `contract.rs`

Implements:
- Instantiate, execute, and query handlers.
- Core logic for offer creation, trade initiation, payment confirmation, fund release, and dispute resolution.
- Utility functions for admin validation and state updates.

---

## Installation

### Prerequisites
- Rust with `wasm32-unknown-unknown` target installed.
- [CosmWasm](https://github.com/CosmWasm/cosmwasm) environment setup.

### Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/SId-pranay123/p2p_exchange.git
   cd p2p_exchange
   ```
2. Build Schema and Contract: 
   ```bash
   cargo schema
   cargo wasm 
   ```
3. Optimize the WASM file:
   ```bash
   docker run --rm -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/rust-optimizer:0.12.6
    ```

## Usage 

### Instantiate the Contract

...To be Added 


