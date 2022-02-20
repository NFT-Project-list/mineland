use std::str::FromStr;
use std::string::ParseError;

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum CardRarity {
    Common,
    UnCommon,
    Rare,
    Legendary,
}

impl fmt::Display for CardRarity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CardRarity::Common => write!(f, "Common"),
            CardRarity::UnCommon => write!(f, "UnCommon"),
            CardRarity::Rare => write!(f, "Rare"),
            CardRarity::Legendary => write!(f, "Legendary"),
        }
    }
}

impl FromStr for CardRarity {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Common" => CardRarity::Common,
            "UnCommon" => CardRarity::UnCommon,
            "Rare" => CardRarity::Rare,
            "Legendary" => CardRarity::Legendary,
            _ => panic!("Wrong metadata")
        })
    }
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Stone {
    pub token_id: TokenId,
    pub card_rarity: CardRarity,
    pub sale_price: Option<u128>,
    pub kill_tokens: String,
    pub media: String,
    pub collection_id: u32,
    pub collection_index: u8,
    pub mint_date: u64,
    pub health: u8,
    pub attack: u8,
    pub brain: u8,
}

impl Stone {
    pub fn get_kill_tokens(&self, timestamp: u64) -> u128 {
        let token_count = 100_000_000_000_000_000_000_000; // 0.1 MNL
        let duration = timestamp - self.mint_date;
        let coefficient = fast_math::log2_raw(duration as f32) as u128;

        self.kill_tokens.parse::<u128>().unwrap() + coefficient * token_count
    }
}

impl Contract {
    pub(crate) fn get_stones_by_id(&self, id_list: Vec<TokenId>) -> Vec<Stone> {
        id_list.into_iter()
            .flat_map(|token_id| self.stones.get(&token_id))
            .collect()
    }

    pub(crate) fn stones_metadata(&mut self, mint_count: u8, mine_id: TokenId) -> (Vec<JsonValue>, Vec<Stone>) {
        let mut metadata: Vec<JsonValue> = vec![];
        let mut new_stones: Vec<Stone> = vec![];
        let owner_id = env::predecessor_account_id().to_string();
        let mut user_stones: Vec<TokenId> = self.user_stones.get(&owner_id).unwrap_or(vec![]);

        let current_mine: Mine = self.mines.get(&mine_id).unwrap();
        let user_mines = self.user_mines.get(&owner_id).unwrap();
        if !user_mines.contains(&mine_id) {
            panic!("You don't have this Mine");
        }

        for num in 1..=mint_count {
            let num = num as u32;
            let collection_id: u32 = self.generate_stone_random_collection(num as usize);
            let (media_url, collection_index) = self.generate_stone_media(collection_id, num as usize);
            let token_id = format!("zm-{}-{}{}", self.stone_minted_count + 1, self.random_u8(0), num);
            let title = format!("Stone #{}", self.stone_minted_count + 1);

            metadata.push(json!({
                "token_id": token_id,
                "receiver_id": owner_id,
                "token_metadata": {
                    "title": title,
                    "media": media_url,
                    "copies": 1
                }
            }));

            // Add for user
            let card_rarity = self.generate_stone_card_rarity(&current_mine.mine_type, num as usize);
            let health = self.generate_stone_health(num as usize);
            let attack = self.generate_stone_attack(num as usize);
            let brain = self.generate_stone_brain(num as usize);
            let kill_tokens = self.generate_stone_kill_tokens(&card_rarity, &health, &attack, &brain);

            // Add stone to user Rarity & Collection
            self.stone_add_user_rarity(&owner_id, &token_id, &card_rarity);
            self.stone_add_user_collection(&owner_id, &token_id, &collection_id);

            let stone = Stone {
                token_id: token_id.to_string(),
                mint_date: env::block_timestamp(),
                sale_price: None,
                kill_tokens: kill_tokens.to_string(),
                media: media_url,
                collection_id,
                collection_index,
                card_rarity,
                health,
                attack,
                brain,
            };
            self.stones.insert(&token_id, &stone);
            user_stones.push(token_id.to_string());
            new_stones.push(stone);

            // increase total count
            self.stone_minted_count += 1;
        }

        self.user_stones.insert(&owner_id, &user_stones);

        // update mine claim timestamp
        self.update_mine_claim_timestamp(current_mine);

        (metadata, new_stones)
    }

