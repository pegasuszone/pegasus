use crate::error::ContractError;
use crate::execute::{
    execute_accept_offer, execute_create_offer, execute_reject_offer, execute_remove_offer,
    execute_remove_stale_offer,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg, TokenMsg};
use crate::query::{query_offer, query_offers_by_peer, query_offers_by_sender};
use crate::state::{SudoParams, Token, SUDO_PARAMS};
use crate::sudo::{sudo_update_params, ParamInfo};
use crate::ExpiryRangeError;

// use crate::query::{query_offers_by_sender};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, StdError, StdResult,
};
use cw2::set_contract_version;
use sg_std::Response;

// Version info for migration info
const CONTRACT_NAME: &str = "crates.io:pegasus";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const MIN_EXPIRY: u64 = 3600 * 24; // seconds -> one day
const MAX_EXPIRY: u64 = 3600 * 24 * 28; // seconds -> one month

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    if msg.offer_expiry.min < MIN_EXPIRY {
        return Err(ContractError::ExpiryRange(
            ExpiryRangeError::InvalidExpirationRange {},
        ));
    }
    if msg.offer_expiry.max > MAX_EXPIRY {
        return Err(ContractError::ExpiryRange(
            ExpiryRangeError::InvalidExpirationRange {},
        ));
    }

    let params = SudoParams {
        escrow_deposit_amount: msg.escrow_deposit_amount,
        offer_expiry: msg.offer_expiry,
        maintainer: deps.api.addr_validate(&msg.maintainer)?,
        removal_reward_bps: msg.removal_reward_bps,
        max_offers: msg.max_offers,
        bundle_limit: msg.bundle_limit,
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
        ExecuteMsg::CreateOffer {
            offered_nfts,
            wanted_nfts,
            peer,
            expires_at,
        } => execute_create_offer(
            deps,
            env,
            info,
            offered_nfts
                .into_iter()
                .map(|nft: TokenMsg| Token {
                    collection: api.addr_validate(&nft.collection).unwrap(),
                    token_id: nft.token_id,
                })
                .collect(),
            wanted_nfts
                .into_iter()
                .map(|nft: TokenMsg| Token {
                    collection: api.addr_validate(&nft.collection).unwrap(),
                    token_id: nft.token_id,
                })
                .collect(),
            api.addr_validate(&peer)?,
            expires_at,
        ),

        ExecuteMsg::RemoveOffer { id } => execute_remove_offer(deps, info, id),
        ExecuteMsg::AcceptOffer { id } => execute_accept_offer(deps, env, info, id),
        ExecuteMsg::RejectOffer { id } => execute_reject_offer(deps, info, id),
        ExecuteMsg::RemoveStaleOffer { id } => execute_remove_stale_offer(deps, env, info, id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let api = deps.api;

    match msg {
        QueryMsg::Offer { id } => to_binary(&query_offer(deps, id)?),
        QueryMsg::OffersBySender { sender } => {
            to_binary(&query_offers_by_sender(deps, api.addr_validate(&sender)?)?)
        }
        QueryMsg::OffersByPeer { peer } => {
            to_binary(&query_offers_by_peer(deps, api.addr_validate(&peer)?)?)
        }
        QueryMsg::Params {} => todo!(),
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    let ver = cw2::get_contract_version(deps.storage)?;
    // ensure we are migrating from an allowed contract
    if ver.contract != CONTRACT_NAME {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }
    // note: better to do proper semver compare, but string compare *usually* works
    if *ver.version >= *CONTRACT_VERSION {
        return Err(StdError::generic_err("Cannot upgrade from a newer version").into());
    }

    // set the new version
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // do any desired state migrations...

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    // let api = deps.api;

    match msg {
        SudoMsg::UpdateParams {
            escrow_deposit_amount,
            offer_expiry,
            maintainer,
            removal_reward_bps,
            max_offers,
            bundle_limit
        } => sudo_update_params(
            deps,
            env,
            ParamInfo {
                escrow_deposit_amount,
                offer_expiry,
                maintainer,
                removal_reward_bps,
                max_offers,
                bundle_limit
            },
        ),
    }
}
