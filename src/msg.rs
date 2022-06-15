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
    /// Fair Burn fee for winning bids
    /// 0.25% = 25, 0.5% = 50, 1% = 100, 2.5% = 250
    // pub trading_fee_bps: u64,

    pub escrow_deposit_amount: Uint128,
    /// Valid time range for Offers
    /// (min, max) in seconds
    pub offer_expiry: ExpiryRange,

    // /// Valid time range for Bids
    // /// (min, max) in seconds
    // pub bid_expiry: ExpiryRange,
    /// Operators are entites that are responsible for maintaining the active state of Offers.
    /// They listen to NFT transfer events, and update the active state of Offers.
    pub operators: Vec<String>,

    /// Max basis points for the finders fee
    pub max_finders_fee_bps: u64,
    /// Min value for bids and Offers
    pub min_price: Uint128,
    /// Duration after expiry when a bid becomes stale (in seconds)
    pub stale_offer_duration: Duration,
    /// Stale bid removal reward
    pub offer_removal_reward_bps: u64,


}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// List an NFT on the marketplace by creating a new Offer
    // SetOffer {
    //     sale_type: SaleType,
    //     collection: String,
    //     token_id: TokenId,
    //     price: Coin,
    //     funds_recipient: Option<String>,
    //     reserve_for: Option<String>,
    //     finders_fee_bps: Option<u64>,
    //     expires: Timestamp,
    // },

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
        token_id: TokenId
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

    /// Priviledged operation to change the active state of an Offer when an NFT is transferred
    SyncOffer {
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
        escrow_deposit_amount:Option<Uint128>,
        Offer_expiry:Option<ExpiryRange>,
        operators:Option<Vec<Addr>>,
        stale_offer_duration:Option<Duration>,
        offer_removal_reward_bps:Option<u64>,
    },

    /// Add a new hook to be informed of all Offers
    AddOfferHook { hook: String },
}

pub type Collection = String;
pub type Offeror = String;
pub type Peer = String;

/// Offset for Offer pagination
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OfferOffset {
    pub price: Uint128,
    pub token_id: TokenId,
}

impl OfferOffset {
    pub fn new(price: Uint128, token_id: TokenId) -> Self {
        OfferOffset { price, token_id }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// List of collections that have Offers on them
    /// Return type: `CollectionsResponse`
    Collections {
        start_after: Option<Collection>,
        limit: Option<u32>,
    },
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
    },
    
    /// Get all Offers for a collection, sorted by price
    /// Return type: `OffersResponse`
    OffersSortedByCollectionPrice {
        offeror: Offeror,
        collection: Collection,
        include_inactive: Option<bool>,
        start_after: Option<OfferOffset>,
        limit: Option<u32>,
    },
    /// Get all Offers for a collection, sorted by price in reverse
    /// Return type: `OffersResponse`
    ReverseOffersSortedByPrice {
        offeror: Offeror,
        collection: Collection,
        include_inactive: Option<bool>,
        start_after: Option<OfferOffset>,
        limit: Option<u32>,
    },
    /// Count of all Offers
    /// Return type: `OfferCountResponse`
    OffersCount { collection: Collection },

    /// Return type: `HooksResponse`
    OfferHooks {},

    /// Get the config for the contract
    /// Return type: `ParamsResponse`
    Params {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OfferResponse {
    pub Offer: Option<Offer>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OffersResponse {
    pub Offers: Vec<Offer>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OfferCountResponse {
    pub count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionsResponse {
    pub collections: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ParamsResponse {
    pub params: SudoParams,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SaleHookMsg {
    pub collection: String,
    pub token_id: u32,
    pub price: Coin,
    pub seller: String,
    pub buyer: String,
}

impl SaleHookMsg {
    pub fn new(
        collection: String,
        token_id: u32,
        price: Coin,
        seller: String,
        buyer: String,
    ) -> Self {
        SaleHookMsg {
            collection,
            token_id,
            price,
            seller,
            buyer,
        }
    }

    /// serializes the message
    pub fn into_binary(self) -> StdResult<Binary> {
        let msg = SaleExecuteMsg::SaleHook(self);
        to_binary(&msg)
    }
}

// This is just a helper to properly serialize the above message
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SaleExecuteMsg {
    SaleHook(SaleHookMsg),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum HookAction {
    Create,
    Update,
    Delete,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OfferHookMsg {
    pub Offer: Offer,
}

impl OfferHookMsg {
    pub fn new(Offer: Offer) -> Self {
        OfferHookMsg { Offer }
    }

    /// serializes the message
    pub fn into_binary(self, action: HookAction) -> StdResult<Binary> {
        let msg = match action {
            HookAction::Create => OfferHookExecuteMsg::OfferCreatedHook(self),
            HookAction::Update => OfferHookExecuteMsg::OfferUpdatedHook(self),
            HookAction::Delete => OfferHookExecuteMsg::OfferDeletedHook(self),
        };
        to_binary(&msg)
    }
}

// This is just a helper to properly serialize the above message
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum OfferHookExecuteMsg {
    OfferCreatedHook(OfferHookMsg),
    OfferUpdatedHook(OfferHookMsg),
    OfferDeletedHook(OfferHookMsg),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct BidHookMsg {
    pub bid: Bid,
}

impl BidHookMsg {
    pub fn new(bid: Bid) -> Self {
        BidHookMsg { bid }
    }

    /// serializes the message
    pub fn into_binary(self, action: HookAction) -> StdResult<Binary> {
        let msg = match action {
            HookAction::Create => BidExecuteMsg::BidCreatedHook(self),
            HookAction::Update => BidExecuteMsg::BidUpdatedHook(self),
            HookAction::Delete => BidExecuteMsg::BidDeletedHook(self),
        };
        to_binary(&msg)
    }
}

// This is just a helper to properly serialize the above message
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BidExecuteMsg {
    BidCreatedHook(BidHookMsg),
    BidUpdatedHook(BidHookMsg),
    BidDeletedHook(BidHookMsg),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CollectionBidHookMsg {
    pub collection_bid: CollectionBid,
}

impl CollectionBidHookMsg {
    pub fn new(collection_bid: CollectionBid) -> Self {
        CollectionBidHookMsg { collection_bid }
    }

    /// serializes the message
    pub fn into_binary(self, action: HookAction) -> StdResult<Binary> {
        let msg = match action {
            HookAction::Create => CollectionBidExecuteMsg::CollectionBidCreatedHook(self),
            HookAction::Update => CollectionBidExecuteMsg::CollectionBidUpdatedHook(self),
            HookAction::Delete => CollectionBidExecuteMsg::CollectionBidDeletedHook(self),
        };
        to_binary(&msg)
    }
}

// This is just a helper to properly serialize the above message
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CollectionBidExecuteMsg {
    CollectionBidCreatedHook(CollectionBidHookMsg),
    CollectionBidUpdatedHook(CollectionBidHookMsg),
    CollectionBidDeletedHook(CollectionBidHookMsg),
}
