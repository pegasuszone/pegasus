use crate::state::{MAX_EXPIRY, SUDO_PARAMS};
use crate::{error::ContractError, state::MIN_EXPIRY};
use cosmwasm_std::{DepsMut, Env};
use pegasus::cw721_trade::{ExpiryRange, ExpiryRangeError};
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
        offer_expiry.validate()?;

        if offer_expiry.min < MIN_EXPIRY {
            return Err(ContractError::ExpiryRange(
                ExpiryRangeError::InvalidExpirationRange {},
            ));
        }
        if offer_expiry.max > MAX_EXPIRY {
            return Err(ContractError::ExpiryRange(
                ExpiryRangeError::InvalidExpirationRange {},
            ));
        }

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
