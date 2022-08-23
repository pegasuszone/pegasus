use cosmwasm_std::{Addr, BlockInfo, Decimal, Timestamp, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};
use cw_utils::Duration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::helpers::ExpiryRange;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SudoParams {
    // incentive for users to close stale p2p trade
    pub escrow_deposit_amount: Uint128,
    /// Valid time range for Asks
    /// (min, max) in seconds
    pub offer_expiry: ExpiryRange,

    /// Duration after expiry when a bid becomes stale
    pub stale_offer_duration: Duration,
    /// Stale bid removal reward
    pub offer_removal_reward_percent: Decimal,
}

pub const SUDO_PARAMS: Item<SudoParams> = Item::new("sudo-params");

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

/// Represents a token that can be offered
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Token {
    pub collection: Addr,
    pub token_id: TokenId,
}

/// Represents an ask on the marketplace
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Offer {
    pub offer: Token,
    pub wanted: Token,
    pub sender: Addr,
    pub peer: Addr,

    pub expires_at: Timestamp,
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
    pub sender: MultiIndex<'a, Addr, Offer, OfferKey>,
    pub peer: MultiIndex<'a, Addr, Offer, OfferKey>,
}

impl<'a> IndexList<Offer> for OfferIndicies<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Offer>> + '_> {
        let v: Vec<&dyn Index<Offer>> = vec![&self.sender, &self.peer];
        Box::new(v.into_iter())
    }
}

pub fn offers<'a>() -> IndexedMap<'a, OfferKey, Offer, OfferIndicies<'a>> {
    let indexes = OfferIndicies {
        sender: MultiIndex::new(|d: &Offer| d.sender.clone(), "offers", "senders"),
        peer: MultiIndex::new(|d: &Offer| d.peer.clone(), "offers", "peers"),
    };
    IndexedMap::new("offers", indexes)
}
