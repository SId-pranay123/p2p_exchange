// #[cfg(not(feature = "library"))]
// use cosmwasm_std::entry_point;
// use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

// use cw2::set_contract_version;

// // use crate::error::ContractError;
// use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
// use crate::state::{Config, Offer, OfferStatus, Trade, TradeStatus, CONFIG, OFFERS, TRADES};


// const CONTRACT_NAME: &str = "crates.io:p2p_exchange";
// const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn instantiate(
//     deps: DepsMut,
//     _env: Env,
//     info: MessageInfo,
//     msg: InstantiateMsg,
// ) -> StdResult<Response> {
//     set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
//     let admin = msg.admin.unwrap_or_else(|| info.sender.to_string());
//     let validated_admin = deps.api.addr_validate(&admin)?;
//     let config = Config {
//         admin: validated_admin.clone(),
//     };
//     CONFIG.save(deps.storage, &config)?;
//     Ok(Response::new()
//         .add_attribute("action", "instantiate")
//         .add_attribute("admin", validated_admin.to_string()))
// }



// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn execute(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     msg: ExecuteMsg,
// ) -> StdResult<Response> {
//     match msg {
//         ExecuteMsg::CreateOffer {
//             crypto_type,
//             amount,
//             price_per_unit,
//             payment_methods,
//             min_trade_limit,
//             max_trade_limit,
//         } => execute_create_offer(
//             deps,
//             env,
//             info,
//             crypto_type,
//             amount,
//             price_per_unit,
//             payment_methods,
//             min_trade_limit,
//             max_trade_limit
//         ),
//         ExecuteMsg::InitiateTrade {
//             offer_id,
//             amount,
//             payment_method,
//         } => execute_initiate_trade(deps, env, info, offer_id, amount, payment_method),
//         ExecuteMsg::ConfirmPayment { trade_id } => {
//             execute_confirm_payment(deps, env, info, trade_id)
//         },
//         ExecuteMsg::ReleaseFunds { trade_id } => execute_release_funds(deps, env, info, trade_id),
//         ExecuteMsg::DisputeTrade { trade_id, reason } => {
//             execute_dispute_trade(deps, env, info, trade_id, reason)
//         },
//         ExecuteMsg::ResolveDispute { trade_id, winner } => {
//             execute_resolve_dispute(deps, env, info, trade_id, winner)
//         },
//     }
// }

// fn execute_create_offer(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     crypto_type: String,
//     amount: u128,
//     price_per_unit: u128,
//     payment_methods: String ,
//     min_trade_limit: u128,
//     max_trade_limit: u128,
// ) -> StdResult<Response> {
//     let offer_id = format!("offer_{}", env.block.height);
//     let offer = Offer {
//         seller: info.sender.to_string(),
//         crypto_type,
//         amount,
//         price_per_unit,
//         payment_methods,
//         min_trade_limit,
//         max_trade_limit,
//         status: OfferStatus::Active,
//         created_at: env.block.time.seconds(),
//     };
    
//     OFFERS.save(deps.storage, &offer_id, &offer)?;
    
//     Ok(Response::new()
//         .add_attribute("action", "create_offer")
//         .add_attribute("offer_id", offer_id))
// }

// fn execute_initiate_trade(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     offer_id: String,
//     amount: u128,
//     payment_method: String,
// ) -> StdResult<Response> {
//     let trade_id = format!("trade_{}", env.block.height);
//     let _trade = Trade {
//         buyer: info.sender.to_string(),
//         seller: OFFERS.load(deps.storage, &offer_id)?.seller,
//         offer_id,
//         amount,
//         payment_method,
//         status: TradeStatus::Pending,
//         created_at: env.block.time.seconds(),
//         payment_confirmed_at: None,
//         completed_at: None,
//         disputed: false,
//     };
//     Ok(Response::new()
//         .add_attribute("action", "initiate_trade")
//         .add_attribute("trade_id", trade_id))
// }

// fn execute_confirm_payment(
//     deps: DepsMut,
//     _env: Env,
//     _info: MessageInfo,
//     trade_id: String,
// ) -> StdResult<Response> {
//     let mut trade = TRADES.load(deps.storage, &trade_id)?;
//     trade.status = TradeStatus::PaymentSent;
//     TRADES.save(deps.storage, &trade_id, &trade)?;
//     Ok(Response::new()
//         .add_attribute("action", "confirm_payment")
//         .add_attribute("trade_id", trade_id))
// }

