use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Timestamp};

use cw_storage_plus::Item;

use crate::helpers::ExpiryRange;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SudoParams {
    offer_expiry: ExpiryRange
}

pub struct Offer {
    pub offeror: Addr,
    pub offer_collection: Addr,
    pub offer_token_id: String,
    pub return_collection: String,
    pub return_token_id: String,
    pub expires: Timestamp,
    // the peer can be made optional in later versions
    pub peer: Addr,
}

// impl Order for Offer {
//     
// }

// TODO: 



pub const SUDO_PARAMS: Item<SudoParams> = Item::new("sudo-params");



// pub const STATE: Item<State> = Item::new("state");
