use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;

use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{
    AccountId, assert_one_yocto, Balance, BorshStorageKey, env, Gas, near_bindgen, Promise, serde_json::json, setup_alloc,
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet, UnorderedMap};
use near_sdk::json_types::{U128, U64};
use near_sdk::json_types::ValidAccountId;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::Value as JsonValue;

pub use crate::collection::Collection;
pub use crate::mine::{Mine, MineType};
use crate::mine::TotalMineResponse;
pub use crate::stone::{CardRarity, Stone};

mod mine;
mod stone;
mod collection;
mod ft;
mod utils;


setup_alloc!();

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    FtStorageAccounts,
    Mines,
    UserMines,
    UserMineCounter,
    Stones,
    Collections,
    UserStones,
    UserStoneByRarity,
    UserStoneByRarityInner { account_hash: AccountId },
    UserStoneByCollection,
    UserStoneByCollectionInner { account_hash: AccountId },
    Market,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner_id: AccountId,
    contract_ft: AccountId,
    contract_nft_stone: AccountId,
    contract_nft_mine: AccountId,
    ft_storage_accounts: LookupSet<AccountId>,

    mines: LookupMap<TokenId, Mine>,
    user_mines: LookupMap<AccountId, Vec<TokenId>>,
    mines_count: UnorderedMap<MineType, u32>,

    stones: LookupMap<TokenId, Stone>,
    stone_minted_count: u32,
    stone_killed_count: u32,

    user_stones: LookupMap<AccountId, Vec<TokenId>>,
    user_stone_by_rarity: LookupMap<AccountId, LookupMap<CardRarity, Vec<TokenId>>>,
    user_stone_by_collection: LookupMap<AccountId, LookupMap<u32, Vec<TokenId>>>,

    collections: UnorderedMap<u32, Collection>,

    market: UnorderedMap<TokenId, AccountId>,
}

impl Default for Contract {
    fn default() -> Self {
        let mut mine_counter = UnorderedMap::new(StorageKeys::UserMineCounter);
        mine_counter.insert(&MineType::Small, &0);
        mine_counter.insert(&MineType::Medium, &0);
        mine_counter.insert(&MineType::Large, &0);

        Self {
            owner_id: env::predecessor_account_id(),
            contract_ft: format!("ft.{}", env::current_account_id()),
            contract_nft_stone: format!("nft-stone.{}", env::current_account_id()),
            contract_nft_mine: format!("nft-mine.{}", env::current_account_id()),
            ft_storage_accounts: LookupSet::new(StorageKeys::FtStorageAccounts),

            mines: LookupMap::new(StorageKeys::Mines),
            user_mines: LookupMap::new(StorageKeys::UserMines),
            mines_count: mine_counter,

            stones: LookupMap::new(StorageKeys::Stones),
            user_stones: LookupMap::new(StorageKeys::UserStones),
            user_stone_by_rarity: LookupMap::new(StorageKeys::UserStoneByRarity),
            user_stone_by_collection: LookupMap::new(StorageKeys::UserStoneByCollection),
            stone_minted_count: 0,
            stone_killed_count: 0,

            collections: UnorderedMap::new(StorageKeys::Collections),

            market: UnorderedMap::new(StorageKeys::Market),
        }
    }
}

#[near_bindgen]
impl Contract {

    // --------------- Mines ----------------

    // Mint new Mine
    #[payable]
    pub fn mint_mine_nft(&mut self) -> Mine {
        let mint_deposit: Balance = self.to_yocto("0.01");
        let mint_gas: Gas = self.to_tera(25);

        if env::attached_deposit() < mint_deposit {
            panic!("Attach mint deposit!");
        }

        let (mine_metadata, mine) = self.mine_metadata();

        // Add Token Storage
        if !self.ft_storage_accounts.contains(&env::predecessor_account_id()) {
            self.add_token_storage(&env::predecessor_account_id());
        }

        Promise::new(self.contract_nft_mine.clone()).function_call(
            b"nft_mint".to_vec(),
            json!(mine_metadata).to_string().as_bytes().to_vec(),
            mint_deposit,
            mint_gas,
        );

        mine
    }

    // Get user mines
    pub fn user_mines(&self, account_id: ValidAccountId) -> Vec<Mine> {
        let mut user_mine_ids = self.user_mines.get(&account_id.into()).unwrap_or(vec![]);
        user_mine_ids.reverse();
        self.get_mines_by_id(user_mine_ids)
    }

