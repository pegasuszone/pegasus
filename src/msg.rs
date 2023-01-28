use crate::{
    helpers::ExpiryRange,
    state::{Offer, SudoParams},
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Timestamp};

#[cw_serde]
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
