use crate::error::ContractError;
use crate::msg::TokenMsg;
use crate::query::query_offers_by_sender;
use crate::state::{next_offer_id, offers, Offer, Royalty, Token, SUDO_PARAMS};
// use crate::query::{query_offers_by_sender};

use cosmwasm_std::{
    coin, to_binary, Addr, BankMsg, Coin, Deps, DepsMut, Env, MessageInfo, StdResult, SubMsg,
    Timestamp, WasmMsg,
};
use cw721::{Cw721ExecuteMsg, OwnerOfResponse};
use cw721_base::helpers::Cw721Contract;
use cw_utils::must_pay;
use sg721::msg::{CollectionInfoResponse, QueryMsg as Sg721QueryMsg};
use sg_std::{Response, StargazeMsgWrapper};

#[allow(clippy::too_many_arguments)]
pub fn execute_create_offer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    offered_tokens: Vec<TokenMsg>,
    wanted_tokens: Vec<TokenMsg>,
    offered_balances: Vec<Coin>,
    message: Option<String>,
    peer: Addr,
    expires_at: Option<Timestamp>,
) -> Result<Response, ContractError> {
    if info.sender == peer {
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

    // check if the expiry date is valid
    let expires =
        expires_at.unwrap_or_else(|| env.block.time.plus_seconds(params.offer_expiry.min + 1));
    params
        .offer_expiry
        .is_valid(&env.block, env.block.time, expires)?;

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

    // Verify that the offered balances have been sent to the contract
    for balance in offered_balances.clone() {
        let amount_paid = must_pay(&info, &balance.denom)?;
        if amount_paid < balance.amount {
            return Err(ContractError::Payment(cw_utils::PaymentError::NoFunds {}));
        }
    }

    // If there are any native/ibc tokens involved in the tx, we need to enforce
    // creator royalties on both the NFTs that are offered and requested.
    // The tokens used for royalties will be initially stored in the contract and
    // will be distributed to creators when the offer is confirmed.
    // This array will store the royalty amounts to be paid out.
    let mut royalties: Vec<Royalty> = vec![];

    // Are royalties enforced on this transaction?
    let royalties_enforced = offered_balances.len() > 1;

    // Store data we're fetching in the next 2 loops for performance
    let mut offered_nfts: Vec<Token> = vec![];
    let mut wanted_nfts: Vec<Token> = vec![];

    // check if the peer is the owner of the requested nfts
    for token in wanted_tokens {
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

        // If royalties are to be enforced,
        // figure out the amount of royalties to pay out.
        if royalties_enforced {
            let collection_info: CollectionInfoResponse = deps
                .querier
                .query_wasm_smart(token.collection.clone(), &Sg721QueryMsg::CollectionInfo {})?;
            if let Some(royalty_info) = collection_info.royalty_info {
                for balance in offered_balances.clone() {
                    // Return an error if insufficient amount paid
                    let to_pay = royalty_info.share * balance.amount;
                    let amount_paid = must_pay(&info, &balance.denom)?;
                    if amount_paid < to_pay {
                        return Err(ContractError::InsufficientRoyalties {});
                    }

                    royalties.push(Royalty {
                        creator: deps.api.addr_validate(&royalty_info.payment_address)?,
                        amount: coin(to_pay.u128(), balance.denom),
                    });
                }
            }
        }
    }

    // check if the sender is the owner of the nfts
    for token in offered_tokens {
        // Verify token collection addr
        let collection = api.addr_validate(&token.collection)?;

        let token = Token {
            collection,
            token_id: token.token_id,
        };

        offered_nfts.push(token.clone());

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

        // If royalties are to be enforced,
        // figure out the amount of royalties to pay out.
        if royalties_enforced {
            let collection_info: CollectionInfoResponse = deps
                .querier
                .query_wasm_smart(token.collection.clone(), &Sg721QueryMsg::CollectionInfo {})?;
            if let Some(royalty_info) = collection_info.royalty_info {
                for balance in offered_balances.clone() {
                    // Return an error if insufficient amount paid
                    let to_pay = royalty_info.share * balance.amount;
                    let amount_paid = must_pay(&info, &balance.denom)?;
                    if amount_paid < to_pay {
                        return Err(ContractError::InsufficientRoyalties {});
                    }

                    royalties.push(Royalty {
                        creator: deps.api.addr_validate(&royalty_info.payment_address)?,
                        amount: coin(to_pay.u128(), balance.denom),
                    });
                }
            }
        }
    }

    // create and save offer
    let offer = Offer {
        id: next_offer_id(deps.storage)?,
        offered_nfts,
        wanted_nfts,
        offered_balances,
        message,
        royalties,
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

    let mut msgs = vec![];

    // Return any funds held by the contract to the sender
    for balance in offer.offered_balances {
        msgs.push(send_tokens(offer.sender.clone(), balance)?);
    }
    for royalty in offer.royalties {
        msgs.push(send_tokens(offer.sender.clone(), royalty.amount)?);
    }

    offers().remove(deps.storage, offer.id)?;

    Ok(Response::new()
        .add_attribute("action", "revoke_offer")
        .add_attribute("offer_id", offer.id.to_string())
        .add_attribute("offer_sender", offer.sender)
        .add_attribute("offer_peer", offer.peer)
        .add_submessages(msgs))
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

    // transfer funds to peer
    let mut send_msgs = vec![];
    for balance in offer.offered_balances {
        send_msgs.push(send_tokens(offer.peer.clone(), balance)?);
    }

    // transfer royalties to creators
    let mut royalty_msgs = vec![];
    for royalty in offer.royalties {
        royalty_msgs.push(send_tokens(royalty.creator, royalty.amount)?);
    }

    Ok(res
        .add_attribute("action", "accept_offer")
        .add_attribute("offer_id", offer.id.to_string())
        .add_attribute("offer_sender", offer.sender)
        .add_attribute("offer_peer", offer.peer)
        .add_submessages(send_msgs)
        .add_submessages(royalty_msgs))
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

    let mut msgs = vec![];

    // Return any funds held by the contract to the sender
    for balance in offer.offered_balances {
        msgs.push(send_tokens(offer.sender.clone(), balance)?);
    }
    for royalty in offer.royalties {
        msgs.push(send_tokens(offer.sender.clone(), royalty.amount)?);
    }

    offers().remove(deps.storage, offer.id)?;

    Ok(Response::new()
        .add_attribute("action", "reject_offer")
        .add_attribute("offer_id", offer.id.to_string())
        .add_attribute("offer_sender", offer.sender)
        .add_attribute("offer_peer", offer.peer)
        .add_submessages(msgs))
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

    let mut msgs = vec![];

    // Return any funds held by the contract to the sender
    for balance in offer.offered_balances {
        msgs.push(send_tokens(offer.sender.clone(), balance)?);
    }
    for royalty in offer.royalties {
        msgs.push(send_tokens(offer.sender.clone(), royalty.amount)?);
    }

    offers().remove(deps.storage, id)?;

    Ok(Response::new()
        .add_attribute("action", "remove_stale_offer")
        .add_attribute("offer_id", offer.id.to_string())
        .add_attribute("offer_sender", offer.sender)
        .add_attribute("offer_peer", offer.peer)
        .add_submessages(msgs))
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

// Send native tokens to another address
pub fn send_tokens(to: Addr, balance: Coin) -> StdResult<SubMsg<StargazeMsgWrapper>> {
    let msg = BankMsg::Send {
        to_address: to.into_string(),
        amount: vec![balance],
    };

    let exec = SubMsg::<StargazeMsgWrapper>::new(msg);

    Ok(exec)
}

// fn finalize_trade(deps: Deps, offered: Vec<Token>) {}