// fn execute_release_funds(
//     deps: DepsMut,
//     _env: Env,
//     _info: MessageInfo,
//     trade_id: String,
// ) -> StdResult<Response> {
//     let mut trade = TRADES.load(deps.storage, &trade_id)?;
//     trade.status = TradeStatus::Completed;
//     TRADES.save(deps.storage, &trade_id, &trade)?;
//     Ok(Response::new()
//         .add_attribute("action", "release_funds")
//         .add_attribute("trade_id", trade_id))
// }

// fn execute_dispute_trade(
//     deps: DepsMut,
//     _env: Env,
//     _info: MessageInfo,
//     trade_id: String,
//     reason: String,
// ) -> StdResult<Response> {
//     let mut trade = TRADES.load(deps.storage, &trade_id)?;
//     trade.status = TradeStatus::Disputed;
//     TRADES.save(deps.storage, &trade_id, &trade)?;
//     Ok(Response::new()
//         .add_attribute("action", "dispute_trade")
//         .add_attribute("trade_id", trade_id)
//         .add_attribute("reason", reason))
// }

// fn execute_resolve_dispute(
//     deps: DepsMut,
//     _env: Env,
//     _info: MessageInfo,
//     trade_id: String,
//     winner: String,
// ) -> StdResult<Response> {
//     let mut trade = TRADES.load(deps.storage, &trade_id)?;
//     trade.status = TradeStatus::Resolved;
//     TRADES.save(deps.storage, &trade_id, &trade)?;
//     Ok(Response::new()
//         .add_attribute("action", "resolve_dispute")
//         .add_attribute("trade_id", trade_id)
//         .add_attribute("winner", winner))
// }



// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
//     match msg {
//         QueryMsg::GetConfig {  } => query_config(),
//         QueryMsg::GetOffer { offer_id } => query_offer(_deps, offer_id),
//         QueryMsg::ListOffers { start_after, limit } => query_offers(_deps, start_after, limit),
//         QueryMsg::GetTrade { trade_id } => query_trade(_deps, trade_id),
//         QueryMsg::ListTradesByUser { user, start_after, limit } => query_trades_by_user(_deps, user, start_after, limit),
//     }
// }


// fn query_config() -> StdResult<Binary> {
//     todo!()
// }

// fn query_offer(deps: Deps, offer_id: String) -> StdResult<Binary> {
//     todo!()
// }

// fn query_offers(deps: Deps, start_after: Option<String>, limit: Option<u32>) -> StdResult<Binary> {
//     todo!()
// }

// fn query_trade(deps: Deps, trade_id: String) -> StdResult<Binary> {
//     todo!()
// }

// fn query_trades_by_user(deps: Deps, user: String, start_after: Option<String>, limit: Option<u32>) -> StdResult<Binary> {
//     todo!()
// }

// #[cfg(test)]
// mod tests {}





#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_json_binary, Order
};
use cw2::set_contract_version;
use cw_storage_plus::Bound;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ConfigResponse, OfferResponse, OffersListResponse, TradeResponse, TradesListResponse};
use crate::state::{
    Config, Offer, OfferStatus, Trade, TradeStatus, CONFIG, OFFERS, TRADES, OFFER_SEQ, TRADE_SEQ,
};

const CONTRACT_NAME: &str = "crates.io:p2p_exchange";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Default pagination limit
const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 30;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin = msg.admin.unwrap_or_else(|| info.sender.to_string());
    let validated_admin = deps.api.addr_validate(&admin)?;

    let config = Config { admin: validated_admin.clone() };
    CONFIG.save(deps.storage, &config)?;
    OFFER_SEQ.save(deps.storage, &0)?;
    TRADE_SEQ.save(deps.storage, &0)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", validated_admin.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateOffer {
            crypto_type,
            amount,
            price_per_unit,
            payment_methods,
            min_trade_limit,
            max_trade_limit,
        } => execute_create_offer(
            deps, env, info, crypto_type, amount, price_per_unit, payment_methods, min_trade_limit, max_trade_limit
        ),
        ExecuteMsg::InitiateTrade {
            offer_id,
            amount,
            payment_method,
        } => execute_initiate_trade(deps, env, info, offer_id, amount, payment_method),
        ExecuteMsg::ConfirmPayment { trade_id } => execute_confirm_payment(deps, env, info, trade_id),
        ExecuteMsg::ReleaseFunds { trade_id } => execute_release_funds(deps, env, info, trade_id),
        ExecuteMsg::DisputeTrade { trade_id, reason } => execute_dispute_trade(deps, env, info, trade_id, reason),
        ExecuteMsg::ResolveDispute { trade_id, winner } => execute_resolve_dispute(deps, env, info, trade_id, winner),
    }
}

