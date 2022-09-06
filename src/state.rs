use cosmwasm_std::{Addr, StdResult, Storage, Timestamp, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, UniqueIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::helpers::ExpiryRange;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SudoParams {
    /// Amount in micros to be deposited by the sender of an offer
    /// This escrow will be refunded when the offer is accepted or denied
    /// The sender will lose this deposit if they let the offer expire
    pub escrow_deposit_amount: Uint128, // TODO: This in not implemented yet
    /// Valid time range for Offers
    /// (min, max) in seconds
    pub offer_expiry: ExpiryRange,

    /// Developer address
    pub maintainer: Addr,

    /// Stale trade removal reward, // TODO: This in not implemented yet
    pub removal_reward_bps: u64,

    /// Maximum amount of offers a user can send
    pub max_offers: u64,
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
    pub created_at: Timestamp,
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
