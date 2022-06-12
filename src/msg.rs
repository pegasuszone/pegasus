use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw721::Cw721ReceiveMsg;


// instantiate msg sets the owner of the contract 
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub offer_id: String,
    pub requested_id: String,
    pub peer_addr: Option<String>,
    pub expiry_date: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Owner or peer can send the cw721 to the contract.
    // Alternative could be that the owner of the contract enables the contract to send the NFT from the owner to the receiver. In that way the nft would never be owned by the contract.
    ReceiveNft(Cw721ReceiveMsg),
    
    CancelTrade {},
    
    AcceptTrade (Cw721ReceiveMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // get the id of the token that is requested
    RequestId {},
    
    // get the id of the token that is offered
    OfferId{},

    // get ownerid
    Owner{},

    // get optional peer id
    Peer{},

    // Get all the trade data
    TradeData{},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenIdResponse {
    pub token_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AddrResponse {
    pub addr: String,
}

pub struct TradeDataResponse {
    pub requested_token_id: String,
    pub offered_token_id: String,
    pub owner_addr: String,
    pub peer_addres: String,
    pub expiry_date: String,
}
