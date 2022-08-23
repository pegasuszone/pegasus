use crate::msg::{
    OfferCountResponse, OfferOffset, QueryMsg, OffersResponse,
};
use crate::state::{
    offer_key, Offer, OfferKey, OFFER_HOOKS, SUDO_PARAMS, TokenId, offers
};
// use crate::state::{
//     ask_key, asks, bid_key, bids, collection_bid_key, collection_bids, BidKey, CollectionBidKey,
//     TokenId, ASK_HOOKS, BID_HOOKS, SALE_HOOKS, SUDO_PARAMS,
// };
use cosmwasm_std::{entry_point, to_binary, Addr, Binary, Deps, Env, Order, StdResult};
use cw_storage_plus::{Bound, PrefixBound};
use cw_utils::maybe_addr;

// Query limits
const DEFAULT_QUERY_LIMIT: u32 = 10;
const MAX_QUERY_LIMIT: u32 = 30;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let api = deps.api;

    match msg {
    QueryMsg::CollectionsWanted { start_after, limit } => todo!(),
    QueryMsg::CollectionsOffered { start_after, limit } => todo!(),
    QueryMsg::Offer { collection, token_id } => todo!(),
    QueryMsg::Offers { offeror, start_after, limit } => todo!(),
    QueryMsg::Requests { peer, start_after, limit } => todo!(),
    QueryMsg::OfferHooks {  } => todo!(),
    QueryMsg::Params {  } => todo!(),
}
}

pub fn query_offers(
    deps: Deps,
    offeror: Addr,
    start_after: Option<TokenId>,
    limit: Option<u32>,
) -> StdResult<OffersResponse> {
    let limit = limit.unwrap_or(DEFAULT_QUERY_LIMIT).min(MAX_QUERY_LIMIT) as usize;

    let offers = offers()
        .idx
        .offeror
        .prefix(offeror.clone())
        .range(
            deps.storage,
            Some(Bound::exclusive((
                offeror,
                start_after.unwrap_or_default(),
            ))),
            None,
            Order::Ascending,
        )
        .filter(|item| match item {
            Ok((_, ask)) => match include_inactive {
                Some(true) => true,
                _ => ask.is_active,
            },
            Err(_) => true,
        })
        .take(limit)
        .map(|res| res.map(|item| item.1))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(AsksResponse { asks })
}

pub fn query_asks_sorted_by_price(
    deps: Deps,
    collection: Addr,
    owner: Addr,
    include_inactive: Option<bool>,
    start_after: Option<OfferOffset>,
    limit: Option<u32>,
) -> StdResult<AsksResponse> {
    let limit = limit.unwrap_or(DEFAULT_QUERY_LIMIT).min(MAX_QUERY_LIMIT) as usize;

    let start = start_after.map(|offset| {
        Bound::exclusive((offset.price.u128(), offer_key(&collection, offset.token_id, owner)))
    });

    let asks = offers()
        .idx
        .collection_price
        .sub_prefix(collection)
        .range(deps.storage, start, None, Order::Ascending)
        .filter(|item| match item {
            Ok((_, ask)) => match include_inactive {
                Some(true) => true,
                _ => ask.is_active,
            },
            Err(_) => true,
        })
        .take(limit)
        .map(|res| res.map(|item| item.1))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(AsksResponse { asks })
}