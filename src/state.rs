use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// State structures
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: String,
    pub platform_fee: Uint128,  // Fee in basis points (1/100th of a percent)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Offer {
    pub seller: String,
    pub crypto_type: String,
    pub amount: Uint128,
    pub price_per_unit: Uint128,
    pub payment_methods: Vec<String>,
    pub min_trade_limit: Option<Uint128>,
    pub region_restrictions: Option<Vec<String>>,
    pub status: OfferStatus,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum OfferStatus {
    Active,
    InTrade,
    Completed,
    Cancelled,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Trade {
    pub offer_id: String,
    pub seller: String,
    pub buyer: String,
    pub amount: Uint128,
    pub status: TradeStatus,
    pub payment_method: String,
    pub created_at: u64,
    pub payment_confirmed_at: Option<u64>,
    pub completed_at: Option<u64>,
    pub disputed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum TradeStatus {
    Pending,
    PaymentSent,
    Completed,
    Disputed,
    Cancelled,
}

// State
const CONFIG: Item<Config> = Item::new("config");
const OFFERS: Map<&str, Offer> = Map::new("offers");
const TRADES: Map<&str, Trade> = Map::new("trades");