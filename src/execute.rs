use crate::error::ContractError;
use crate::msg::TokenMsg;
use crate::query::query_offers_by_sender;
use crate::state::{next_offer_id, offers, Offer, Token, SUDO_PARAMS};
// use crate::query::{query_offers_by_sender};

use cosmwasm_std::{to_binary, Addr, Deps, DepsMut, Env, MessageInfo, SubMsg, Timestamp, WasmMsg};
use cw721::{Cw721ExecuteMsg, OwnerOfResponse};
use cw721_base::helpers::Cw721Contract;
use sg_std::Response;

pub fn execute_create_offer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    offered_tokens: Vec<TokenMsg>,
    wanted_tokens: Vec<TokenMsg>,
    peer: Addr,
    expires_at: Option<Timestamp>,
) -> Result<Response, ContractError> {
    if info.sender == peer {
        // TODO: This error needs refactor: Dont know how to describe this situation. SelfSend?
        return Err(ContractError::AlreadyOwned {});
    }

    if offered_tokens.is_empty() {
        return Err(ContractError::EmptyTokenVector {});
    }
    if wanted_tokens.is_empty() {
        return Err(ContractError::EmptyTokenVector {});
    }

    let offers_from_sender = query_offers_by_sender(deps.as_ref(), info.sender.clone())?;
    let params = SUDO_PARAMS.load(deps.storage)?;

    let api = deps.api;

    // Return an error if the amount of offers by this user + 1 exceeds the limit of active offers
    if (offers_from_sender.offers.len() as u64) + 1 > params.max_offers {
        return Err(ContractError::MaxOffers {
            addr: info.sender.to_string(),
            max_offers: params.max_offers,
        });
    }

    // Return an error if the bundle size exceeds the bundle limit
    if (offered_tokens.len() as u64) > params.bundle_limit
        || (wanted_tokens.len() as u64) > params.bundle_limit
    {
        return Err(ContractError::MaxBundle {
            limit: params.bundle_limit,
        });
    }

    // Store data we're fetching in the next 2 loops for performance
    let mut offered_nfts: Vec<Token> = vec![];
    let mut wanted_nfts: Vec<Token> = vec![];

    // check if the sender is the owner of the tokens
    // TODO: Consider a different order of checks: Now, you might get a not approved error, after which you approved, but actually there is another error, like the peer is not the right owner.
    //          Then you've approved the contract but no offer has been made, which feels a bit unsafe.
    for token in offered_tokens.clone() {
        // Verify token collection addr
        let collection = api.addr_validate(&token.collection)?;

        let token = Token {
            collection,
            token_id: token.token_id,
        };

        offered_nfts.push(token.clone());

        // TODO: [OPTIMISATION] See if we can levarage the OwnerOfResponse.Approvals for checking if the contract has been approved
        only_owner(deps.as_ref(), &info, &token.collection, token.token_id)?;

        // check if the contract is approved to send transfer the tokens
        Cw721Contract(token.collection.clone())
            .approval(
                &deps.querier,
                token.token_id.to_string(),
                env.contract.address.to_string(),
                None,
            )
            .map_err(|_| ContractError::Unauthorized {
                collection: token.collection.to_string(),
                token_id: token.token_id,
            })?;

        // check if the tokens arent already offered in another trade
        for offer in offers_from_sender.clone().offers {
            if offer.offered_nfts.contains(&token) {
                return Err(ContractError::TokenAlreadyOffered {
                    collection: token.collection.into_string(),
                    token_id: token.token_id,
                    offer_id: offer.id,
                });
            }
        }
    }

    // check if the peer is the owner of the requested tokens
    for token in wanted_tokens.clone() {
        // Verify token collection addr
        let collection = api.addr_validate(&token.collection)?;

        let token = Token {
            collection,
            token_id: token.token_id,
        };

        wanted_nfts.push(token.clone());

        if peer
            != Cw721Contract(token.collection.clone())
                .owner_of(&deps.querier, token.token_id.to_string(), false)?
                .owner
        {
            return Err(ContractError::UnauthorizedPeer {
                collection: token.collection.to_string(),
                token_id: token.token_id,
                peer: peer.into_string(),
            });
        }
    }
    let params = SUDO_PARAMS.load(deps.storage)?;
    // check if the expiry date is valid
    let expires =
        expires_at.unwrap_or_else(|| env.block.time.plus_seconds(params.offer_expiry.min + 1));
    params
        .offer_expiry
        .is_valid(&env.block, env.block.time, expires)?;

    // create and save offer
    let offer = Offer {
        id: next_offer_id(deps.storage)?,
        offered_nfts: offered_nfts,
        wanted_nfts: wanted_nfts,
        sender: info.sender,
        peer,
        expires_at: expires,
        created_at: env.block.time,
    };
    offers().save(deps.storage, offer.id, &offer)?;

    Ok(Response::new()
        .add_attribute("action", "create_offer")
        .add_attribute("offer_id", offer.id.to_string())
        .add_attribute("offer_sender", offer.sender)
        .add_attribute("offer_peer", offer.peer))
}

