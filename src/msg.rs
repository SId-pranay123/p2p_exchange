// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};
// use crate::state::Offer;

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub struct InstantiateMsg {
//     pub admin: Option<String>,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub enum ExecuteMsg {
//     CreateOffer {
//         crypto_type: String,
//         amount: u128,
//         price_per_unit: u128,
//         payment_methods: String,
//         min_trade_limit: u128,
//         max_trade_limit: u128,
//     },
//     InitiateTrade {
//         offer_id: String,
//         amount: u128,
//         payment_method: String,
//     },
//     ConfirmPayment {
//         trade_id: String,
//     },
//     ReleaseFunds {
//         trade_id: String,
//     },
//     DisputeTrade {
//         trade_id: String,
//         reason: String,
//     },
//     ResolveDispute {
//         trade_id: String,
//         winner: String,
//     },
// }


// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub enum QueryMsg {
//     GetConfig {},
//     GetOffer { offer_id: String },
//     ListOffers {
//         start_after: Option<String>,
//         limit: Option<u32>,
//     },
//     GetTrade { trade_id: String },
//     ListTradesByUser {
//         user: String,
//         start_after: Option<String>,
//         limit: Option<u32>,
//     },
// }


// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub struct AllOffersResponse {
//     pub offers: Vec<Offer>,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub enum MigrateMsg {}



use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::Offer;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateOffer {
        crypto_type: String,
        amount: u128,
        price_per_unit: u128,
        payment_methods: String,
        min_trade_limit: u128,
        max_trade_limit: u128,
    },
    InitiateTrade {
        offer_id: u64,
        amount: u128,
        payment_method: String,
    },
    ConfirmPayment {
        trade_id: u64,
    },
    ReleaseFunds {
        trade_id: u64,
    },
    DisputeTrade {
        trade_id: u64,
        reason: String,
    },
    ResolveDispute {
        trade_id: u64,
        winner: String,
    },
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig {},
    GetOffer { offer_id: u64 },
    ListOffers {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    GetTrade { trade_id: u64 },
    ListTradesByUser {
        user: String,
        start_after: Option<u64>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub admin: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OfferResponse {
    pub offer: Offer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OffersListResponse {
    pub offers: Vec<(u64, Offer)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TradeResponse {
    pub trade_id: u64,
    pub trade: crate::state::Trade,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TradesListResponse {
    pub trades: Vec<(u64, crate::state::Trade)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum MigrateMsg {}
