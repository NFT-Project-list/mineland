use std::collections::HashMap;
use near_sdk::json_types::{U128};
use near_sdk_sim::{call, to_yocto, view};

use main::{MineType, Stone};

use crate::utils::{claim_mine, claim_stones, init};
use nft_stone::JsonToken;

#[test]
fn mint_stones() {
    let (_, main_contract, _, _, stone_contract, alice) = init();
    let mine = claim_mine(&main_contract, &alice, MineType::Small);

    // Check stone claim time
    let stone_claim_time: HashMap<String, u64> = view!(
        main_contract.stone_claim_time(
            alice.valid_account_id(),
            1000000000000000000.into()
        )
    ).unwrap_json();
    assert_eq!(&0, stone_claim_time.get(&mine.token_id).unwrap());

    // Claim stones
    let stones = claim_stones(&main_contract, &alice, mine.token_id.to_string(), mine.mine_type);
    assert_eq!(1, stones.len());

    // Check metadata
    let metadata_result: Vec<JsonToken> = view!(
        stone_contract.nft_tokens_for_owner(alice.account_id(), Some(U128::from(0)), Some(5 as u64))
    ).unwrap_json();
    assert_eq!(metadata_result.len(), 1);
    assert_eq!(metadata_result[0].owner_id, alice.account_id());
    assert_eq!(metadata_result[0].metadata.title.as_ref().unwrap(), &"Stone #1".to_string());

    // Check total stones count
    let total_stone_count: u32 = view!(
        main_contract.minted_stones_count()
    ).unwrap_json();
    assert_eq!(1, total_stone_count);

    // Check stone claim time after mint
    let stone_claim_time: HashMap<String, u64> = view!(
        main_contract.stone_claim_time(
            alice.valid_account_id(),
            1000000000000000000.into()
        )
    ).unwrap_json();
    assert_ne!(&0, stone_claim_time.get(&mine.token_id).unwrap());
}

#[test]
fn mint_second_stone_error() {
    let (_, main_contract, _, _, _, alice) = init();
    let mine = claim_mine(&main_contract, &alice, MineType::Small);

    // First Claim
    claim_stones(&main_contract, &alice, mine.token_id.to_string(), mine.mine_type);

    // Second claim - expect error
    let stone_claim_result = call!(
        alice,
        main_contract.mint_free_stone_nft(mine.token_id.to_string()),
        deposit = to_yocto("0.01")
    );
    assert!(!stone_claim_result.is_ok());
}

#[test]
fn user_kill_stone() {
    let (_, main_contract, ft_contract, _, _, alice) = init();
    let mine = claim_mine(&main_contract, &alice, MineType::Large);

    // Mint stones on large mine (8 stones)
    claim_stones(&main_contract, &alice, mine.token_id.to_string(), mine.mine_type);

    let (total_count, user_stones): (String, Vec<Stone>) = view!(
        main_contract.user_stones(alice.account_id(), 1.into(), 40.into(), None, None)
    ).unwrap_json();
    assert_eq!(total_count, 8.to_string());
    assert_eq!(user_stones.len(), 8);

    // Kill stone
    let kill_tokens_result: String = call!(
        alice,
        main_contract.stone_kill(user_stones[0].token_id.to_string()),
        deposit=1
    ).unwrap_json();

    // Check user stones count
    let (total_count, user_stones): (String, Vec<Stone>) = view!(
        main_contract.user_stones(alice.account_id(), 1.into(), 40.into(), None, None)
    ).unwrap_json();
    assert_eq!(total_count, 7.to_string());
    assert_eq!(user_stones.len(), 7);

    // Check ft token balance
    let user_balance: String = view!(
        ft_contract.ft_balance_of(alice.valid_account_id())
    ).unwrap_json();
    assert_eq!(kill_tokens_result, user_balance);

    // Check Killed stone count
    let killed_total_count: u32 = view!(
        main_contract.killed_stones_count()
    ).unwrap_json();
    assert_eq!(1, killed_total_count);
}
