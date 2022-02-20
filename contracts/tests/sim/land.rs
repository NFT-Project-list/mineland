use std::collections::HashMap;
use near_sdk::json_types::{U128};
use near_sdk_sim::{call, to_yocto, view};

use main::{Mine, MineType};

use crate::utils::{claim_mine, init};
use nft_mine::JsonToken;

#[test]
fn mint_mines() {
    let (_, main_contract, _, mine_contract, _, alice) = init();

    // Small Mine - test owner
    let small_mine = claim_mine(&main_contract, &alice, MineType::Small);
    let medium_mine = claim_mine(&main_contract, &alice, MineType::Medium);
    let large_mine = claim_mine(&main_contract, &alice, MineType::Large);

    assert_eq!(small_mine.last_stone_claim, 0);
    assert_eq!(small_mine.token_id.to_string().chars().nth(0).unwrap().to_string(), "s".to_string());
    assert_eq!(small_mine.mine_type, MineType::Small);
    assert_eq!(small_mine.sale_price, None);
    assert_eq!(medium_mine.mine_type, MineType::Medium);
    assert!(large_mine.token_id.to_string().len() > 5);

    // Check metadata
    let metadata_result: Vec<JsonToken> = view!(
        mine_contract.nft_tokens_for_owner(alice.account_id(), Some(U128::from(0)), Some(5 as u64))
    ).unwrap_json();
    assert_eq!(metadata_result.len(), 3);
    assert_eq!(metadata_result[0].owner_id, alice.account_id());
    assert_eq!(metadata_result[0].metadata.title.as_ref().unwrap(), &"Small Mine #1".to_string());

    // Check user mines count
    let user_mines: Vec<Mine> = view!(
        main_contract.user_mines(alice.valid_account_id())
    ).unwrap_json();
    assert_eq!(3, user_mines.len());

    // Check total mines count
    let total_mines: HashMap<String, u32> = view!(
        main_contract.total_mines_count()
    ).unwrap_json();
    assert_eq!(&1, total_mines.get("Small").unwrap());
    assert_eq!(&1, total_mines.get("Medium").unwrap());
    assert_eq!(&1, total_mines.get("Large").unwrap());
}

#[test]
fn mint_second_small_mine_error() {
    let (_, main_contract, _, _, _, alice) = init();

    claim_mine(&main_contract, &alice, MineType::Small);

    // Claim second mine - should return error
    let small_mine_result_err = call!(
        alice,
        main_contract.mint_mine_nft(),
        to_yocto("0.01"),
        near_sdk_sim::DEFAULT_GAS
    );
    assert!(!small_mine_result_err.is_ok());
}

