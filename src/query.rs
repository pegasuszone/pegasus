use crate::msg::{OfferResponse, OffersResponse, ContractVersionResponse};
use crate::state::offers;
use cosmwasm_std::{Addr, Deps, Order, StdResult};
use cw2::get_contract_version;

// Query limits
// const DEFAULT_QUERY_LIMIT: u32 = 10;
// const MAX_QUERY_LIMIT: u32 = 30;

pub fn query_offer(deps: Deps, id: u8) -> StdResult<OfferResponse> {
    let offer = offers().may_load(deps.storage, &[id])?;
    Ok(OfferResponse { offer })
}

// TODO: Implement pagination
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

// TODO: Implement pagination (Although people wont have a lot of open offers)
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

pub fn query_contract_version(deps:Deps) -> StdResult<ContractVersionResponse> {
    let ver = get_contract_version(deps.storage)?;
    Ok( ContractVersionResponse {
        contract: ver.contract,
        version: ver.version,
    })
}