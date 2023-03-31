use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Addr, BlockInfo, StdError, StdResult, Timestamp, WasmMsg};
use pegasus::cw721_trade::ExecuteMsg;
use sg_std::CosmosMsg;

/// MarketplaceContract is a wrapper around Addr that provides a lot of helpers
#[cw_serde]
pub struct MarketplaceContract(pub Addr);

impl MarketplaceContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }
}