fn ensure_admin(deps: &DepsMut, info: &MessageInfo) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::NotAdmin {});
    }
    Ok(())
}

fn execute_create_offer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _crypto_type: String,
    amount: u128,
    price_per_unit: u128,
    payment_methods: String,
    min_trade_limit: u128,
    max_trade_limit: u128,
) -> Result<Response, ContractError> {
    if info.funds.len() != 1 {
        return Err(ContractError::Std(cosmwasm_std::StdError::generic_err("Must send exactly one type of token")));
    }
    let sent = &info.funds[0];
    if sent.amount.u128() != amount {
        return Err(ContractError::Std(cosmwasm_std::StdError::generic_err("Must send exact amount of crypto")));
    }
    if min_trade_limit > max_trade_limit || amount < min_trade_limit {
        return Err(ContractError::InvalidTradeAmount {});
    }

    let mut offer_seq = OFFER_SEQ.load(deps.storage)?;
    offer_seq += 1;
    OFFER_SEQ.save(deps.storage, &offer_seq)?;

    let offer = Offer {
        seller: info.sender.to_string(),
        crypto_type: sent.denom.clone(),
        amount,
        price_per_unit,
        payment_methods,
        min_trade_limit,
        max_trade_limit,
        status: OfferStatus::Active,
        created_at: env.block.time.seconds(),
    };

    OFFERS.save(deps.storage, offer_seq.into(), &offer)?;

    Ok(Response::new()
        .add_attribute("action", "create_offer")
        .add_attribute("offer_id", offer_seq.to_string()))
}

fn execute_initiate_trade(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    offer_id: u64,
    amount: u128,
    payment_method: String,
) -> Result<Response, ContractError> {
    let mut offer = OFFERS.may_load(deps.storage, offer_id.into())?.ok_or(ContractError::OfferNotFound {})?;
    if offer.status != OfferStatus::Active {
        return Err(ContractError::OfferNotActive {});
    }
    if amount < offer.min_trade_limit || amount > offer.max_trade_limit || amount > offer.amount {
        return Err(ContractError::InvalidTradeAmount {});
    }

    // Calculate payment required from buyer
    let required_payment = amount.checked_mul(offer.price_per_unit).ok_or(ContractError::InvalidTradeAmount {})?;

    // Check buyer funds
    if info.funds.len() != 1 {
        return Err(ContractError::Std(cosmwasm_std::StdError::generic_err("Must send exactly one type of token as payment")));
    }

    let payment = &info.funds[0];
    // The buyer must send the right token type (e.g., stable token) 
    // Let's assume the buyer pays in "ust" or some known denom. For simplicity, we just require correct amount:
    if payment.amount.u128() != required_payment {
        return Err(ContractError::Std(cosmwasm_std::StdError::generic_err("Incorrect payment amount sent")));
    }

    // Deduct the trading amount from the offer
    offer.amount -= amount;
    // If after this trade initiation, offer has no more amount left, or we want to consider it "in trade"
    // For simplicity, if still >0, it remains Active, otherwise Completed or we can leave it Active for future trades.
    if offer.amount == 0 {
        offer.status = OfferStatus::InTrade;
    }
    OFFERS.save(deps.storage, offer_id.into(), &offer)?;

    let mut trade_seq = TRADE_SEQ.load(deps.storage)?;
    trade_seq += 1;
    TRADE_SEQ.save(deps.storage, &trade_seq)?;

    let trade = Trade {
        offer_id,
        seller: offer.seller.clone(),
        buyer: info.sender.to_string(),
        amount,
        status: TradeStatus::Pending,
        payment_method,
        created_at: env.block.time.seconds(),
        payment_confirmed_at: None,
        completed_at: None,
        disputed: false,
        buyer_escrow_funded: true,
    };

    TRADES.save(deps.storage, trade_seq.into(), &trade)?;

    Ok(Response::new()
        .add_attribute("action", "initiate_trade")
        .add_attribute("trade_id", trade_seq.to_string()))
}


//Buyer confirms payment, After trade initiated
fn execute_confirm_payment(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: u64,
) -> Result<Response, ContractError> {
    let mut trade = TRADES.may_load(deps.storage, trade_id.into())?.ok_or(ContractError::TradeNotFound {})?;
    if trade.status != TradeStatus::Pending {
        return Err(ContractError::InvalidTradeStatus {});
    }
    if info.sender.to_string() != trade.buyer {
        return Err(ContractError::NotBuyer {});
    }

    trade.status = TradeStatus::PaymentSent;
    trade.payment_confirmed_at = Some(env.block.time.seconds());

    TRADES.save(deps.storage, trade_id.into(), &trade)?;

    Ok(Response::new()
        .add_attribute("action", "confirm_payment")
        .add_attribute("trade_id", trade_id.to_string()))
}

