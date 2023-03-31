#[cfg(test)]
use cosmwasm_std::{coins, Addr, Coin, Empty, Timestamp};
use cw721::{Cw721QueryMsg, OwnerOfResponse};
use cw721_base::msg::{ExecuteMsg as Cw721ExecuteMsg, MintMsg};
use cw_utils::Expiration;
use pegasus_trade::pegasus::ExpiryRange;
use sg721::msg::InstantiateMsg as Sg721InstantiateMsg;
use sg721::state::CollectionInfo;

use cw_multi_test::{BankSudo, Contract, ContractWrapper, Executor, SudoMsg as CwSudoMsg};
use sg_multi_test::StargazeApp;

use sg_std::{StargazeMsgWrapper, NATIVE_DENOM};

use crate::msg::{ExecuteMsg, OffersResponse, QueryMsg, TokenMsg};
use crate::ContractError;

const CREATOR: &str = "creator";
const COLLECTION_A: &str = "collection-a";
// const COLLECTION_B: &str = "collection-b";
const TOKEN1_ID: u32 = 123;
const TOKEN2_ID: u32 = 234;
const TOKEN3_ID: u32 = 345;
const TOKEN4_ID: u32 = 456;

const SENDER: &str = "sender";
// const SENDER2: &str = "sender";
const PEER: &str = "peer";
const MAX_EXPIRY: u64 = 604800;
const MIN_EXPIRY: u64 = 86400;

fn custom_mock_app() -> StargazeApp {
    StargazeApp::default()
}

pub fn contract_p2p_trade() -> Box<dyn Contract<StargazeMsgWrapper>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    )
    .with_sudo(crate::contract::sudo);
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
        offer_expiry: ExpiryRange {
            min: MIN_EXPIRY,
            max: MAX_EXPIRY,
        },
        maintainer: CREATOR.to_string(),
        max_offers: 16,
        bundle_limit: 3,
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
        collection_info: CollectionInfo {
            creator: CREATOR.to_string(),
            description: "test".to_string(),
            image: "ipfs://test".to_string(),
            external_link: None,
            royalty_info: None,
        },
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
        collection_info: CollectionInfo {
            creator: CREATOR.to_string(),
            description: "test".to_string(),
            image: "ipfs://test".to_string(),
            external_link: None,
            royalty_info: None,
        },
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

    println!(
        "collection_a: {:?}, collection_b: {:?}",
        collection_a, collection_b
    );

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
    expires: Option<Expiration>,
) {
    let approve_msg = Cw721ExecuteMsg::<Empty>::Approve {
        spender: marketplace.to_string(),
        token_id: token_id.to_string(),
        expires: expires,
    };
    let res = router.execute_contract(creator.clone(), collection.clone(), &approve_msg, &[]);
    assert!(res.is_ok());
}

