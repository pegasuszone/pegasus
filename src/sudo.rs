use crate::error::ContractError;
use crate::helpers::ExpiryRange;
use crate::state::SUDO_PARAMS;
use cosmwasm_std::{DepsMut, Env, Uint128};
use sg_std::Response;

pub struct ParamInfo {
    pub escrow_deposit_amount: Option<Uint128>,
    pub offer_expiry: Option<ExpiryRange>,
    pub maintainer: Option<String>,
    pub removal_reward_bps: Option<u64>,
    pub max_offers: Option<u64>,
    pub bundle_limit: Option<u64>
}

/// Only governance can update contract params
pub fn sudo_update_params(
    deps: DepsMut,
    _env: Env,
    param_info: ParamInfo,
) -> Result<Response, ContractError> {
    let ParamInfo {
        escrow_deposit_amount,
        offer_expiry,
        maintainer,
        removal_reward_bps,
        max_offers,
        bundle_limit,
    } = param_info;

    let mut params = SUDO_PARAMS.load(deps.storage)?;

    params.escrow_deposit_amount = escrow_deposit_amount.unwrap_or_else(Uint128::zero);

    if let Some(offer_expiry) = offer_expiry {
        params.offer_expiry = offer_expiry;
    }

    if let Some(maintainer) = maintainer {
        params.maintainer = deps.api.addr_validate(&maintainer)?;
    }
    if let Some(removal_reward_bps) = removal_reward_bps {
        params.removal_reward_bps = removal_reward_bps
        // .map(Decimal::percent)
        // .unwrap_or(params.bid_removal_reward_percent);
    }

    if let Some(max_offers) = max_offers {
        params.max_offers = max_offers
    }

    if let Some(bundle_limit) = bundle_limit {
        params.bundle_limit = bundle_limit
    }

    SUDO_PARAMS.save(deps.storage, &params)?;

    Ok(Response::new().add_attribute("action", "update_params"))
}