//After Buyer confirms payment, contract will release funds
fn execute_release_funds(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: u64,
) -> Result<Response, ContractError> {
    let mut trade = TRADES.may_load(deps.storage, trade_id.into())?.ok_or(ContractError::TradeNotFound {})?;
    if trade.status != TradeStatus::PaymentSent {
        return Err(ContractError::InvalidTradeStatus {});
    }
    if info.sender.to_string() != trade.seller {
        return Err(ContractError::NotSeller {});
    }

    // Load the offer to know the crypto type
    let offer = OFFERS.load(deps.storage, trade.offer_id)?;
    let crypto_denom = offer.crypto_type;
    let crypto_amount = trade.amount;
    let payment_amount = trade.amount.checked_mul(offer.price_per_unit).ok_or(ContractError::InvalidTradeAmount {})?;
    let seller_addr = deps.api.addr_validate(&trade.seller)?;
    let buyer_addr = deps.api.addr_validate(&trade.buyer)?;

    // Release buyer's payment to the seller
    // Release seller's crypto to the buyer
    // This involves sending BankMsg submessages:
    use cosmwasm_std::{BankMsg, CosmosMsg, Uint128};

    let msgs = vec![
        CosmosMsg::Bank(BankMsg::Send {
            to_address: buyer_addr.to_string(),
            amount: vec![cosmwasm_std::Coin {
                denom: crypto_denom.clone(),
                amount: Uint128::from(crypto_amount),
            }],
        }),
        CosmosMsg::Bank(BankMsg::Send {
            to_address: seller_addr.to_string(),
            amount: vec![cosmwasm_std::Coin {
                denom: "ust".to_string(), // Assuming buyer paid in "ust"
                amount: Uint128::from(payment_amount),
            }],
        })
    ];


    // Here you would actually transfer funds. This is just changing state for demonstration.
    trade.status = TradeStatus::Completed;
    trade.completed_at = Some(env.block.time.seconds());

    TRADES.save(deps.storage, trade_id.into(), &trade)?;

    Ok(Response::new()
        .add_attribute("action", "release_funds")
        .add_attribute("trade_id", trade_id.to_string())
        .add_messages(msgs))
}


//If not happy with trade buyer can create dispute
fn execute_dispute_trade(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    trade_id: u64,
    reason: String,
) -> Result<Response, ContractError> {
    let mut trade = TRADES.may_load(deps.storage, trade_id.into())?.ok_or(ContractError::TradeNotFound {})?;

    // Let's assume either party can dispute if payment has been sent but not released
    if trade.status != TradeStatus::PaymentSent {
        return Err(ContractError::InvalidTradeStatus {});
    }

    // The trade is now disputed
    trade.status = TradeStatus::Disputed;
    trade.disputed = true;
    TRADES.save(deps.storage, trade_id.into(), &trade)?;

    Ok(Response::new()
        .add_attribute("action", "dispute_trade")
        .add_attribute("trade_id", trade_id.to_string())
        .add_attribute("reason", reason)
        .add_attribute("by", info.sender.to_string()))
}

fn execute_resolve_dispute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    trade_id: u64,
    winner: String,
) -> Result<Response, ContractError> {
    ensure_admin(&deps, &info)?;
    let mut trade = TRADES.may_load(deps.storage, trade_id.into())?.ok_or(ContractError::TradeNotFound {})?;

    if trade.status != TradeStatus::Disputed {
        return Err(ContractError::InvalidTradeStatus {});
    }

    // winner could be "buyer" or "seller" address. Actual logic of awarding funds would be here.
    trade.status = TradeStatus::Resolved;
    TRADES.save(deps.storage, trade_id.into(), &trade)?;

    Ok(Response::new()
        .add_attribute("action", "resolve_dispute")
        .add_attribute("trade_id", trade_id.to_string())
        .add_attribute("winner", winner))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_json_binary(&query_config(deps)?),
        QueryMsg::GetOffer { offer_id } => to_json_binary(&query_offer(deps, offer_id)?),
        QueryMsg::ListOffers { start_after, limit } => to_json_binary(&query_offers(deps, start_after, limit)?),
        QueryMsg::GetTrade { trade_id } => to_json_binary(&query_trade(deps, trade_id)?),
        QueryMsg::ListTradesByUser { user, start_after, limit } => to_json_binary(&query_trades_by_user(deps, user, start_after, limit)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse { admin: config.admin.to_string() })
}

