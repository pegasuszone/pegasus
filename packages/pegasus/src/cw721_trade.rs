use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, BlockInfo, Coin, StdError, Timestamp};
use thiserror::Error;

pub const CW721_TRADE: &str = "cw721-trade";

#[cw_serde]
#[cfg_attr(feature = "boot", derive(boot_core::ExecuteFns))]
#[cfg_attr(feature = "boot", impl_into(ExecuteMsg))]
pub struct InstantiateMsg {
    /// Valid time range for Offers
    /// (min, max) in seconds
    pub offer_expiry: ExpiryRange,

    /// Developer address
    pub maintainer: String,

    /// Maximum amount of offers that can be sent by a user
    pub max_offers: u64,

    /// Maximum amount of NFTs in bundle
    pub bundle_limit: u64,
}

#[cw_serde]
pub struct TokenMsg {
    pub collection: String,
    pub token_id: u32,
}

#[cw_serde]
#[cfg_attr(feature = "boot", derive(boot_core::ExecuteFns))]
#[cfg_attr(feature = "boot", impl_into(ExecuteMsg))]
pub enum ExecuteMsg {
    /// Create a new offer
    CreateOffer {
        offered_nfts: Vec<TokenMsg>,
        wanted_nfts: Vec<TokenMsg>,
        offered_balances: Vec<Coin>,
        message: Option<String>,
        peer: String,
        expires_at: Option<Timestamp>,
    },
    /// Remove an offer (called by sender)
    RemoveOffer { id: u64 },
    /// Accept an existing offer (called by peer)
    AcceptOffer { id: u64 },
    /// Reject an existing offer (called by peer)
    RejectOffer { id: u64 },
    /// Operation to remove stale offers (called by anyone & incentivized)
    RemoveStaleOffer { id: u64 },
}

#[cw_serde]
#[cfg_attr(feature = "boot", derive(boot_core::SudoMsgFns))]
#[cfg_attr(feature = "boot", impl_into(SudoMsg))]
pub enum SudoMsg {
    /// Update the contract parameters
    /// Can only be called by governance
    UpdateParams {
        offer_expiry: Option<ExpiryRange>,
        maintainer: Option<String>,
        max_offers: Option<u64>,
        bundle_limit: Option<u64>,
    },
}

#[cw_serde]
pub enum MigrateMsg {}

#[cw_serde]
#[cfg_attr(feature = "boot", derive(boot_core::QueryFns))]
#[cfg_attr(feature = "boot", impl_into(QueryMsg))]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(OfferResponse)]
    Offer { id: u64 },
    #[returns(OffersResponse)]
    OffersBySender { sender: String },
    #[returns(OffersResponse)]
    OffersByPeer { peer: String },
    #[returns(ParamsResponse)]
    Params {},
}

#[cw_serde]
pub struct OfferResponse {
    pub offer: Option<Offer>,
}

#[cw_serde]
pub struct OffersResponse {
    pub offers: Vec<Offer>,
}

#[cw_serde]
pub struct ParamsResponse {
    pub params: SudoParams,
}

#[cw_serde]
pub struct SudoParams {
    /// Valid time range for Offers
    /// (min, max) in seconds
    pub offer_expiry: ExpiryRange,

    /// Developer address
    pub maintainer: Addr,

    /// Maximum amount of offers a user can send
    pub max_offers: u64,

    /// Maximum amount of NFTs in bundle
    pub bundle_limit: u64,
}

pub type TokenId = u32;

/// Represents a token that can be offered
#[cw_serde]
pub struct Token {
    pub collection: Addr,
    pub token_id: TokenId,
}

/// Represents a set of royalties to be paid out to a creator
#[cw_serde]
pub struct Royalty {
    pub creator: Addr,
    pub amount: Coin,
}

/// Represents an ask on the marketplace
#[cw_serde]
pub struct Offer {
    /// Unique identifier
    pub id: u64,

    /// Arrays of offered & wanted NFTs, both defined by the sender
    pub offered_nfts: Vec<Token>,
    pub wanted_nfts: Vec<Token>,

    /// Array of offered native/ibc tokens, defined by sender
    pub offered_balances: Vec<Coin>,

    /// Optional text message from the sender
    pub message: Option<String>,

    /// Royalties to be paid out to creators when native/ibc tokens are involved
    pub royalties: Vec<Royalty>,

    pub sender: Addr,
    pub peer: Addr,
    pub created_at: Timestamp,
    pub expires_at: Timestamp,
}

// Incrementing ID counter

// Get next incrementing ID

#[derive(Error, Debug, PartialEq)]
pub enum ExpiryRangeError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Invalid expiration range")]
    InvalidExpirationRange {},

    #[error("Expiry min > max")]
    InvalidExpiry {},
}

#[cw_serde]
pub struct ExpiryRange {
    pub min: u64,
    pub max: u64,
}

impl ExpiryRange {
    pub fn new(min: u64, max: u64) -> Self {
        ExpiryRange { min, max }
    }

    /// Validates if given expires time is within the allowable range
    pub fn is_valid(
        &self,
        block: &BlockInfo,
        created_at: Timestamp,
        expires: Timestamp,
    ) -> Result<(), ExpiryRangeError> {
        let now = block.time;
        if !(expires > created_at.plus_seconds(self.min) && expires <= now.plus_seconds(self.max)) {
            return Err(ExpiryRangeError::InvalidExpirationRange {});
        }

        Ok(())
    }

    pub fn validate(&self) -> Result<(), ExpiryRangeError> {
        if self.min > self.max {
            return Err(ExpiryRangeError::InvalidExpiry {});
        }

        Ok(())
    }
}
