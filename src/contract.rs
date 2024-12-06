#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

/*
const CONTRACT_NAME: &str = "crates.io:p2p_exchange";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
 */

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    platform_fee: Uint128,
) -> StdResult<Response> {
    let config = Config {
        admin: info.sender.to_string(),
        platform_fee,
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreateOffer {
            crypto_type,
            amount,
            price_per_unit,
            payment_methods,
            min_trade_limit,
            region_restrictions,
        } => execute_create_offer(
            deps,
            env,
            info,
            crypto_type,
            amount,
            price_per_unit,
            payment_methods,
            min_trade_limit,
            region_restrictions,
        ),
        ExecuteMsg::InitiateTrade {
            offer_id,
            amount,
            payment_method,
        } => execute_initiate_trade(deps, env, info, offer_id, amount, payment_method),
        ExecuteMsg::ConfirmPayment { trade_id } => {
            execute_confirm_payment(deps, env, info, trade_id)
        },
        ExecuteMsg::ReleaseFunds { trade_id } => execute_release_funds(deps, env, info, trade_id),
        ExecuteMsg::DisputeTrade { trade_id, reason } => {
            execute_dispute_trade(deps, env, info, trade_id, reason)
        },
        ExecuteMsg::ResolveDispute { trade_id, winner } => {
            execute_resolve_dispute(deps, env, info, trade_id, winner)
        },
    }
}

fn execute_create_offer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    crypto_type: String,
    amount: Uint128,
    price_per_unit: Uint128,
    payment_methods: Vec<String>,
    min_trade_limit: Option<Uint128>,
    region_restrictions: Option<Vec<String>>,
) -> StdResult<Response> {
    let offer_id = format!("offer_{}", env.block.height);
    let offer = Offer {
        seller: info.sender.to_string(),
        crypto_type,
        amount,
        price_per_unit,
        payment_methods,
        min_trade_limit,
        region_restrictions,
        status: OfferStatus::Active,
        created_at: env.block.time.seconds(),
    };
    
    OFFERS.save(deps.storage, &offer_id, &offer)?;
    
    Ok(Response::new()
        .add_attribute("action", "create_offer")
        .add_attribute("offer_id", offer_id))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}





