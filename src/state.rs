use cosmwasm_std::{Addr, Decimal, StdResult, Storage, Timestamp, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex, UniqueIndex};
use cw_utils::Duration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::helpers::ExpiryRange;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SudoParams {
    // incentive for users to close stale p2p trade
    pub Offer_deposit_amount: Uint128,
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

/// Represents a token that can be offered
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Token {
    pub collection: Addr,
    pub token_id: TokenId,
}

/// Represents an ask on the marketplace
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Offer {
    /// Unique identifier
    pub id: u8,

    /// Arrays of offered & wanted NFTs, both defined by the sender
    pub offered_nfts: Vec<Token>,
    pub wanted_nfts: Vec<Token>,

    pub sender: Addr,
    pub peer: Addr,
    pub expires_at: Timestamp,
}

// Incrementing ID counter
pub const OFFER_ID_COUNTER: Item<u8> = Item::new("offer_id_counter");

// Get next incrementing ID
pub fn next_offer_id(store: &mut dyn Storage) -> StdResult<u8> {
    let id: u8 = OFFER_ID_COUNTER.may_load(store)?.unwrap_or_default() + 1;
    OFFER_ID_COUNTER.save(store, &id)?;

    Ok(id)
}

pub const OFFER_NAMESPACE: &str = "offers";
pub struct OfferIndexes<'a> {
    pub id: UniqueIndex<'a, u8, Offer>,
}
impl<'a> IndexList<Offer> for OfferIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Offer>> + '_> {
        let v: Vec<&dyn Index<Offer>> = vec![&self.id];
        Box::new(v.into_iter())
    }
}

// Function to get all offers and manipulate offer data
pub fn offers<'a>() -> IndexedMap<'a, &'a [u8], Offer, OfferIndexes<'a>> {
    let indexes = OfferIndexes {
        id: UniqueIndex::new(|d| d.id, "offer_id"),
    };
    IndexedMap::new(OFFER_NAMESPACE, indexes)
}