    // Get total mines minted count
    pub fn total_mines_count(&self) -> HashMap<String, TotalMineResponse> {
        let mut result = HashMap::new();
        for (mine_type, counts) in self.mines_count.iter() {
            result.insert(mine_type.to_string(), TotalMineResponse {
                total_count: counts,
                price: Mine::mine_prices(&mine_type, self),
                mine_type: mine_type.to_string(),
                stone_per_day: Mine::mine_stone_per_day(&mine_type),
                media: Mine::mine_media_hash(&mine_type),
            });
        }
        result
    }

    #[payable]
    pub fn transfer_mine(&mut self, token_id: TokenId, recipient_id: AccountId) {
        assert_one_yocto();
        let owner_id = env::predecessor_account_id();
        let mut owner_mines = self.user_mines.get(&owner_id).unwrap_or(vec![]);
        if !owner_mines.contains(&token_id) {
            panic!("You don't have this Mine");
        }
        if recipient_id == owner_id {
            panic!("You can't send to yourself");
        }

        // Add for recipient
        let mut recipient_mines = self.user_mines.get(&recipient_id).unwrap_or(vec![]);
        recipient_mines.push(token_id.to_string());
        self.user_mines.insert(&recipient_id, &recipient_mines);

        // Remove from sender
        let index = owner_mines.iter().position(|id| &token_id == id).unwrap();
        owner_mines.remove(index);
        self.user_mines.insert(&owner_id, &owner_mines);

        // Send Promise to transfer NFT
        let deposit: Balance = 1;
        let gas: Gas = self.to_tera(20);
        Promise::new(self.contract_nft_mine.clone()).function_call(
            b"nft_transfer_extended".to_vec(),
            json!({
                "sender_id": owner_id.to_string(),
                "receiver_id": recipient_id.to_string(),
                "token_id": token_id,
            }).to_string().as_bytes().to_vec(),
            deposit,
            gas,
        );

        // Add token storage
        if !self.ft_storage_accounts.contains(&recipient_id.to_string()) {
            self.add_token_storage(&recipient_id.to_string());
        }
    }

    #[payable]
    pub fn transfer_stone(&mut self, _token_id: TokenId, _recipient_id: AccountId) {
        assert_one_yocto();
        // TODO: Transfer
    }


    // -------------- Stones ---------------

    // Get claim time for each mine
    pub fn stone_claim_time(&self, account_id: ValidAccountId, timestamp: U64) -> HashMap<String, u64> {
        self.stone_free_claim_time(account_id.into(), timestamp.into())
    }

    // Mint new stone for mine
    #[payable]
    pub fn mint_free_stone_nft(&mut self, mine_id: TokenId) -> Vec<Stone> {
        let mint_count = self.mine_claim_free_stone_count(env::predecessor_account_id(), mine_id.clone());
        if mint_count == 0 {
            panic!("You can't claim stones from this Mine");
        }

        let mint_gas: Gas = 20 + (mint_count * 8) as u64;
        let min_deposit: Balance;

        match mint_count {
            1 => {
                min_deposit = self.to_yocto("0.01");
            }
            4 => {
                min_deposit = self.to_yocto("0.03");
            }
            8 => {
                min_deposit = self.to_yocto("0.06");
            }
            _ => panic!("Wrong mint_count")
        }

        if env::attached_deposit() < min_deposit {
            panic!("Attach mint deposit!");
        }

        let (stones_metadata, result) = self.stones_metadata(mint_count, mine_id.clone());

        Promise::new(self.contract_nft_stone.to_string()).function_call(
            b"mint_multiple_stones".to_vec(),
            json!({
                "stones_metadata": stones_metadata
            }).to_string().as_bytes().to_vec(),
            min_deposit,
            self.to_tera(mint_gas),
        );

        result
    }

    // Get total minted stones count
    pub fn minted_stones_count(self) -> u32 {
        self.stone_minted_count
    }

    // Get total killed stones
    pub fn killed_stones_count(self) -> u32 {
        self.stone_killed_count
    }

    // Kill stone
    #[payable]
    pub fn stone_kill(&mut self, stone_id: TokenId) -> String {
        assert_one_yocto();
        let stone = self.stones.get(&stone_id).unwrap();
        self.stone_remove_from_user(&stone, &env::predecessor_account_id());
        self.stone_remove_token_transfer(stone)
    }

    // User stones with pagination
    pub fn user_stones(&self, account_id: AccountId, page_num: U64, page_limit: U64, filter_rarity: Option<CardRarity>, filter_collection: Option<u32>) -> (U64, Vec<Stone>) {
        let page_num = page_num.into();
        let page_limit = page_limit.into();
        if page_num < 1 || page_limit < 1 {
            panic!("Please provide correct page_num and page_limit");
        }
        self.user_stones_with_pagination(account_id, page_num, page_limit, filter_rarity, filter_collection)
    }


