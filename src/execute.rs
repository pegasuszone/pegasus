use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{SudoParams, TokenId, SUDO_PARAMS, Token};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr, Coin, DepsMut, Env, MessageInfo, Timestamp, Deps};
use cw2::set_contract_version;
use sg_std::Response;

// Version info for migration info
const CONTRACT_NAME: &str = "crates.iosg-p2p-nft-trade";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let params = SudoParams { 
        escrow_deposit_amount: msg.escrow_deposit_amount, 
        offer_expiry: msg.offer_expiry, 
        maintainer: deps.api.addr_validate(&msg.maintainer)?, 
        removal_reward_bps: msg.removal_reward_bps 
    };
    SUDO_PARAMS.save(deps.storage, &params)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let api = deps.api;

    match msg {
    ExecuteMsg::CreateOffer { offered_nfts, wanted_nfts, peer, expires_at } => todo!(),
    ExecuteMsg::RemoveOffer { id } => todo!(),
    ExecuteMsg::AcceptOffer { id } => todo!(),
    ExecuteMsg::RefuseOffer { id } => todo!(),
    ExecuteMsg::RemoveStaleOffer { id } => todo!(),
}
}


fn execute_create_offer(deps:DepsMut, env: Env, offered_tokens: Vec<Token>, wanted_tokens: Vec<Token>, peer: Addr, expires_at: Timestamp ) -> Result<Response, ContractError> {
    

}


fn finalize_trade(deps: Deps, offered: Vec<Token>) {}