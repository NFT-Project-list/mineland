use crate::*;

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Collection {
    pub title: String,
    pub image: String,
    pub stone_images: Vec<String>,
}

impl Contract {
    pub(crate) fn add_new_collection(&mut self, title: String, image: String, stone_images: Vec<String>) {
        for (_, collection) in self.collections.iter() {
            if collection.title == title {
                panic!("Collection already exists");
            }
        }

        let collection_count = self.collections.len() as u32;
        self.collections.insert(&(collection_count + 1), &Collection {
            title,
            image,
            stone_images,
        });
    }

    pub(crate) fn get_user_collection_counts(&self, account_id: &AccountId) -> HashMap<u32, u32> {
        let mut result = HashMap::new();
        let user_collections = self.user_stone_by_collection.get(account_id).unwrap_or(
            LookupMap::new(b"lm".to_vec())
        );

        for (id, _) in self.collections.iter() {
            let collection_stones = user_collections.get(&id).unwrap_or(vec![]);
            result.insert(id, collection_stones.len() as u32);
        }

        result
    }
}
