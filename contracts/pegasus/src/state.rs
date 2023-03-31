use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, StdResult, Storage, Timestamp};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex, UniqueIndex};
use pegasus_trade::pegasus::{ExpiryRange, Offer, SudoParams};
pub const MIN_EXPIRY: u64 = 3600 * 24; // seconds -> one day
pub const MAX_EXPIRY: u64 = 3600 * 24 * 28; // seconds -> one monthst M

pub const SUDO_PARAMS: Item<SudoParams> = Item::new("sudo-params");
// Incrementing ID counter
pub const OFFER_ID_COUNTER: Item<u64> = Item::new("offer_id_counter");

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
