use cosmwasm_std::StdError;
use thiserror::Error;

use crate::helpers::ExpiryRangeError;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("UnauthorizedOwner")]
    UnauthorizedOwner {},
    #[error("Token (collection: {collection:?}, id: {token_id:?}) is already offered in offer {offer_id:?}" )]
    TokenAlreadyOffered {
        collection: String,
        token_id: u32,
        offer_id: u8,
    },

    #[error("Peer {peer:?} is not owner of Token (collection: {collection:?}, id: {token_id:?})")]
    UnauthorizedPeer {
        collection: String,
        token_id: u32,
        peer: String,
    },

    #[error("UnauthorizedOperator")]
    UnauthorizedOperator {},

    #[error("{0}")]
    ExpiryRange(#[from] ExpiryRangeError),
}
