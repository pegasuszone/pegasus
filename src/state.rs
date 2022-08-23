use cosmwasm_std::{Addr, BlockInfo, Decimal, Timestamp, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};
use cw_utils::Duration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sg_controllers::Hooks;

use crate::helpers::ExpiryRange;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SudoParams {
    // incentive for users to close stale p2p trade
    pub escrow_deposit_amount: Uint128,
    /// Valid time range for Asks
    /// (min, max) in seconds
    pub offer_expiry: ExpiryRange,
    /// Operators are entites that are responsible for maintaining the active state of Asks
    /// They listen to NFT transfer events, and update the active state of Asks
    pub operators: Vec<Addr>,

    /// Duration after expiry when a bid becomes stale
    pub stale_offer_duration: Duration,
    /// Stale bid removal reward
    pub offer_removal_reward_percent: Decimal,
}

pub const SUDO_PARAMS: Item<SudoParams> = Item::new("sudo-params");

pub const OFFER_HOOKS: Hooks = Hooks::new("offer-hooks");
// pub const BID_HOOKS: Hooks = Hooks::new("bid-hooks");
// pub const SALE_HOOKS: Hooks = Hooks::new("sale-hooks");
// pub const COLLECTION_BID_HOOKS: Hooks = Hooks::new("collection-bid-hooks");

pub type TokenId = u32;

pub trait Order {
    fn expires_at(&self) -> Timestamp;

    fn is_expired(&self, block: &BlockInfo) -> bool {
        self.expires_at() <= block.time
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SaleType {
    FixedPrice,
    Auction,
}

/// Represents an ask on the marketplace
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Offer {
    // pub sale_type: SaleType,
    pub collection_offered: Addr,
    pub token_id_offered: TokenId,
    
    pub token_id_wanted: TokenId,
    pub collection_wanted: Addr,
    
    pub offeror: Addr,
    pub peer: Addr,
    // pub price: Uint128,
    // pub funds_recipient: Option<Addr>,
    pub reserve_for: Option<Addr>,
    // pub finders_fee_bps: Option<u64>,
    pub expires_at: Timestamp,
    pub is_active: bool,
}

impl Order for Offer {
    fn expires_at(&self) -> Timestamp {
        self.expires_at
    }
}

/// Primary key for offer: (collection, token_id, Owner)
pub type OfferKey = (Addr, TokenId, Addr);
/// Convenience ask key constructor
pub fn offer_key(collection: &Addr, token_id: TokenId, owner: Addr) -> OfferKey {
    (collection.clone(), token_id, owner)
}

/// Defines indices for accessing Asks
pub struct OfferIndicies<'a> {
    pub offeror: MultiIndex<'a, Addr, Offer, OfferKey>,
    pub collection_wanted: MultiIndex<'a, Addr, Offer, OfferKey>,
    pub collection_offered: MultiIndex<'a, Addr, Offer, OfferKey>,
    pub collection_token_id_wanted: MultiIndex<'a, (Addr, TokenId), Offer, OfferKey>,
    pub collection_token_id_offered: MultiIndex<'a, (Addr, TokenId), Offer, OfferKey>,
    pub peer: MultiIndex<'a, Addr, Offer, OfferKey>,
}


impl<'a> IndexList<Offer> for OfferIndicies<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Offer>> + '_> {
        let v: Vec<&dyn Index<Offer>> = vec![
            &self.offeror, 
            &self.collection_wanted, 
            &self.collection_offered,
            &self.collection_token_id_wanted,
            &self.collection_token_id_offered, 
            &self.peer
        ];
        Box::new(v.into_iter())
    }
}

// TODO: Implement this properly
// what do we want this to do
pub fn offers<'a>() -> IndexedMap<'a, OfferKey, Offer, OfferIndicies<'a>> {
    let indexes = OfferIndicies {
        offeror: MultiIndex::new(|d: &Offer| d.offeror.clone(), "offers", "offers_offerors"),
        collection_wanted: MultiIndex::new(|d: &Offer| d.collection_wanted.clone(),"offers", "collection_wanted"),
        collection_offered: MultiIndex::new(|d: &Offer| d.collection_offered.clone(),"offers", "collection_offered"),
        collection_token_id_wanted: MultiIndex::new(|d: &Offer| (d.collection_wanted, d.token_id_wanted), "offers", "tokens_wanted"),
        collection_token_id_offered: MultiIndex::new(|d: &Offer| (d.collection_offered, d.token_id_offered), "offers", "tokens_wanted"),
        peer: MultiIndex::new(|d: &Offer| d.peer.clone(), "Offers", "peers")
    };
    IndexedMap::new("offers", indexes)
}