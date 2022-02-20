use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue, log
};

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::approval::*;
pub use crate::royalty::*;
pub use crate::events::*;

mod internal;
mod approval;
mod enumeration;
mod metadata;
mod mint;
mod nft_core;
mod royalty;
mod events;

/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "nft-1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";

const TOKEN_IMAGE_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 512 512'%3E%3Cpath d='m92.8 325.1 2.1-19.1s-4.2-12.5 5-13.7 9.1-.8 26.1-3.7c17-2.9 12-3.3 28.2-7.1 16.2-3.7 3.7-6.6 19.1-5.4 15.4 1.2 64.8 3.3 70.2 4.2s7.5-2.1 7.5 1.2 5 8.7 23.3 10 12.9 3.3 21.2 1.2c8.3-2.1 26.6-4.2 33.6-1.2 7.1 2.9 17.4 12 31.1 12 13.7 0 24.3 5.8 28.8 14.9s2.3 4.2 8.5 8.3c6.2 4.2 17.8 1.7 23.2 10.8 5.4 9.1 6.6 11.6 0 19.5s-36.9 10.8-46.1 11.2-15.4-2.9-28.2 2.5c-12.9 5.4 19.5 5.8-19.1 5.4-38.6-.4-27.4 2.1-46.1-3.3-18.7-5.4-50.6-10.4-50.6-10.4s-81.4-19.1-85.5-20.3c-4.2-1.2-18.3-8.7-23.7-7.9-5.4.8-2.9 5.8-13.7.8-10.7-4.9-14.9-9.9-14.9-9.9z' fill='%23666'/%3E%3Cpath d='M114.9 350.4s-.6.6 5.4 2.6 5.5 1.4 8.9 2c3.4.6 4.2.3 8.7 3.8s2.6 1.4 8.4 2.5c5.7 1 19.2 15.1 47.8 17.2 28.6 2.1 23.7 1.3 34.7 2.5 11 1.2 8.2-2.9 27.4 5s16.8 5.8 27 8.7 40.4-3.8 47-3.8c6.5 0 3.3-6.2 15.9-1.7 12.7 4.6 18.8 6.2 22.9 6.2 4.1 0 7-5.4 15.5-7.9 8.6-2.5 18.8-10 24.9-11.2 6.1-1.2 9.8-7.9 9.8-7.9l7-13.4s2.5-1.1 2.3-2.5c-.2-1.4-5.4-6.7-5.4-6.7l-3-8.6-.6 1.9c3.2 6.1 3.2 8.9-2.3 15.5-.8 1-2.1 2-3.6 2.8-10.3 5.8-33.9 8-41.7 8.4-9 .4-15.1-2.9-27.8 2.5-8.6 3.7-2.8 5.1-6.3 5.5-1.6.2-.4 0-12.5-.1-38-.4-27 2.1-45.4-3.3S230 360 230 360s-80.1-19.1-84.2-20.3c-4.1-1.2-18-8.7-23.3-7.9-5.3.8-2.9 5.8-13.5.8s-14.7-10-14.7-10l2-19.1s-1-3.1-.9-6.4c-1.8 1.4-4.1 3.1-5.1 7.6-1 4.6-6.7 16.8-6.7 19.9 0 3.1 4.7 8.5 8 13.9s14.3 12.7 14.3 12.7l9-.8z' fill='%235c5c5c'/%3E%3Cpath d='M284 247.5s16.6-24.4 21.3-45.6c4.8-21.2 4.8-25.5 4.8-25.5l17.5-25.2c1.2-1.8 2.6-8.9-1.1-11.9-5-6.5-10.8-7.8-15.8-4.8-3.7 2.3-7.7 9.5-7.3 13l-.4 6.1c-9.6 10-8.8 10.4-12.7 14.2-1.5 1.4-5.9-2.5-5.8-2.4.5.6 4.2-14.5 4.2-22.4 0-7.9 13.2-23 16.9-31.9 3.7-9-8.8-19.6-18-8.2-9.2 11.4-13.9 25-13.9 25l-8.7 31.6-7-.4s-1.3-26.4-.4-31c.5-2.3 3.9-11.6 7-20.8 3-8.9 5.7-17.8 5.2-20.5-1.1-5.6-13.2-10.4-19.5-.8-6.4 9.5-12.7 46.4-13.2 48-.5 1.6-1.6 11.7-1.6 16.2s-.1 10.1-.1 10.1l-5.3-2.5c-3.6-1.7-3.2-8.5-3.9-15.6-.8-8.8-2-18.4.3-22.8 4.1-7.6 1.5-10.4-4.4-16.2-2.1-.1-12.7-3.3-16.5 3-4.8 6.9-3.2 12.1-2.9 17.9.6 10.5 3.7 20.5 3.7 28.9 0 1.2 1.7 6.2 3.3 11.2-3.6 2-8.1 4.3-11.3 5.5-6.4 2.4-7.2 5-7.2 7.9 0 2.9 3.7 16.7 4.2 25.7.4 7.2 10.4 15 17.2 19 6.1 15.2 12.4 10.3 14 21.3 2.5 16.4-2 48.6-6.2 79.9 9.2-2 17.6-2.2 24.1-.6 7.3 1.8 5.4 4.7 16.2-2.9s1-10.5 19.1-2.6l2.3 1 1.9-70.9z' fill='%23edd6c0'/%3E%3Cpath fill='%23999' d='M253 82.9c1.8-3.9 7.1-14.9 13.2-18 6.1-3 7.1 2.4 7.1 2.4-.4 3.3-3 13.1-4.3 17.4-2.6-3.9-10.3-6.4-16-1.8zM211.1 106.4s9.4 15.5 16.7 15c4-.3.1-9.2-1-10.9l-2.9-4.2c-4.7-1.3-9-1.5-12.8.1zM305.7 146.8l6.2-9.7c2.6-1.2 9.1.5 9.7 4.1.3 1.7-5 6.5-9.4 11.9-4.8 5.9-14.6 7.6-6.5-6.3zM292.9 98.6c3.4-4.2 8.5-9.5 13.1-10.8 6.5-1.8 6.6 3.7 6.6 3.7-.9 2.7-4.1 9.5-6.4 14-1.2-5.3-7.2-9.5-13.3-6.9z'/%3E%3Cpath d='M221.9 164s4.4 9.5 1.6 14.6c-2.8 5-9.5 6.8-12 8.5l1.5 3.2s27.9-4.5 36.9 13.4 9 18.1 9 18.1 7.8-18.5 13.4-27.3c5.7-8.8 8.4-11.3 8.4-11.3s-9 1.6-15.6-1c-6.6-2.5-7-3.7-13.4-3.3-5.1.3-16.8 7.3-25 1.2 2.1-10-2.5-14.7-4.8-16.1z' fill='%23ddc8b5'/%3E%3Cpath fill='%23999' d='M205.8 167.3s22.3-10.3 24.5-10.6 3.2 1.3.7 4.2-22.7 13.3-22.7 13.3-1.7-5.8-2.5-6.9z'/%3E%3C/svg%3E%0A";


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StoneTokenData {
    token_id: TokenId,
    receiver_id: AccountId,
    token_metadata: TokenMetadata,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MonsterTokenData {
    token_id: TokenId,
    receiver_id: AccountId,
    token_metadata: TokenMetadata,
    input_stones: Vec<TokenId>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
}

#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "ZomMine Mine".to_string(),
                symbol: "LAND".to_string(),
                icon: Some(TOKEN_IMAGE_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id.
    */
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        //create a variable of type Self with all the fields initialized.
        let this = Self {
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            //set the owner_id field equal to the passed in owner_id.
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
        };

        //return the Contract object
        this
    }

}