pub fn execute_remove_offer(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
) -> Result<Response, ContractError> {
    // check if the sender of this msg is the sender of the offer
    let offer = offers().load(deps.as_ref().storage, id)?;
    if offer.sender != info.sender {
        return Err(ContractError::UnauthorizedSender {});
    }

    offers().remove(deps.storage, offer.id)?;

    // TODO: Remove approvals

    Ok(Response::new()
        .add_attribute("action", "revoke_offer")
        .add_attribute("offer_id", offer.id.to_string())
        .add_attribute("offer_sender", offer.sender)
        .add_attribute("offer_peer", offer.peer))
}

pub fn execute_accept_offer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: u64,
) -> Result<Response, ContractError> {
    let offer = offers().load(deps.storage, id)?;

    let params = SUDO_PARAMS.load(deps.storage)?;

    // check if the sender is the peer of the offer
    if offer.peer != info.sender {
        return Err(ContractError::UnauthorizedSender {});
    }

    // check if the offer is not yet expired
    params
        .offer_expiry
        .is_valid(&env.block, offer.created_at, offer.expires_at)?;

    // check if the sender owns the requested nfts
    for token in offer.wanted_nfts.clone() {
        only_owner(deps.as_ref(), &info, &token.collection, token.token_id)?;

        // check if the contract is approved to send transfer the tokens
        Cw721Contract(token.collection.clone())
            .approval(
                &deps.querier,
                token.token_id.to_string(),
                env.contract.address.to_string(),
                None,
            )
            .map_err(|_| ContractError::Unauthorized {
                collection: token.collection.to_string(),
                token_id: token.token_id,
            })?;
    }

    // check if the offeror owns the offered nfts
    for token in offer.offered_nfts.clone() {
        if offer.sender
            != Cw721Contract(token.collection.clone())
                .owner_of(&deps.querier, token.token_id.to_string(), false)?
                .owner
        {
            return Err(ContractError::UnauthorizedPeer {
                collection: token.collection.into_string(),
                token_id: token.token_id,
                peer: offer.sender.to_string(),
            });
        }

        // check if the contract is approved to send transfer the tokens
        Cw721Contract(token.collection.clone())
            .approval(
                &deps.querier,
                token.token_id.to_string(),
                env.contract.address.to_string(),
                None,
            )
            .map_err(|_| ContractError::UnauthorizedOperator {})?;
    }
    let mut res = Response::new();

    // remove the offer
    offers().remove(deps.storage, offer.id)?;

    // transfer nfts
    transfer_nfts(offer.peer.to_string(), offer.offered_nfts.clone(), &mut res)?;
    transfer_nfts(offer.sender.to_string(), offer.wanted_nfts, &mut res)?;

    Ok(res
        .add_attribute("action", "accept_offer")
        .add_attribute("offer_id", offer.id.to_string())
        .add_attribute("offer_sender", offer.sender)
        .add_attribute("offer_peer", offer.peer))
}

pub fn transfer_nfts(
    recipient: String,
    nfts: Vec<Token>,
    res: &mut cosmwasm_std::Response<sg_std::StargazeMsgWrapper>,
) -> Result<(), ContractError> {
    for token in nfts {
        let cw721_transfer_msg = Cw721ExecuteMsg::TransferNft {
            recipient: recipient.clone(),
            token_id: token.token_id.to_string(),
        };
        let exec_cw721_transfer_msg = WasmMsg::Execute {
            contract_addr: token.collection.to_string(),
            msg: to_binary(&cw721_transfer_msg)?,
            funds: vec![],
        };

        res.messages.push(SubMsg::new(exec_cw721_transfer_msg));
    }
    Ok(())
}

pub fn execute_reject_offer(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
) -> Result<Response, ContractError> {
    // check if the sender of this msg is the peer of the offer
    let offer = offers().load(deps.as_ref().storage, id)?;
    if offer.peer != info.sender {
        return Err(ContractError::UnauthorizedOperator {});
    }
    // TODO: Remove approvals

    offers().remove(deps.storage, offer.id)?;

    Ok(Response::new()
        .add_attribute("action", "reject_offer")
        .add_attribute("offer_id", offer.id.to_string())
        .add_attribute("offer_sender", offer.sender)
        .add_attribute("offer_peer", offer.peer))
}

pub fn execute_remove_stale_offer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: u64,
) -> Result<Response, ContractError> {
    let params = SUDO_PARAMS.load(deps.storage)?;

    if info.sender != params.maintainer {
        return Err(ContractError::UnauthorizedOperator {});
    }

    let offer = offers().load(deps.storage, id)?;

    params
        .offer_expiry
        .is_valid(&env.block, offer.created_at, offer.expires_at)?;

    offers().remove(deps.storage, id)?;

    Ok(Response::new()
        .add_attribute("action", "remove_stale_offer")
        .add_attribute("offer_id", offer.id.to_string())
        .add_attribute("offer_sender", offer.sender)
        .add_attribute("offer_peer", offer.peer))
}

// ---------------------------------------------------------------------------------
// helper functions
// ---------------------------------------------------------------------------------

/// Checks to enfore only NFT owner can call
fn only_owner(
    deps: Deps,
    info: &MessageInfo,
    collection: &Addr,
    token_id: u32,
) -> Result<OwnerOfResponse, ContractError> {
    let res =
        Cw721Contract(collection.clone()).owner_of(&deps.querier, token_id.to_string(), false)?;
    if res.owner != info.sender {
        return Err(ContractError::UnauthorizedSender {});
    }

    Ok(res)
}

// fn finalize_trade(deps: Deps, offered: Vec<Token>) {}
