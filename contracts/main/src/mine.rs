use std::str::FromStr;
use std::string::ParseError;

use near_sdk::serde_json::json;
use near_sdk::serde_json::Value as JsonValue;
use near_sdk::Timestamp;

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum MineType {
    Small,
    Medium,
    Large,
}

impl fmt::Display for MineType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MineType::Small => write!(f, "Small"),
            MineType::Medium => write!(f, "Medium"),
            MineType::Large => write!(f, "Large"),
        }
    }
}

impl FromStr for MineType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Small" => MineType::Small,
            "Medium" => MineType::Medium,
            "Large" => MineType::Large,
            _ => panic!("Wrong metadata")
        })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TotalMineResponse {
    pub total_count: u32,
    pub price: u128,
    pub stone_per_day: u8,
    pub media: String,
    pub mine_type: String,
}


#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Mine {
    pub token_id: TokenId,
    pub mine_type: MineType,
    pub last_stone_claim: Timestamp,
    pub sale_price: Option<u128>,
    pub media: String,
}

impl Mine {
    pub fn mine_media_hash(mine_type: &MineType) -> String {
        match mine_type {
            MineType::Small => String::from("bafkreiay22z3j7gbgl4qucxc5l4f5grgw2lukesaphclj42cbsaznmgpqa"),
            MineType::Medium => String::from("bafkreiarhc2ivvhhcsfqiym6g3er7lnktdiw6rlmahco7l3awtvcvxve4m"),
            MineType::Large => String::from("bafkreiarhc2ivvhhcsfqiym6g3er7lnktdiw6rlmahco7l3awtvcvxve4m"),
        }
    }

    pub fn mine_limits(mine_type: &MineType) -> u32 {
        match mine_type {
            MineType::Small => 59999,
            MineType::Medium => 5999,
            MineType::Large => 1999,
        }
    }

    pub fn mine_stone_per_day(mine_type: &MineType) -> u8 {
        match mine_type {
            MineType::Small => 1,
            MineType::Medium => 4,
            MineType::Large => 8,
        }
    }

    pub fn mine_prices(mine_type: &MineType, contract: &Contract) -> u128 {
        let small_deposit: u128 = contract.to_yocto("0.01");

        match mine_type {
            MineType::Small => small_deposit, // 0.01 NEAR
            MineType::Medium => small_deposit * 100 * 5, // 5 NEAR
            MineType::Large => small_deposit * 100 * 9, // 9 NEAR
        }
    }
}

impl Contract {
    pub(crate) fn get_mines_by_id(&self, id_list: Vec<TokenId>) -> Vec<Mine> {
        id_list.into_iter()
            .flat_map(|token_id| self.mines.get(&token_id))
            .collect()
    }

    pub(crate) fn mine_metadata(&mut self) -> (JsonValue, Mine) {
        let owner_id = env::predecessor_account_id().to_string();
        let mine_type: MineType;

        if env::attached_deposit() == Mine::mine_prices(&MineType::Small, self) {
            // Check if user can mint free mine
            self.check_mint_free_mine(owner_id.to_string());
            mine_type = MineType::Small;
        } else if env::attached_deposit() == Mine::mine_prices(&MineType::Medium, self) {
            mine_type = MineType::Medium;
        } else if env::attached_deposit() == Mine::mine_prices(&MineType::Large, self) {
            mine_type = MineType::Large;
        } else {
            panic!("Wrong deposit amount");
        }

        let media_url: String = Mine::mine_media_hash(&mine_type);
        let mines_limit: u32 = Mine::mine_limits(&mine_type);
        let mines_count: u32 = self.mines_count.get(&mine_type).unwrap() + 1;
        let first_char = mine_type.to_string().chars().nth(0).unwrap().to_lowercase();
        let token_id = format!("{}-{}-{}", first_char, mines_count, self.random_u8(0));
        let title = format!("{} Mine #{}", mine_type.to_string(), mines_count);

        // limit token count
        if mines_count > mines_limit {
            panic!("You can't mint this mine type, the limit is reached.");
        }

        let metadata = json!({
            "token_id": token_id,
            "receiver_id": owner_id.to_string(),
            "token_metadata": {
                "title": title,
                "media": media_url,
                "copies": 1
            }
        });

        // Update mines counter
        let mines_count: u32 = self.mines_count.get(&mine_type).unwrap() + 1;
        self.mines_count.insert(&mine_type, &mines_count);

        // Add new mine
        let mut user_mines = self.user_mines.get(&owner_id).unwrap_or(vec![]);
        let mine = Mine {
            token_id: token_id.to_string(),
            mine_type,
            media: media_url,
            last_stone_claim: 0,
            sale_price: None,
        };

        self.mines.insert(&token_id, &mine);
        user_mines.push(token_id.to_string());
        self.user_mines.insert(&owner_id, &user_mines);

        (metadata, mine)
    }

    pub(crate) fn check_mint_free_mine(&self, account_id: String) {
        let user_mines = self.user_mines.get(&account_id).unwrap_or(vec![]);

        for token_id in user_mines {
            let mine = self.mines.get(&token_id).unwrap();
            if mine.mine_type == MineType::Small {
                panic!("You can't mint more Small Mines");
            }
        }
    }

    pub(crate) fn stone_free_claim_time(&self, account_id: AccountId, timestamp: u64) -> HashMap<String, u64> {
        let mut result: HashMap<String, u64> = HashMap::new();
        let allow_claim_timestamp = timestamp - 1_000_000_000 * 60 * 60 * 24;
        let user_mines = self.user_mines.get(&account_id).unwrap().to_vec();

        for user_mine in user_mines {
            let mine = self.mines.get(&user_mine).unwrap();
            if mine.last_stone_claim <= allow_claim_timestamp {
                result.insert(mine.token_id, 0 as u64);
            } else {
                let claim_diff = mine.last_stone_claim - allow_claim_timestamp;
                result.insert(mine.token_id, claim_diff);
            }
        }

        result
    }

    pub(crate) fn mine_claim_free_stone_count(&self, account_id: AccountId, mine_id: TokenId) -> u8 {
        let allow_claim_timestamp = env::block_timestamp() - 1_000_000_000 * 60 * 60 * 24;
        let user_mines = self.user_mines.get(&account_id).unwrap().to_vec();

        for user_mine in user_mines {
            let mine = self.mines.get(&user_mine).unwrap();
            if mine.token_id == mine_id && mine.last_stone_claim <= allow_claim_timestamp {
                return match mine.mine_type {
                    MineType::Small => 1,
                    MineType::Medium => 4,
                    MineType::Large => 8,
                };
            }
        }
        return 0;
    }

    pub(crate) fn update_mine_claim_timestamp(&mut self, mut current_mine: Mine) {
        // remove and add updated mine
        self.mines.remove(&current_mine.token_id);
        current_mine.last_stone_claim = env::block_timestamp();
        self.mines.insert(&current_mine.token_id, &current_mine);
    }
}