fn _transfer(
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

    mint_for(router, &creator, &creator, &collection_a, TOKEN3_ID);
    mint_for(router, &creator, &creator, &collection_a, TOKEN4_ID);

    //  ----- TestCase for empty offered/wanted nft vector ----
    let exec_create_msg = ExecuteMsg::CreateOffer {
        offered_nfts: vec![],
        wanted_nfts: vec![TokenMsg {
            collection: collection_a.to_string(),
            token_id: TOKEN2_ID,
        }],
        offered_balances: vec![],
        message: None,
        peer: peer.to_string(),
        expires_at: None,
    };

    // empty offer is not allowed
    let err = router
        .execute_contract(
            sender.clone(),
            trade_contract.clone(),
            &exec_create_msg,
            &[],
        )
        .unwrap_err();
    assert_eq!(
        err.downcast::<ContractError>().unwrap(),
        ContractError::EmptyTokenVector {}
    );
    // -----------

    // ------ TESTCASE: Exceeds bundle limit ----------
    let exec_create_msg = ExecuteMsg::CreateOffer {
        offered_nfts: vec![
            TokenMsg {
                collection: collection_a.to_string(),
                token_id: TOKEN1_ID,
            },
            TokenMsg {
                collection: collection_a.to_string(),
                token_id: TOKEN1_ID,
            },
            TokenMsg {
                collection: collection_a.to_string(),
                token_id: TOKEN1_ID,
            },
            TokenMsg {
                collection: collection_a.to_string(),
                token_id: TOKEN1_ID,
            },
        ],
        wanted_nfts: vec![TokenMsg {
            collection: collection_a.to_string(),
            token_id: TOKEN2_ID,
        }],
        offered_balances: vec![],
        message: None,
        peer: peer.to_string(),
        expires_at: None,
    };

    // sender should fail to create a offer if the nfts are not approved yet
    let err = router
        .execute_contract(
            sender.clone(),
            trade_contract.clone(),
            &exec_create_msg,
            &[],
        )
        .unwrap_err();
    assert_eq!(
        err.downcast::<ContractError>().unwrap(),
        ContractError::MaxBundle { limit: 3 }
    );
    // ------------

    // ------ TESTCASE: Non-approved NFT's ----------
    let exec_create_msg = ExecuteMsg::CreateOffer {
        offered_nfts: vec![TokenMsg {
            collection: collection_a.to_string(),
            token_id: TOKEN1_ID,
        }],
        wanted_nfts: vec![TokenMsg {
            collection: collection_a.to_string(),
            token_id: TOKEN2_ID,
        }],
        offered_balances: vec![],
        message: None,
        peer: peer.to_string(),
        expires_at: None,
    };

    // sender should fail to create a offer if the nfts are not approved yet
    let err = router
        .execute_contract(
            sender.clone(),
            trade_contract.clone(),
            &exec_create_msg,
            &[],
        )
        .unwrap_err();
    assert_eq!(
        err.downcast::<ContractError>().unwrap(),
        ContractError::Unauthorized {
            collection: collection_a.to_string(),
            token_id: TOKEN1_ID
        }
    );
    // ------------

    // Approves contract on the sender side
    approve(
        router,
        &sender,
        &collection_a,
        &trade_contract,
        TOKEN1_ID,
        None,
    );
    approve(
        router,
        &sender,
        &collection_b,
        &trade_contract,
        TOKEN1_ID,
        None,
    );

    let res = router.execute_contract(
        sender.clone(),
        trade_contract.clone(),
        &exec_create_msg,
        &[],
    );
    assert!(res.is_ok(), "Offer should be correct.");

    let query_msg = QueryMsg::OffersBySender {
        sender: sender.to_string(),
    };
    let qres: OffersResponse = router
        .wrap()
        .query_wasm_smart(trade_contract.clone(), &query_msg)
        .unwrap();
    assert_eq!(qres.offers.len(), 1);
    let on_chain_offer = qres.offers.first().unwrap();
    assert_eq!(
        on_chain_offer.offered_nfts.first().unwrap().token_id,
        TOKEN1_ID
    );

    // if the token is already being offered by the sender in another offer, the tx should fail
    let exec_create_not_owned_by_peer_msg = ExecuteMsg::CreateOffer {
        offered_nfts: vec![TokenMsg {
            collection: collection_a.to_string(),
            token_id: TOKEN1_ID,
        }],
        wanted_nfts: vec![TokenMsg {
            collection: collection_a.to_string(),
            token_id: TOKEN2_ID,
        }],
        offered_balances: vec![],
        message: None,
        peer: peer.to_string(),
        expires_at: None,
    };

    let err = router
        .execute_contract(
            sender.clone(),
            trade_contract.clone(),
            &exec_create_not_owned_by_peer_msg,
            &[],
        )
        .unwrap_err();
    assert_eq!(
        err.downcast::<ContractError>().unwrap(),
        ContractError::TokenAlreadyOffered {
            collection: collection_a.to_string(),
            token_id: TOKEN1_ID,
            offer_id: 1
        }
    );

    // if the sender doesnt own the nfts it is trying to offer, the tx should fail
    let exec_create_not_owned_by_sender_msg = ExecuteMsg::CreateOffer {
        offered_nfts: vec![TokenMsg {
            collection: collection_a.to_string(),
            token_id: TOKEN3_ID,
        }],
        wanted_nfts: vec![TokenMsg {
            collection: collection_a.to_string(),
            token_id: TOKEN2_ID,
        }],
        offered_balances: vec![],
        message: None,
        peer: peer.to_string(),
        expires_at: None,
    };

    let err = router
        .execute_contract(
            sender.clone(),
            trade_contract.clone(),
            &exec_create_not_owned_by_sender_msg,
            &[],
        )
        .unwrap_err();
    assert_eq!(
        err.downcast::<ContractError>().unwrap(),
        ContractError::UnauthorizedSender {}
    );

    // if the peer doesnt own the nfts wanted by the sender, the offer should fail
    let exec_create_not_owned_by_peer_msg = ExecuteMsg::CreateOffer {
        offered_nfts: vec![TokenMsg {
            collection: collection_b.to_string(),
            token_id: TOKEN1_ID,
        }],
        wanted_nfts: vec![TokenMsg {
            collection: collection_a.to_string(),
            token_id: TOKEN3_ID,
        }],
        offered_balances: vec![],
        message: None,
        peer: peer.to_string(),
        expires_at: None,
    };

    let err = router
        .execute_contract(
            sender.clone(),
            trade_contract.clone(),
            &exec_create_not_owned_by_peer_msg,
            &[],
        )
        .unwrap_err();
    assert_eq!(
        err.downcast::<ContractError>().unwrap(),
        ContractError::UnauthorizedPeer {
            collection: collection_a.to_string(),
            token_id: TOKEN3_ID,
            peer: peer.to_string()
        }
    );

    approve(
        router,
        &peer,
        &collection_a,
        &trade_contract,
        TOKEN2_ID,
        None,
    );
}

