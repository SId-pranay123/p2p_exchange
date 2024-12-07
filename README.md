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
1. Send an instantiation message to deploy the contract:
   ```json
   {
     "admin": "optional_admin_address"
   }
   ```

### Execute Messages 
1. Create Offer
   ```json
   {
      "create_offer": {
         "crypto_type": "BTC",
         "amount": 1000,
         "price_per_unit": 50000,
         "payment_methods": "BankTransfer",
         "min_trade_limit": 100,
         "max_trade_limit": 1000
      }
	}
   ```
2. Initiate Trade 
   ```json
   {
	  "initiate_trade": {
	    "offer_id": 1,
	    "amount": 200,
	    "payment_method": "BankTransfer"
	  }
	}
   ```

3. Confirm Paymet 
   ```json
   {
	  "confirm_payment": {
	    "trade_id": 1
	  }
	}
   ```

4. Release Fund
   ```json 
   {
	  "release_funds": {
	    "trade_id": 1
	  }
	}
   ```

5. Dispute Trade
   ```json
   {
	  "dispute_trade": {
	    "trade_id": 1,
	    "reason": "Buyer claims non-receipt of crypto."
	  }
	}
   ```

6. Resolve Dispute 
   ```json
   {
	  "resolve_dispute": {
	    "trade_id": 1,
	    "winner": "seller"
	  }
	}
   ```

### Query Messages 

1. Get config
   ```json 
   {
	  "get_config": {}
	}
   ```

2. Get Offer
   ```json
   {
	  "get_offer": {
	    "offer_id": 1
	  }
	}
   ```

3. List Offers 
   ```json
   {
	  "list_offers": {
	    "start_after": null,
	    "limit": 10
	  }
	}
   ```

4. Get Trade 
   ```json
   {
	  "get_trade": {
	    "trade_id": 1
	  }
	}
   ```

5. List Trades By Users
   ```json
   {
	  "list_trades_by_user": {
	    "user": "buyer_address",
	    "start_after": null,
	    "limit": 10
	  }
	}
   ```

## Testing 
Unit tests are included in contract.rs. To run the tests:
   ```bash
   cargo test
   ```

## Contract State

1. Config 
   Stores the admin address of the contract.

2. Offers
   Stores cryptocurrency offers by sellers.

3. Trades
   Tracks trades initiated by buyers against seller offers.


## Error Handling

The contract includes custom error handling for:
- Unauthorized actions.
- Invalid trade amounts or statuses.
- Trade and offer not found errors.

## Pagination 

Query responses like ListOffers and ListTradesByUser support optional pagination with start_after and limit parameters.



