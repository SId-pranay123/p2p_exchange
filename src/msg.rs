use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub val: String,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub enum ExecuteMsg {
//     CustomMsg { val: String },
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateOffer {
        crypto_type: String,
        amount: Uint128,
        price_per_unit: Uint128,
        payment_methods: Vec<String>,
        min_trade_limit: Option<Uint128>,
        region_restrictions: Option<Vec<String>>,
    },
    InitiateTrade {
        offer_id: String,
        amount: Uint128,
        payment_method: String,
    },
    ConfirmPayment {
        trade_id: String,
    },
    ReleaseFunds {
        trade_id: String,
    },
    DisputeTrade {
        trade_id: String,
        reason: String,
    },
    ResolveDispute {
        trade_id: String,
        winner: String,
    },
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig {},
    GetOffer { offer_id: String },
    ListOffers {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    GetTrade { trade_id: String },
    ListTradesByUser {
        user: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CustomResponse {
    val: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
