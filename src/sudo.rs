use crate::error::ContractError;
use crate::helpers::ExpiryRange;
use crate::state::SUDO_PARAMS;
use cosmwasm_std::{DepsMut, Env};
use sg_std::Response;

pub struct ParamInfo {
    pub offer_expiry: Option<ExpiryRange>,
    pub maintainer: Option<String>,
    pub max_offers: Option<u64>,
    pub bundle_limit: Option<u64>,
}

/// Only governance can update contract params
pub fn sudo_update_params(
    deps: DepsMut,
    _env: Env,
    param_info: ParamInfo,
) -> Result<Response, ContractError> {
    let ParamInfo {
        offer_expiry,
        maintainer,
        max_offers,
        bundle_limit,
    } = param_info;

    let mut params = SUDO_PARAMS.load(deps.storage)?;

    if let Some(offer_expiry) = offer_expiry {
        params.offer_expiry = offer_expiry;
    }

    if let Some(maintainer) = maintainer {
        params.maintainer = deps.api.addr_validate(&maintainer)?;
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