#[test]
fn accept_offer() {
    let router = &mut custom_mock_app();
    setup_block_time(router, 1000);

    let (sender, peer, creator) = setup_accounts(router).unwrap();
    let (trade_contract, collection_a, collection_b) = setup_contracts(router, &creator).unwrap();

    mint_for(router, &sender, &creator, &collection_a, TOKEN1_ID);
    mint_for(router, &sender, &creator, &collection_b, TOKEN1_ID);

    mint_for(router, &peer, &creator, &collection_a, TOKEN2_ID);
    mint_for(router, &peer, &creator, &collection_b, TOKEN2_ID);

    mint_for(router, &creator, &creator, &collection_a, TOKEN3_ID);
    mint_for(router, &creator, &creator, &collection_a, TOKEN4_ID);

    let exec_create_msg = ExecuteMsg::CreateOffer {
        offered_nfts: vec![TokenMsg {
            collection: collection_a.to_string(),
            token_id: TOKEN1_ID,
        }],
        wanted_nfts: vec![TokenMsg {
            collection: collection_a.to_string(),
            token_id: TOKEN2_ID,
        }],
        offered_balances: vec![],
        message: None,
        peer: peer.to_string(),
        expires_at: None,
    };

    // Approves contract on the sender side
    approve(
        router,
        &sender,
        &collection_a,
        &trade_contract,
        TOKEN1_ID,
        Some(Expiration::AtTime(Timestamp::from_seconds(1900))),
    );

    let res = router.execute_contract(
        sender.clone(),
        trade_contract.clone(),
        &exec_create_msg,
        &[],
    );
    // Offer should now be created properly.
    assert!(res.is_ok(), "Offer should be correct.");

    let exec_accept_msg = ExecuteMsg::AcceptOffer { id: 1 };
    // test when peer accepts without approval on peer side
    let err = router
        .execute_contract(peer.clone(), trade_contract.clone(), &exec_accept_msg, &[])
        .unwrap_err();
    assert_eq!(
        err.downcast::<ContractError>().unwrap(),
        ContractError::Unauthorized {
            collection: collection_a.to_string(),
            token_id: TOKEN2_ID
        }
    );

    // approve on the peer side
    approve(
        router,
        &peer,
        &collection_a,
        &trade_contract,
        TOKEN2_ID,
        None,
    );

    // // test when peer accepts without approval on sender side
    // setup_block_time(router, 2000); // approval should be expired by now
    // let err = router.execute_contract(peer.clone(), trade_contract.clone(), &exec_accept_msg, &[]).unwrap_err();
    // assert_eq!(
    //     err.downcast::<ContractError>().unwrap(),
    //     ContractError::UnauthorizedOperator {  }
    // );

    // approve again on the sender side
    approve(
        router,
        &sender,
        &collection_a,
        &trade_contract,
        TOKEN1_ID,
        None,
    );

    let res = router.execute_contract(peer.clone(), trade_contract, &exec_accept_msg, &[]);
    assert!(res.is_ok());

    // check if the NFTs are transfered properly
    let owner_of_1_query_msg = Cw721QueryMsg::OwnerOf {
        token_id: TOKEN1_ID.to_string(),
        include_expired: None,
    };
    let owner_of_2_query_msg = Cw721QueryMsg::OwnerOf {
        token_id: TOKEN2_ID.to_string(),
        include_expired: None,
    };

    let res1: OwnerOfResponse = router
        .wrap()
        .query_wasm_smart(collection_a.clone(), &owner_of_1_query_msg)
        .unwrap();
    let res2: OwnerOfResponse = router
        .wrap()
        .query_wasm_smart(collection_a, &owner_of_2_query_msg)
        .unwrap();

    assert_eq!(res1.owner, peer.to_string());
    assert_eq!(res2.owner, sender.to_string());

    // test when peer accepts whithout ownership first send

    // test if the contract
}
