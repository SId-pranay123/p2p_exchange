// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};
// use cw_storage_plus::{Item, Map};
// use cosmwasm_std::Addr;

// // State structures
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct Config {
//     pub admin: Addr,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct Offer {
//     pub seller: String,
//     pub crypto_type: String,
//     pub amount: u128,
//     pub price_per_unit: u128,
//     pub payment_methods: String,
//     pub min_trade_limit: u128,
//     pub max_trade_limit: u128,
//     pub status: OfferStatus,
//     pub created_at: u64,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub enum OfferStatus {
//     Active,
//     InTrade,
//     Completed,
//     Cancelled,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct Trade {
//     pub offer_id: String,
//     pub seller: String,
//     pub buyer: String,
//     pub amount: u128,
//     pub status: TradeStatus,
//     pub payment_method: String,
//     pub created_at: u64,
//     pub payment_confirmed_at: Option<u64>,
//     pub completed_at: Option<u64>,
//     pub disputed: bool,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub enum TradeStatus {
//     Pending,
//     PaymentSent,
//     Completed,
//     Disputed,
//     Cancelled,
//     Resolved
// }

// // State
// pub const CONFIG: Item<Config> = Item::new("config");
// pub const OFFERS: Map<&str, Offer> = Map::new("offers");
// pub const TRADES: Map<&str, Trade> = Map::new("trades");


use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Item, Map};
use cosmwasm_std::Addr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum OfferStatus {
    Active,
    InTrade,
    Completed,
    Cancelled,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Offer {
    pub seller: String,
    pub crypto_type: String,
    pub amount: u128,
    pub price_per_unit: u128,
    pub payment_methods: String,
    pub min_trade_limit: u128,
    pub max_trade_limit: u128,
    pub status: OfferStatus,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum TradeStatus {
    Pending,
    PaymentSent,
    Completed,
    Disputed,
    Cancelled,
    Resolved,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Trade {
    pub offer_id: u64,
    pub seller: String,
    pub buyer: String,
    pub amount: u128,
    pub status: TradeStatus,
    pub payment_method: String,
    pub created_at: u64,
    pub payment_confirmed_at: Option<u64>,
    pub completed_at: Option<u64>,
    pub disputed: bool,
    // New field to track if buyer's escrow is funded
    pub buyer_escrow_funded: bool,  
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const OFFER_SEQ: Item<u64> = Item::new("offer_seq");
pub const TRADE_SEQ: Item<u64> = Item::new("trade_seq");

// Offers keyed by ID
pub const OFFERS: Map<u64, Offer> = Map::new("offers");

// Trades keyed by ID
pub const TRADES: Map<u64, Trade> = Map::new("trades");
