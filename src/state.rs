use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, StdResult, Storage, Timestamp};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex, UniqueIndex};

use crate::helpers::ExpiryRange;

pub const MIN_EXPIRY: u64 = 3600 * 24; // seconds -> one day
pub const MAX_EXPIRY: u64 = 3600 * 24 * 28; // seconds -> one month

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

pub const SUDO_PARAMS: Item<SudoParams> = Item::new("sudo-params");

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
pub const OFFER_ID_COUNTER: Item<u64> = Item::new("offer_id_counter");

// Get next incrementing ID
pub fn next_offer_id(store: &mut dyn Storage) -> StdResult<u64> {
    let id: u64 = OFFER_ID_COUNTER.may_load(store)?.unwrap_or_default() + 1;
    OFFER_ID_COUNTER.save(store, &id)?;

    Ok(id)
}

pub const OFFER_NAMESPACE: &str = "offers";
pub struct OfferIndexes<'a> {
    pub id: UniqueIndex<'a, u64, Offer>,
    pub by_sender: MultiIndex<'a, Addr, Offer, u64>,
    pub by_peer: MultiIndex<'a, Addr, Offer, u64>,
}

impl<'a> IndexList<Offer> for OfferIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Offer>> + '_> {
        let v: Vec<&dyn Index<Offer>> = vec![&self.id, &self.by_sender, &self.by_peer];
        Box::new(v.into_iter())
    }
}

// Function to get all offers and manipulate offer data
pub fn offers<'a>() -> IndexedMap<'a, u64, Offer, OfferIndexes<'a>> {
    let indexes = OfferIndexes {
        id: UniqueIndex::new(|d| d.id, "offers__id"),
        by_sender: MultiIndex::new(|d| d.sender.clone(), "offers", "offers__sender"),
        by_peer: MultiIndex::new(|d| d.peer.clone(), "offers", "offers__peer"),
    };
    IndexedMap::new(OFFER_NAMESPACE, indexes)
}