    pub(crate) fn generate_stone_card_rarity(&self, mine_type: &MineType, num: usize) -> CardRarity {
        let rand_val = self.random_u8_range(num, 100) as u32 * 10;
        let result: CardRarity;

        if mine_type == &MineType::Small {
            result = match rand_val {
                0..=9 => CardRarity::Legendary,
                10..=59 => CardRarity::Rare,
                60..=299 => CardRarity::UnCommon,
                _ => CardRarity::Common
            };
        } else if mine_type == &MineType::Medium {
            result = match rand_val {
                0..=11 => CardRarity::Legendary,
                12..=71 => CardRarity::Rare,
                72..=359 => CardRarity::UnCommon,
                _ => CardRarity::Common
            };
        } else {
            result = match rand_val {
                0..=14 => CardRarity::Legendary,
                15..=89 => CardRarity::Rare,
                90..=449 => CardRarity::UnCommon,
                _ => CardRarity::Common
            };
        }

        result
    }

    // Get random stone type
    pub(crate) fn generate_stone_random_collection(&self, num: usize) -> u32 {
        let rand_index = self.random_u8_range(0 + num, self.collections.len() as usize);
        let collection_ids: Vec<u32> = self.collections.keys().collect();
        *collection_ids.get(rand_index as usize).unwrap()
    }

    pub(crate) fn generate_stone_media(&self, collection: u32, num: usize) -> (String, u8) {
        let collection = self.collections.get(&collection).unwrap();
        let mut stone_index = self.random_u8_range(2 + num, collection.stone_images.len());
        if stone_index as usize >= collection.stone_images.len() {
            stone_index = 0;
        }
        let media_url = collection.stone_images[stone_index as usize].to_string();
        (media_url, stone_index)
    }

    // 1 - 6
    pub(crate) fn generate_stone_health(&self, num: usize) -> u8 {
        let rand_val = self.random_u8_range(5 + num, 5);
        rand_val + 1
    }

    // 1 - 3
    pub(crate) fn generate_stone_attack(&self, num: usize) -> u8 {
        let rand_val = self.random_u8_range(10 + num, 2);
        rand_val + 1
    }

    // 0 - 2
    pub(crate) fn generate_stone_brain(&self, num: usize) -> u8 {
        let rand_val = self.random_u8_range(15 + num, 2);
        rand_val
    }

    pub(crate) fn generate_stone_kill_tokens(&self, card_rarity: &CardRarity, health: &u8, attack: &u8, brain: &u8) -> u128 {
        let one_token: u128 = self.to_yocto("1");
        let rarity: u128 = match card_rarity {
            CardRarity::Legendary => 70,
            CardRarity::Rare => 30,
            CardRarity::UnCommon => 8,
            CardRarity::Common => 2
        };
        return (health + attack + brain) as u128 * rarity * one_token;
    }

    pub(crate) fn stone_remove_from_user(&mut self, stone: &Stone, owner_id: &AccountId) {
        let mut user_stones = self.user_stones.get(owner_id).unwrap();
        if !user_stones.contains(&stone.token_id) {
            panic!("User don't own this stone");
        }

        let index = user_stones.iter().position(|stone_id| &stone.token_id == stone_id).unwrap();
        user_stones.remove(index);
        self.user_stones.insert(owner_id, &user_stones);

        // Remove from user rarities & collections
        self.stone_remove_user_rarities(&owner_id, stone);
        self.stone_remove_user_collections(&owner_id, stone);

        self.stones.remove(&stone.token_id);
        self.stone_killed_count += 1;
    }

    pub fn stone_add_user_rarity(&mut self, owner_id: &AccountId, token_id: &TokenId, card_rarity: &CardRarity) {
        let mut user_stone_rarity = self.user_stone_by_rarity.get(owner_id).unwrap_or(LookupMap::new(
            StorageKeys::UserStoneByRarityInner { account_hash: owner_id.to_string() })
        );
        let mut rarity = user_stone_rarity.get(card_rarity).unwrap_or(vec![]);
        rarity.push(token_id.to_string());
        user_stone_rarity.insert(card_rarity, &rarity);
        self.user_stone_by_rarity.insert(owner_id, &user_stone_rarity);
    }

    pub fn stone_add_user_collection(&mut self, owner_id: &AccountId, token_id: &TokenId, collection_id: &u32) {
        let mut user_stone_collection = self.user_stone_by_collection.get(owner_id).unwrap_or(LookupMap::new(
            StorageKeys::UserStoneByCollectionInner { account_hash: owner_id.to_string() })
        );
        let mut collection = user_stone_collection.get(collection_id).unwrap_or(vec![]);
        collection.push(token_id.to_string());
        user_stone_collection.insert(collection_id, &collection);
        self.user_stone_by_collection.insert(owner_id, &user_stone_collection);
    }

    pub fn stone_remove_user_collections(&mut self, owner_id: &AccountId, stone: &Stone) {
        let mut all_collections = self.user_stone_by_collection.get(owner_id).unwrap();
        let mut user_stone_collection = all_collections.get(&stone.collection_id).unwrap();
        let index = user_stone_collection.iter().position(|stone_id| &stone.token_id == stone_id).unwrap();
        user_stone_collection.remove(index);
        all_collections.insert(&stone.collection_id, &user_stone_collection);
        self.user_stone_by_collection.insert(owner_id, &all_collections);
    }

