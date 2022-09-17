use crate::msg::{OfferResponse, OffersResponse, ParamsResponse};
use crate::state::{offers, SUDO_PARAMS};
use cosmwasm_std::{Addr, Deps, Order, StdResult};

// Query limits
// const DEFAULT_QUERY_LIMIT: u32 = 10;
// const MAX_QUERY_LIMIT: u32 = 30;

pub fn query_offer(deps: Deps, id: u64) -> StdResult<OfferResponse> {
    let offer = offers().may_load(deps.storage, id)?;
    Ok(OfferResponse { offer })
}

pub fn query_params(deps: Deps) -> StdResult<ParamsResponse> {
    let params = SUDO_PARAMS.load(deps.storage)?;
    Ok(ParamsResponse { params })
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
