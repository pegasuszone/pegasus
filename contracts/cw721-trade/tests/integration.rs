use std::str::FromStr;

use abstract_boot::boot_core::*;
use boot_core::Mock;
use cosmwasm_std::{Addr, Decimal, Empty};

use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
use cw_multi_test::{App, ContractWrapper, Executor};

use pegasus_boot::cw721_trade::{Cw721Trade, Sg721};

pub fn setup_cw721() -> anyhow::Result<Sg721<Mock>> {
    let owner = Addr::unchecked("owner");

    let (_, mock_chain) = instantiate_default_mock_env(&owner)?;

    let mut cw721_contract = Sg721::new("cw721", mock_chain);
    let app = mock_chain.app.borrow_mut();
    let cw721_id = store_sg721(&mut app);

    let cw721 = app.instantiate_contract(
        cw721_id,
        owner.clone(),
        &Cw721InstantiateMsg {
            name: "ColA".to_string(),
            symbol: "A".to_string(),
            minter: None,
        },
        &vec![],
        "cw721",
        Some(owner.to_string()),
    )?;

    cw721_contract.as_instance_mut().set_mock(Box::new(
        ContractWrapper::new_with_empty(
            sg721::contract::execute,
            sg721::contract::instantiate,
            sg721::contract::query,
        )
        .into(),
    ));

    cw721_contract.upload();
    Ok(cw721_contract)
}

// fn setup_contracts(
//     router: &mut StargazeApp,
//     creator: &Addr,
// ) -> Result<(Addr, Addr, Addr), ContractError> {
//     // Instantiate marketplace contract
//     let p2p_contract_id = router.store_code(contract_p2p_trade());
//     let msg = crate::msg::InstantiateMsg {
//         offer_expiry: ExpiryRange {
//             min: MIN_EXPIRY,
//             max: MAX_EXPIRY,
//         },
//         maintainer: CREATOR.to_string(),
//         max_offers: 16,
//         bundle_limit: 3,
//     };
//     let p2p_trade = router
//         .instantiate_contract(
//             p2p_contract_id,
//             creator.clone(),
//             &msg,
//             &[],
//             "p2pTrade",
//             Some(CREATOR.to_string()),
//         )
//         .unwrap();
//     println!("marketplace: {:?}", p2p_trade);

//     // Setup media contract
//     let sg721_id = router.store_code(contract_sg721());
//     let msg = Sg721InstantiateMsg {
//         name: COLLECTION_A.to_string(),
//         symbol: "MAU".to_string(),
//         minter: CREATOR.to_string(),
//         collection_info: CollectionInfo {
//             creator: CREATOR.to_string(),
//             description: "test".to_string(),
//             image: "ipfs://test".to_string(),
//             external_link: None,
//             royalty_info: None,
//         },
//     };

//     let collection_a = router
//         .instantiate_contract(
//             sg721_id,
//             creator.clone(),
//             &msg,
//             &coins(1_000_000_000, NATIVE_DENOM),
//             "NFT",
//             Some(creator.to_string()),
//         )
//         .unwrap();

//     // Setup media contract
//     let sg721_id = router.store_code(contract_sg721());
//     let msg = Sg721InstantiateMsg {
//         name: COLLECTION_A.to_string(),
//         symbol: "MAU".to_string(),
//         minter: CREATOR.to_string(),
//         collection_info: CollectionInfo {
//             creator: CREATOR.to_string(),
//             description: "test".to_string(),
//             image: "ipfs://test".to_string(),
//             external_link: None,
//             royalty_info: None,
//         },
//     };
//     let collection_b = router
//         .instantiate_contract(
//             sg721_id,
//             creator.clone(),
//             &msg,
//             &coins(1_000_000_000, NATIVE_DENOM),
//             "NFT",
//             Some(creator.to_string()),
//         )
//         .unwrap();

//     println!(
//         "collection_a: {:?}, collection_b: {:?}",
//         collection_a, collection_b
//     );

//     Ok((p2p_trade, collection_a, collection_b))
// }
