use crate::{
    helpers::ExpiryRange,
    state::{Offer, SudoParams},
};
use cosmwasm_std::{Timestamp, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// Amount in micros to be deposited by the sender of an offer
    /// This escrow will be refunded when the offer is accepted or denied
    /// The sender will lose this deposit if they let the offer expire
    pub escrow_deposit_amount: Uint128,

    /// Valid time range for Offers
    /// (min, max) in seconds
    pub offer_expiry: ExpiryRange,

    /// Developer address
    pub maintainer: String,

    /// Stale trade removal reward
    pub removal_reward_bps: u64,

    /// Maximum amount of offers that can be sent by a user
    pub max_offers: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenMsg {
    pub collection: String,
    pub token_id: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Create a new offer
    CreateOffer {
        offered_nfts: Vec<TokenMsg>,
        wanted_nfts: Vec<TokenMsg>,
        peer: String,
        expires_at: Option<Timestamp>,
    },
    /// Remove an offer (called by sender)
    // TODO: why not u64?
    RemoveOffer { id: u8 },
    /// Accept an existing offer (called by peer)
    AcceptOffer { id: u8 },
    /// Reject an existing offer (called by peer)
    RejectOffer { id: u8 },
    /// Operation to remove stale offers (called by anyone & incentivized)
    RemoveStaleOffer { id: u8 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
    /// Update the contract parameters
    /// Can only be called by governance
    UpdateParams {
        escrow_deposit_amount: Option<Uint128>,
        offer_expiry: Option<ExpiryRange>,
        maintainer: Option<String>,
        removal_reward_bps: Option<u64>,
        max_offers: Option<u64>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Offer { id: u8 },
    OffersBySender { sender: String },
    OffersByPeer { peer: String },
    Params {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OfferResponse {
    pub offer: Option<Offer>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OffersResponse {
    pub offers: Vec<Offer>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ParamsResponse {
    pub params: SudoParams,
}