    // ----------- Collections ------------

    // Add new collection
    pub fn add_collection(&mut self, title: String, image: String, stone_images: Vec<String>) {
        self.assert_contract_owner(self.owner_id.to_string());
        if title.len() < 1 || image.len() < 1 || stone_images.len() < 1 {
            panic!("All fields is required");
        }

        self.add_new_collection(title, image, stone_images)
    }

    // Get all Collections
    pub fn get_collections(&self) -> HashMap<u32, Collection> {
        self.collections.iter().collect()
    }

    // Get one Collection
    pub fn get_one_collection(&self, collection_id: u32) -> Collection {
        self.collections.get(&collection_id).unwrap()
    }

    // Get user stones count for each collection
    pub fn user_collection_counts(&self, account_id: AccountId) -> HashMap<u32, u32> {
        self.get_user_collection_counts(&account_id)
    }

    // ------------- Market --------------

    // Sell Mine
    #[payable]
    pub fn publish_mines_on_market(&mut self, token_price_list: HashMap<TokenId, U128>) {
        assert_one_yocto();
        let owner_id = env::predecessor_account_id();
        let user_mines = self.user_mines.get(&owner_id).unwrap_or(vec![]);

        for (token_id, sale_price) in token_price_list.into_iter() {
            if !user_mines.contains(&token_id) {
                panic!("You don't own this mine");
            }

            let mut mine = self.mines.get(&token_id).unwrap();
            self.market.insert(&token_id, &owner_id);
            self.mines.remove(&token_id);
            mine.sale_price = Some(sale_price.into());
            self.mines.insert(&token_id, &mine);
        }
    }

    // Cancel Mine sell
    pub fn remove_mines_from_market(&mut self, token_list: Vec<TokenId>) {
        let mut tokens_list: Vec<TokenId> = vec![];
        let owner_id = env::predecessor_account_id();
        let user_mines = self.user_mines.get(&owner_id).unwrap();

        for token_id in token_list {
            if !user_mines.contains(&token_id) {
                panic!("You don't own this mine");
            }
            self.market.remove(&token_id);
            tokens_list.push(token_id);
        }
    }

    // Sell Stone
    #[payable]
    pub fn publish_stones_on_market(&mut self, token_price_list: HashMap<TokenId, U128>) {
        assert_one_yocto();
        let owner_id = env::predecessor_account_id();
        let user_stones = self.user_stones.get(&owner_id).unwrap_or(vec![]);

        for (token_id, sale_price) in token_price_list.into_iter() {
            if !user_stones.contains(&token_id) {
                panic!("You don't own this stone");
            }

            let mut stone = self.stones.get(&token_id).unwrap();
            self.market.insert(&token_id, &owner_id);
            self.stones.remove(&token_id);
            stone.sale_price = Some(sale_price.into());
            self.stones.insert(&token_id, &stone);
        }
    }

    // Cancel stone sell
    pub fn remove_stones_from_market(&mut self, token_list: Vec<TokenId>) {
        let mut tokens_list: Vec<TokenId> = vec![];
        let owner_id = env::predecessor_account_id();
        let user_stones = self.user_stones.get(&owner_id).unwrap();

        for token_id in token_list {
            if !user_stones.contains(&token_id) {
                panic!("You don't own this stone");
            }
            self.market.remove(&token_id);
            tokens_list.push(token_id);
        }
    }

    // Get stones on the market
    pub fn get_stones_from_market(&self, start: u64, limit: u64) -> Vec<Stone> {
        let token_list = self.get_tokens_from_market(start, limit);
        self.get_stones_by_id(token_list)
    }

    // Get mines on the market
    pub fn get_mines_from_market(&self, start: u64, limit: u64) -> Vec<Mine> {
        let token_list = self.get_tokens_from_market(start, limit);
        self.get_mines_by_id(token_list)
    }

    //
    pub fn get_tokens_from_market(&self, start: u64, limit: u64) -> Vec<TokenId> {
        self.market.iter()
            .skip(start as usize)
            .take(limit as usize)
            .map(|item| item.0)
            .collect()
    }

    // pub fn test_random(&self) {
    //     for i in 1..32 {
    //         let rand_val = self.random_u8_range(i, i);
    //         log!("{:?} {}", i, rand_val);
    //     }
    // }
}
