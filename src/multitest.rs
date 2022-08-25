#[cfg(test)]
use cosmwasm_std::testing::{mock_dependencies, mock_env};
use cosmwasm_std::{Addr, Timestamp, Uint128, coins, Coin, Empty};
use sg721::msg::{QueryMsg};
use cw721_base::msg::{ExecuteMsg as Cw721ExecuteMsg, MintMsg};
use sg721::msg::{InstantiateMsg as Sg721InstantiateMsg, RoyaltyInfoResponse};
use sg721::state::CollectionInfo;

use cw_multi_test::{ContractWrapper, Contract, Executor, BankSudo, SudoMsg as CwSudoMsg};
use sg_multi_test::StargazeApp;

use sg_std::{NATIVE_DENOM, StargazeMsgWrapper};

use crate::ContractError;
use crate::msg::{ExecuteMsg, TokenMsg};

const CREATOR: &str = "creator";
const COLLECTION_A: &str = "collection-a";
const COLLECTION_B: &str = "collection-b";
const TOKEN1_ID: u32 = 123;
const TOKEN2_ID: u32 = 234;
const TOKEN3_ID: u32 = 345;
const TOKEN4_ID: u32 = 456;

const SENDER: &str = "sender";
// const SENDER2: &str = "sender";
const PEER: &str = "peer";
const MAX_EXPIRY: u64 = 60;
const MIN_EXPIRY: u64 = 60;


fn custom_mock_app() -> StargazeApp {
    StargazeApp::default()
}

pub fn contract_p2p_trade() -> Box<dyn Contract<StargazeMsgWrapper>> {
    let contract = ContractWrapper::new(
        crate::execute::execute,
        crate::execute::instantiate,
        crate::query::query,
    )
    .with_sudo(crate::sudo::sudo);
    Box::new(contract)
}

pub fn contract_sg721() -> Box<dyn Contract<StargazeMsgWrapper>> {
    let contract = ContractWrapper::new(
        sg721::contract::execute,
        sg721::contract::instantiate,
        sg721::contract::query,
    );
    Box::new(contract)
}

fn setup_block_time(router: &mut StargazeApp, seconds: u64) {
    let mut block = router.block_info();
    block.time = Timestamp::from_seconds(seconds);
    router.set_block(block);
}

fn setup_contracts(
    router: &mut StargazeApp,
    creator: &Addr,
) -> Result<(Addr, Addr, Addr), ContractError> {
    // Instantiate marketplace contract
    let p2p_contract_id = router.store_code(contract_p2p_trade());
    let msg = crate::msg::InstantiateMsg {
        escrow_deposit_amount: Uint128::new(0),
        offer_expiry: crate::ExpiryRange { min: 60, max: 60*10 },
        maintainer: CREATOR.to_string(),
        removal_reward_bps: 0,
    };
    let p2p_trade = router
        .instantiate_contract(
            p2p_contract_id,
            creator.clone(),
            &msg,
            &[],
            "p2pTrade",
            Some(CREATOR.to_string()),
        )
        .unwrap();
    println!("marketplace: {:?}", p2p_trade);

    // Setup media contract
    let sg721_id = router.store_code(contract_sg721());
    let msg = Sg721InstantiateMsg {
        name: COLLECTION_A.to_string(),
        symbol: "MAU".to_string(),
        minter: CREATOR.to_string(),
        collection_info: CollectionInfo { creator: CREATOR.to_string(), description: "test".to_string(), image: "ipfs://test".to_string(), external_link: None, royalty_info: None },
    };
    
    let collection_a = router
        .instantiate_contract(
            sg721_id,
            creator.clone(),
            &msg,
            &coins(1_000_000_000, NATIVE_DENOM),
            "NFT",
            Some(creator.to_string()),
        )
        .unwrap();

           // Setup media contract
    let sg721_id = router.store_code(contract_sg721());
    let msg = Sg721InstantiateMsg {
        name: COLLECTION_A.to_string(),
        symbol: "MAU".to_string(),
        minter: CREATOR.to_string(),
        collection_info: CollectionInfo { creator: CREATOR.to_string(), description: "test".to_string(), image: "ipfs://test".to_string(), external_link: None, royalty_info: None },
    };
    let collection_b = router
    .instantiate_contract(
        sg721_id,
        creator.clone(),
        &msg,
        &coins(1_000_000_000, NATIVE_DENOM),
        "NFT",
        Some(creator.to_string()),
    )
    .unwrap();

    println!("collection_a: {:?}, collection_b: {:?}", collection_a, collection_b);

    Ok((p2p_trade, collection_a, collection_b))
}

