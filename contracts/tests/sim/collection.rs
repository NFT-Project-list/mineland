use std::collections::HashMap;
use near_sdk::json_types::{U128};
use near_sdk_sim::{call, to_yocto, view};

use main::{Collection, MineType, Monster, Stone};

use crate::utils::{claim_mine, claim_stones, init};
use nft_stone::JsonToken;

#[test]
fn add_collection() {
    let (root, main_contract, _, _, _, _) = init();

    // Add new Collection
    let collection_title = String::from("Collection 2");
    let stone_images = vec!["2-1".to_string(), "2-2".into(), "2-3".into()];
    let collection_add_result = call!(
        root,
        main_contract.add_collection(collection_title.to_string(), "image-2".to_string(), stone_images)
    );
    assert!(collection_add_result.is_ok());

    // Check all collections
    let all_types: HashMap<String, Collection> = view!(
        main_contract.get_collections()
    ).unwrap_json();
    assert_eq!(all_types.len(), 2);

    let stone_images = &all_types.get("1").unwrap().stone_images;
    assert_eq!(stone_images.len(), 3);
}

#[test]
fn user_collection_counts() {
    let (_, main_contract, _, _, _, alice) = init();
    let mine = claim_mine(&main_contract, &alice, MineType::Small);
    claim_stones(&main_contract, &alice, mine.token_id.to_string(), mine.mine_type);

    let collection_counts: HashMap<u32, u32> = view!(
        main_contract.user_collection_counts(alice.account_id)
    ).unwrap_json();

    let collection_total = collection_counts.get(&1).unwrap();
    assert_eq!(&1, collection_total);
}

#[test]
fn mint_collection() {
    let (_, main_contract, _, _, stone_contract, alice) = init();
    let mine = claim_mine(&main_contract, &alice, MineType::Large);
    let stones = claim_stones(&main_contract, &alice, mine.token_id.to_string(), mine.mine_type);

    let mut unique_index: Vec<u8> = vec![];
    let mut unique_stone_ids: Vec<String> = vec![];
    let mut total_health: u8 = 0;
    let mut total_attack: u8 = 0;
    let mut total_brain: u8 = 0;
    let mut total_kill_tokens: u128 = 0;

    for stone in stones.iter() {
        if !unique_index.contains(&stone.collection_index) {
            unique_index.push(stone.collection_index);
            unique_stone_ids.push(stone.token_id.to_string());
            total_health += stone.health;
            total_attack += stone.attack;
            total_brain += stone.brain;
            total_kill_tokens += stone.kill_tokens.parse::<u128>().unwrap();
        }
    }

    if unique_stone_ids.len() == 3 {
        let mint_result = call!(
            alice,
            main_contract.mint_collection(unique_stone_ids, 1),
            deposit=to_yocto("0.01")
        );
        assert!(mint_result.is_ok());

        let monster: Monster = mint_result.unwrap_json();

        assert_eq!(total_health, monster.health);
        assert_eq!(total_attack, monster.attack);
        assert_eq!(total_brain, monster.brain);
        assert_eq!(total_kill_tokens.to_string(), monster.kill_tokens);

        let metadata_result: Vec<JsonToken> = view!(
            stone_contract.nft_tokens_for_owner(alice.account_id(), Some(U128::from(0)), Some(5 as u64))
        ).unwrap_json();
        assert_eq!(metadata_result.len(), 5);
        assert_eq!(metadata_result[0].owner_id, alice.account_id());
        assert_eq!(metadata_result[0].metadata.title.as_ref().unwrap(), &"Monster #1".to_string());

        // Check count monsters
        let total_monster_count: u32 = view!(
            main_contract.minted_monsters_count()
        ).unwrap_json();
        assert_eq!(1, total_monster_count);

        // Check Killed stone count
        let killed_total_count: u32 = view!(
            main_contract.killed_stones_count()
        ).unwrap_json();
        assert_eq!(3, killed_total_count);

        // Check user stones, should be 8-5
        let (total_count, _): (String, Vec<Stone>) = view!(
            main_contract.user_stones(alice.account_id(), 1.into(), 40.into(), None, None)
        ).unwrap_json();
        assert_eq!(total_count, 5.to_string());
    }
}
