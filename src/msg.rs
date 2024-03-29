use crate::{
    helpers::ExpiryRange,
    state::{Offer, SudoParams},
};
use cosmwasm_std::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
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
    RemoveOffer { id: u64 },
    /// Accept an existing offer (called by peer)
    AcceptOffer { id: u64 },
    /// Reject an existing offer (called by peer)
    RejectOffer { id: u64 },
    /// Operation to remove stale offers (called by anyone & incentivized)
    RemoveStaleOffer { id: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Offer { id: u64 },
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
