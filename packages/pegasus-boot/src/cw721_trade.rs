use boot_core::*;
use pegasus::cw721_trade::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

#[boot_contract(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct Cw721Trade<Chain>;

impl<Chain: BootEnvironment> Cw721Trade<Chain>
where
    TxResponse<Chain>: IndexResponse,
{
    pub fn new(name: &str, chain: Chain) -> Self {
        Self(Contract::new(name, chain).with_wasm_path("cw721_trade"))
    }

    pub fn load(chain: Chain, address: &Addr) -> Self {
        Self(Contract::new("cw721_trade", chain).with_address(Some(address)))
    }
}
