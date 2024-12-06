use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Offer not found")]
    OfferNotFound {},

    #[error("Trade not found")]
    TradeNotFound {},

    #[error("Invalid trade amount")]
    InvalidTradeAmount {},

    #[error("Offer not active")]
    OfferNotActive {},

    #[error("Invalid trade status")]
    InvalidTradeStatus {},

    #[error("Only the buyer can confirm payment")]
    NotBuyer {},

    #[error("Only the seller can release funds")]
    NotSeller {},

    #[error("Only admin can resolve disputes")]
    NotAdmin {},
}