fn query_offer(deps: Deps, offer_id: u64) -> StdResult<OfferResponse> {
    let offer = OFFERS.load(deps.storage, offer_id.into())?;
    Ok(OfferResponse { offer })
}

fn query_offers(deps: Deps, start_after: Option<u64>, limit: Option<u32>) -> StdResult<OffersListResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT);
    let start = start_after.map(Bound::exclusive);
    let offers: Vec<(u64, Offer)> = OFFERS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit as usize)
        .map(|res| {
            let (k, v) = res?;
            let key: u64 = k.into();
            Ok((key, v))
        })
        .collect::<StdResult<Vec<_>>>()?;
    Ok(OffersListResponse { offers })
}

fn query_trade(deps: Deps, trade_id: u64) -> StdResult<TradeResponse> {
    let trade = TRADES.load(deps.storage, trade_id.into())?;
    Ok(TradeResponse { trade_id, trade })
}

fn query_trades_by_user(deps: Deps, user: String, start_after: Option<u64>, limit: Option<u32>) -> StdResult<TradesListResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT);
    let start = start_after.map(Bound::exclusive);

    let user = user;
    // We must scan all trades. In a real scenario, you'd probably have a secondary index.
    // For demonstration, just filter in-memory (not efficient for large sets).
    let trades: Vec<(u64, Trade)> = TRADES
        .range(deps.storage, start, None, Order::Ascending)
        .filter_map(|res| {
            if let Ok((k,v)) = res {
                let key: u64 = k.into();
                if v.buyer == user || v.seller == user {
                    return Some(Ok((key, v)));
                }
            }
            None
        })
        .take(limit as usize)
        .collect::<StdResult<Vec<_>>>()?;

    Ok(TradesListResponse { trades })
}




#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{testing::{mock_dependencies, mock_env, mock_info}, from_json};

    use crate::msg::{InstantiateMsg, QueryMsg, ConfigResponse};

    #[test]
    fn instantiate_and_query_config() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);

        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let query_res = query(deps.as_ref(), env.clone(), QueryMsg::GetConfig {}).unwrap();
        let config: ConfigResponse = from_json(&query_res).unwrap();
        assert_eq!(config.admin, "creator");
    }

    #[test]
    fn create_offer() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("seller", &[]);

        let msg = InstantiateMsg { admin: None };
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let create_offer_msg = ExecuteMsg::CreateOffer {
            crypto_type: "BTC".to_string(),
            amount: 1000,
            price_per_unit: 50000,
            payment_methods: "BankTransfer".to_string(),
            min_trade_limit: 100,
            max_trade_limit: 1000,
        };

        let res = execute(deps.as_mut(), env.clone(), info.clone(), create_offer_msg).unwrap();
        assert!(res.attributes.iter().any(|attr| attr.key == "offer_id"));

        // Query the first offer
        let query_offers_res = query(deps.as_ref(), env.clone(), QueryMsg::ListOffers { start_after: None, limit: None }).unwrap();
        let list: OffersListResponse = from_json(&query_offers_res).unwrap();
        assert_eq!(list.offers.len(), 1);
        let (id, offer) = &list.offers[0];
        assert_eq!(*id, 1u64);
        assert_eq!(offer.seller, "seller");
    }

    #[test]
    fn initiate_trade() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let admin_info = mock_info("admin", &[]);

        // Instantiate with admin
        let msg = InstantiateMsg { admin: Some("admin".to_string()) };
        instantiate(deps.as_mut(), env.clone(), admin_info.clone(), msg).unwrap();

        // Create offer
        let create_offer_msg = ExecuteMsg::CreateOffer {
            crypto_type: "BTC".to_string(),
            amount: 1000,
            price_per_unit: 50000,
            payment_methods: "BankTransfer".to_string(),
            min_trade_limit: 100,
            max_trade_limit: 1000,
        };
        let seller_info = mock_info("seller", &[]);
        execute(deps.as_mut(), env.clone(), seller_info.clone(), create_offer_msg).unwrap();

        // Initiate trade
        let initiate_trade_msg = ExecuteMsg::InitiateTrade {
            offer_id: 1,
            amount: 200,
            payment_method: "BankTransfer".to_string(),
        };
        let buyer_info = mock_info("buyer", &[]);
        let res = execute(deps.as_mut(), env.clone(), buyer_info.clone(), initiate_trade_msg).unwrap();
        assert!(res.attributes.iter().any(|attr| attr.key == "trade_id"));
    }
}