// Intializes accounts with balances
fn setup_accounts(router: &mut StargazeApp) -> Result<(Addr, Addr, Addr), ContractError> {
    let sender: Addr = Addr::unchecked(SENDER);
    let peer: Addr = Addr::unchecked(PEER);
    let creator: Addr = Addr::unchecked(CREATOR);
    let creator_funds: Vec<Coin> = coins(1_000_000_000_000, NATIVE_DENOM);
    let funds: Vec<Coin> = coins(2_000_000_000, NATIVE_DENOM);
    router
        .sudo(CwSudoMsg::Bank({
            BankSudo::Mint {
                to_address: sender.to_string(),
                amount: funds.clone(),
            }
        }))
        .map_err(|err| println!("{:?}", err))
        .ok();
    router
        .sudo(CwSudoMsg::Bank({
            BankSudo::Mint {
                to_address: peer.to_string(),
                amount: funds.clone(),
            }
        }))
        .map_err(|err| println!("{:?}", err))
        .ok();
    router
        .sudo(CwSudoMsg::Bank({
            BankSudo::Mint {
                to_address: creator.to_string(),
                amount: creator_funds.clone(),
            }
        }))
        .map_err(|err| println!("{:?}", err))
        .ok();

    // Check native balances
    let owner_native_balances = router.wrap().query_all_balances(sender.clone()).unwrap();
    assert_eq!(owner_native_balances, funds);
    let bidder_native_balances = router.wrap().query_all_balances(peer.clone()).unwrap();
    assert_eq!(bidder_native_balances, funds);
    let creator_native_balances = router.wrap().query_all_balances(creator.clone()).unwrap();
    assert_eq!(creator_native_balances, creator_funds);

    Ok((sender, peer, creator))
}


fn mint_for(
    router: &mut StargazeApp,
    owner: &Addr,
    minter: &Addr,
    collection: &Addr,
    token_id: u32,
) {
    let mint_for_creator_msg = Cw721ExecuteMsg::Mint(MintMsg {
        token_id: token_id.to_string(),
        owner: owner.to_string(),
        token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
        extension: Empty {},
    });
    let res = router.execute_contract(
        minter.clone(),
        collection.clone(),
        &mint_for_creator_msg,
        &[],
    );
    assert!(res.is_ok());
}

fn approve(
    router: &mut StargazeApp,
    creator: &Addr,
    collection: &Addr,
    marketplace: &Addr,
    token_id: u32,
) {
    let approve_msg = Cw721ExecuteMsg::<Empty>::Approve {
        spender: marketplace.to_string(),
        token_id: token_id.to_string(),
        expires: None,
    };
    let res = router.execute_contract(creator.clone(), collection.clone(), &approve_msg, &[]);
    assert!(res.is_ok());
}

fn transfer(
    router: &mut StargazeApp,
    creator: &Addr,
    recipient: &Addr,
    collection: &Addr,
    token_id: u32,
) {
    let transfer_msg = Cw721ExecuteMsg::<Empty>::TransferNft {
        recipient: recipient.to_string(),
        token_id: token_id.to_string(),
    };
    let res = router.execute_contract(creator.clone(), collection.clone(), &transfer_msg, &[]);
    assert!(res.is_ok());
}

#[test]
fn create_offer() {
    let router = &mut custom_mock_app();


    let (sender, peer, creator) = setup_accounts(router).unwrap();

    let (trade_contract, collection_a, collection_b) = setup_contracts(router, &creator).unwrap();

    mint_for(router, &sender, &creator, &collection_a, TOKEN1_ID);
    mint_for(router, &sender, &creator, &collection_b, TOKEN1_ID);

    mint_for(router, &peer, &creator, &collection_a, TOKEN2_ID);
    mint_for(router, &peer, &creator, &collection_b, TOKEN2_ID);

    let offer_msg = ExecuteMsg::CreateOffer { 
        offered_nfts: vec![TokenMsg { collection: collection_a.to_string(), token_id: TOKEN1_ID}], 
        wanted_nfts: vec![TokenMsg {collection: collection_a.to_string(), token_id: TOKEN2_ID}], 
        peer: peer.to_string(), expires_at: None };
    
    // sender should fail to create a offer if the nfts are not approved yet
    let err = router.execute_contract(sender.clone(), trade_contract.clone(), &offer_msg, &[]).unwrap_err();
    assert_eq!(
        err.downcast::<ContractError>().unwrap(),
        ContractError::Unauthorized { collection: collection_a.to_string(), token_id: TOKEN1_ID }
    );

    approve(router, &sender, &collection_a, &trade_contract, TOKEN1_ID);
    approve(router, &sender, &collection_b, &trade_contract, TOKEN1_ID);
    
    
    
    
    
    
    
    approve(router, &peer, &collection_a, &trade_contract, TOKEN2_ID);
    approve(router, &peer, &collection_b, &trade_contract, TOKEN2_ID);
}
