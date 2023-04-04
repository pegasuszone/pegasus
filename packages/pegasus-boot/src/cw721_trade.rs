use boot_core::*;
use cw721_base::msg::ExecuteMsg as BaseCw721ExecuteMsg;
use cw721_base::{
    InstantiateMsg as BaseCw721InstantiateMsg, MintMsg, QueryMsg as BaseCw721QueryMsg,
};
use pegasus::cw721_trade::{
    ExecuteMsg as TradeExecuteMsg, InstantiateMsg as TradeInstantiateMsg,
    MigrateMsg as TradeMigrateMsg, QueryMsg as TradeQueryMsg,
};

// use sg721::{ExecuteMsg as Sg721ExecuteMsgGen, InstantiateMsg as Sg721InstantiateMsg};

// #[cfg_attr(feature = "boot", derive(boot_core::ExecuteFns))]
// #[cfg_attr(feature = "boot", impl_into(ExecuteMsg))]
// pub(crate) type Cw721ExecuteMsg = GCw721ExecuteMsg<Empty>;

// #[cfg_attr(feature = "boot", derive(boot_core::QueryFns))]
// #[cfg_attr(feature = "boot", impl_into(QueryMsg))]
// pub(crate) type Cw721QueryMsg = Cw721QueryMsg<Empty>;

// #[cfg_attr(feature = "boot", derive(boot_core::ExecuteFns))]
// #[cfg_attr(feature = "boot", impl_into(ExecuteMsg))]
// pub(crate) type Sg721ExecuteMsg = Sg721ExecuteMsgGen<Empty, Empty>;

// #[cfg_attr(feature = "boot", derive(boot_core::QueryFns))]
// #[cfg_attr(feature = "boot", impl_into(QueryMsg))]
// pub(crate) type Sg721QueryMsg = BaseCw721QueryMsg;

#[boot_contract(TradeInstantiateMsg, TradeExecuteMsg, TradeQueryMsg, TradeMigrateMsg)]
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

// #[boot_contract(Cw721InstantiateMsg, Cw721ExecuteMsg, Cw721QueryMsg, Empty)]
// pub struct Cw721<Chain>;

// impl<Chain: BootEnvironment> Cw721<Chain>
// where
//     TxResponse<Chain>: IndexResponse,
// {
//     pub fn new(name: &str, chain: Chain) -> Self {
//         Self(Contract::new(name, chain).with_wasm_path("cw721_base"))
//     }

//     pub fn load(chain: Chain, address: &Addr) -> Self {
//         Self(Contract::new("cw721", chain).with_address(Some(address)))
//     }
// }

// #[boot_contract(Sg721InstantiateMsg, Sg721ExecuteMsg, Sg721QueryMsg, Empty)]
// pub struct Sg721<Chain>;

// impl<Chain: BootEnvironment> Sg721<Chain>
// where
//     TxResponse<Chain>: IndexResponse,
// {
//     pub fn new(name: &str, chain: Chain) -> Self {
//         Self(Contract::new(name, chain).with_wasm_path("sg721"))
//     }

//     pub fn load(chain: Chain, address: &Addr) -> Self {
//         Self(Contract::new("sg721", chain).with_address(Some(address)))
//     }
// }
