use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use p2p_exchange::msg::{ConfigResponse, OfferResponse, OffersListResponse, TradeResponse, TradesListResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use p2p_exchange::state::{Config, Offer, Trade};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(Config), &out_dir);
    export_schema(&schema_for!(Offer), &out_dir);
    export_schema(&schema_for!(Trade), &out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);
    export_schema(&schema_for!(OfferResponse), &out_dir);
    export_schema(&schema_for!(OffersListResponse), &out_dir);
    export_schema(&schema_for!(TradeResponse), &out_dir);
    export_schema(&schema_for!(TradesListResponse), &out_dir);
}
