use crate::{
    helpers::ExpiryRange,
    state::{Offer, SudoParams, TokenId},
};
use cosmwasm_std::{to_binary, Addr, Binary, Coin, StdResult, Timestamp, Uint128};
use cw_utils::Duration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub escrow_deposit_amount: Uint128,
    /// Valid time range for Offers
    /// (min, max) in seconds
    pub offer_expiry: ExpiryRange,

    /// Operators are entites that are responsible for maintaining the active state of Offers.
    /// They listen to NFT transfer events, and update the active state of Offers.
    pub operators: Vec<String>,

    /// Max basis points for the finders fee
    pub max_finders_fee_bps: u64,
    /// Duration after expiry when a bid becomes stale (in seconds)
    pub stale_offer_duration: Duration,
    /// Stale bid removal reward
    pub offer_removal_reward_bps: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetOffer {
        collection_offered: Addr,
        token_id_offered: TokenId,
        token_id_wanted: TokenId,
        collection_wanted: Addr,
        offeror: Addr,
        peer: Addr,

        expires_at: Timestamp,
        is_active: bool,
    },
    RemoveOffer {
        collection: String,
        token_id: TokenId,
    },

    /// Accept an existing offer
    AcceptOffer {
        collection: String,
        token_id: TokenId,
    },
    /// close offer by peer
    RefuseOffer {
        collection: String,
        token_id: TokenId,
    },

    /// Privileged operation to remove stale bids
    RemoveStaleOffer {
        collection: String,
        token_id: TokenId,
        offeror: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
    /// Update the contract parameters
    /// Can only be called by governance
    UpdateParams {
        escrow_deposit_amount: Option<Uint128>,
        Offer_expiry: Option<ExpiryRange>,
        operators: Option<Vec<Addr>>,
        stale_offer_duration: Option<Duration>,
        offer_removal_reward_bps: Option<u64>,
    },

    /// Add a new hook to be informed of all Offers
    AddOfferHook { hook: String },
}

pub type Collection = String;
pub type Offeror = String;
pub type Peer = String;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Get the current Offer for specific NFT
    /// Return type: `CurrentOfferResponse`
    Offer {
        collection: Collection,
        token_id: TokenId,
    },
    /// Get all offers for offeror
    /// Return type: `OffersResponse`
    Offers {
        offeror: Offeror,
        start_after: Option<Offeror>,
        limit: Option<u32>,
    },

    Requests {
        peer: Peer,
        start_after: Option<Peer>,
        limit: Option<u32>,
    },
    /// Return type: `HooksResponse`
    OfferHooks {},

    /// Get the config for the contract
    /// Return type: `ParamsResponse`
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
pub struct RequestsResponse {
    pub requests: Vec<Offer>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OfferCountResponse {
    pub count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenIDWantedResponse {
    pub offer: Option<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenIDOfferedResponse {
    pub offer: Option<Addr>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ParamsResponse {
    pub params: SudoParams,
}

// not sure if we need these query responses
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionsWantedResponse {
    pub collections: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionsOfferedResponse {
    pub collections: Vec<Addr>,
}
