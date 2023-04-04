use boot_core::*;
use cosmwasm_std::{Addr, Decimal, Empty, DepsMut, testing::{mock_info, mock_env}};
use cw721_base::{InstantiateMsg as Cw721InstantiateMsg, Cw721Contract};
use cw_multi_test::{App, ContractWrapper, Executor};
use pegasus_boot::cw721_trade::Cw721Trade;
use sg_multi_test::mock_deps;
use std::str::FromStr;

pub fn store_sg721(app: &mut App) -> u64 {
    let contract = ContractWrapper::new_with_empty(
        cw721_base::Cw721Contract
        sg721::contract::execute,
        sg721::contract::instantiate,
        sg721::contract::query,
    );
    Box::new(contract);
    app.store_code(contract)
}
fn setup_contract(deps: DepsMut<'_>, mock: Mock) -> Cw721Contract<'static, Extension, Empty> {
    let contract = Cw721Contract::default();
    let msg = Cw721InstantiateMsg {
        name: "collection1".to_string(),
        symbol: "C1".to_string(),
        minter: String::from("owner"),
    };
        contract.instantiate(mock_deps(), mock_env(), info, msg);
    
    let info = mock_info("owner", &[]);
    let res = contract.instantiate(deps, mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    contract
}

pub struct Suite {
    pub owner: Addr,
    pub collection1: Addr,
    pub collection2: Addr,
    pub collection3: Addr,
    pub pegasus: Cw721Trade<Mock>,
    mock: Mock,
}
#[derive(Debug)]
pub struct SuiteBuilder {}

impl Default for SuiteBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SuiteBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self, mock_chain: &Mock) -> Suite {
        let mut app = App::default();
        let owner = Addr::unchecked("owner");
        let cw721_id = store_sg721(&mut app);
        let collection1 = app
            .instantiate_contract(
                cw721_id,
                owner.clone(),
                &Cw721InstantiateMsg {
                    name: "collection1".to_string(),
                    symbol: "collection1".to_string(),
                    minter: owner.to_string(),
                },
                &vec![],
                "collection1",
                None,
            )
            .unwrap();

        let collection2 = app
            .instantiate_contract(
                cw721_id,
                owner.clone(),
                &Cw721InstantiateMsg {
                    name: "collection2".to_string(),
                    symbol: "collection2".to_string(),
                    minter: owner.to_string(),
                },
                &vec![],
                "collection2",
                None,
            )
            .unwrap();

        let collection3 = app
            .instantiate_contract(
                cw721_id,
                owner.clone(),
                &Cw721InstantiateMsg {
                    name: "collection3".to_string(),
                    symbol: "collection3".to_string(),
                    minter: owner.to_string(),
                },
                &vec![],
                "collection3",
                None,
            )
            .unwrap();

        let pegasus = Cw721Trade::new("pegasus", mock_chain);

        drop(app);
        Suite {
            owner,
            collection1,
            collection2,
            collection3,
            pegasus,
            mock,
        }
    }
}