    pub fn stone_remove_user_rarities(&mut self, owner_id: &AccountId, stone: &Stone) {
        let mut all_rarities = self.user_stone_by_rarity.get(&owner_id).unwrap();
        let mut user_stone_rarity = all_rarities.get(&stone.card_rarity).unwrap();
        let index = user_stone_rarity.iter().position(|stone_id| &stone.token_id == stone_id).unwrap();
        user_stone_rarity.remove(index);
        all_rarities.insert(&stone.card_rarity, &user_stone_rarity);
        self.user_stone_by_rarity.insert(&owner_id, &all_rarities);
    }

    pub(crate) fn stone_remove_token_transfer(&self, stone: Stone) -> String {
        let kill_tokens = stone.get_kill_tokens(env::block_timestamp());

        // transfer MNL tokens
        let ft_mint_gas: Gas = self.to_tera(10);
        Promise::new(self.contract_ft.to_string()).function_call(
            b"ft_transfer".to_vec(),
            json!({
                "receiver_id": &env::predecessor_account_id(),
                "amount": kill_tokens.to_string()
            }).to_string().as_bytes().to_vec(),
            1,
            ft_mint_gas,
        );

        // remove NFT and free storage
        let call_gas: Gas = self.to_tera(30);
        Promise::new(self.contract_nft_stone.clone()).function_call(
            b"nft_destroy".to_vec(),
            json!({
                "token_id": stone.token_id.to_string(),
                "token_owner_id": &env::predecessor_account_id(),
            }).to_string().as_bytes().to_vec(),
            env::attached_deposit(),
            call_gas,
        );

        kill_tokens.to_string()
    }

    pub(crate) fn user_stones_with_pagination(
        &self,
        account_id: AccountId,
        page_num: u64,
        page_limit: u64,
        filter_rarity: Option<CardRarity>,
        filter_collection: Option<u32>,
    ) -> (U64, Vec<Stone>) {
        let mut user_stone_ids: Vec<TokenId> = vec![];

        if filter_rarity.is_some() && filter_collection.is_some() {
            // Filter by Card Rarity and Collection
            if self.user_stone_by_rarity.contains_key(&account_id) && self.user_stone_by_collection.contains_key(&account_id) {
                let filter_rarity_filter = filter_rarity.as_ref().unwrap();
                let filter_collection_filter = filter_collection.as_ref().unwrap();
                let all_rarities = self.user_stone_by_rarity.get(&account_id).unwrap();
                let all_collections = self.user_stone_by_collection.get(&account_id).unwrap();
                if all_rarities.contains_key(filter_rarity_filter) && all_collections.contains_key(filter_collection_filter) {
                    let user_stone_rarities = all_rarities.get(filter_rarity_filter).unwrap();
                    let user_stone_collections = all_collections.get(filter_collection_filter).unwrap();

                    for rarity in user_stone_rarities.into_iter() {
                        if user_stone_collections.contains(&rarity) {
                            user_stone_ids.push(rarity);
                        }
                    }
                }
            }
        } else if filter_rarity.is_some() {
            // Filter by Card Rarity
            if self.user_stone_by_rarity.contains_key(&account_id) {
                let filter_rarity_filter = filter_rarity.as_ref().unwrap();
                let all_rarities = self.user_stone_by_rarity.get(&account_id).unwrap();
                if all_rarities.contains_key(filter_rarity_filter) {
                    user_stone_ids = all_rarities.get(filter_rarity_filter).unwrap();
                }
            }
        } else if filter_collection.is_some() {
            // Filter by Collection
            if self.user_stone_by_collection.contains_key(&account_id) {
                let filter_collection_filter = filter_collection.as_ref().unwrap();
                let all_collections = self.user_stone_by_collection.get(&account_id).unwrap();
                if all_collections.contains_key(filter_collection_filter) {
                    user_stone_ids = all_collections.get(filter_collection_filter).unwrap();
                }
            }
        } else {
            // No filters
            user_stone_ids = self.user_stones.get(&account_id).unwrap_or(vec![]);
        }

        user_stone_ids.reverse();

        // Pagination
        let count_total = user_stone_ids.len() as u64;
        let start_index = (page_num - 1) * page_limit;
        let user_stone_ids = user_stone_ids.into_iter()
            .skip(start_index as usize)
            .take(page_limit as usize)
            .collect();

        let user_stones: Vec<Stone> = self.get_stones_by_id(user_stone_ids);

        (count_total.into(), user_stones)
    }
}
