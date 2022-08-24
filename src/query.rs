use crate::msg::{OfferResponse, OffersResponse, QueryMsg};
use crate::state::offers;
use cosmwasm_std::{entry_point, to_binary, Addr, Binary, Deps, Env, Order, StdResult};

// Query limits
const DEFAULT_QUERY_LIMIT: u32 = 10;
const MAX_QUERY_LIMIT: u32 = 30;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let api = deps.api;

    match msg {
        QueryMsg::Offer { id } => to_binary(&query_offer(deps, id)?),
        QueryMsg::OffersBySender { sender } => to_binary(&query_offers_by_sender(deps, sender)?),
        QueryMsg::OffersByPeer { peer } => to_binary(&query_offers_by_peer(deps, peer)?),
        QueryMsg::Params {} => todo!(),
    }
}

pub fn query_offer(deps: Deps, id: u8) -> StdResult<OfferResponse> {
    let offer = offers().may_load(deps.storage, &[id])?;
    Ok(OfferResponse { offer })
}

pub fn query_offers_by_sender(deps: Deps, sender: Addr) -> StdResult<OffersResponse> {
    let offers = offers()
        .idx
        .id
        .range(deps.storage, None, None, Order::Ascending)
        .filter(|item| match item {
            Ok((_, offer)) => offer.sender == sender,
            Err(_) => false,
        })
        .map(|res| res.map(|item| item.1))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(OffersResponse { offers })
}

pub fn query_offers_by_peer(deps: Deps, peer: Addr) -> StdResult<OffersResponse> {
    let offers = offers()
        .idx
        .id
        .range(deps.storage, None, None, Order::Ascending)
        .filter(|item| match item {
            Ok((_, offer)) => offer.peer == peer,
            Err(_) => false,
        })
        .map(|res| res.map(|item| item.1))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(OffersResponse { offers })
}
