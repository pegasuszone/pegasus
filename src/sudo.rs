use crate::error::ContractError;
use crate::helpers::{map_validate, ExpiryRange};
use crate::msg::SudoMsg;
use crate::state::SUDO_PARAMS;
use cosmwasm_std::{entry_point, Addr, Decimal, DepsMut, Env, Uint128};
use cw_utils::Duration;
use sg_std::Response;

pub struct ParamInfo {
    pub escrow_deposit_amount: Option<Uint128>,
    pub offer_expiry: Option<ExpiryRange>,
    pub maintainer: Option<Addr>,
    pub removal_reward_percent: Option<u64>,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    let api = deps.api;

    match msg {
        SudoMsg::UpdateParams {
            escrow_deposit_amount,
            offer_expiry,
            maintainer,
            removal_reward_bps,
        } => sudo_update_params(
            deps,
            env,
            ParamInfo {
                escrow_deposit_amount,
                offer_expiry,
                maintainer,
                removal_reward_bps,
            },
        ),
    }
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
    } = param_info;

    let mut params = SUDO_PARAMS.load(deps.storage)?;

    params.escrow_deposit_amount = escrow_deposit_amount.unwrap_or(Uint128::zero());

    if let Some(offer_expiry) = offer_expiry {
        params.offer_expiry = offer_expiry;
    }

    if let Some(maintainer) = maintainer {
        params.maintainer = maintainer;
    }

    params.removal_reward_bps = bid_removal_reward_bps
        .map(Decimal::percent)
        .unwrap_or(params.bid_removal_reward_percent);

    SUDO_PARAMS.save(deps.storage, &params)?;

    Ok(Response::new().add_attribute("action", "update_params"))
}
