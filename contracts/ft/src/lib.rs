/*!
Fungible Token implementation with JSON serialization.
NOTES:
  - The maximum balance value is limited by U128 (2**128 - 1).
  - JSON calls should pass U128 as a base-10 string. E.g. "100".
  - The contract optimizes the inner trie structure by hashing account IDs. It will prevent some
    abuse of deep tries. Shouldn't be an issue, once NEAR clients implement full hashing of keys.
  - The contract tracks the change in storage before and after the call. If the storage increases,
    the contract requires the caller of the contract to attach enough deposit to the function call
    to cover the storage cost.
    This is done to prevent a denial of service attack on the contract by taking all available storage.
    If the storage decreases, the contract will issue a refund for the cost of the released storage.
    The unused tokens from the attached deposit are also refunded, so it's safe to
    attach more deposit than required.
  - To prevent the deployed contract from being modified or deleted, it should not have any access
    keys on its account.
 */
use near_contract_standards::fungible_token::FungibleToken;
use near_contract_standards::fungible_token::metadata::{
    FT_METADATA_SPEC, FungibleTokenMetadata, FungibleTokenMetadataProvider,
};
use near_sdk::{AccountId, Balance, env, log, near_bindgen, PanicOnDefault, Promise, PromiseOrValue};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::{U128, ValidAccountId};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

const FT_IMAGE_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 512 512'%3E%3Cpath d='m92.8 325.1 2.1-19.1s-4.2-12.5 5-13.7 9.1-.8 26.1-3.7c17-2.9 12-3.3 28.2-7.1 16.2-3.7 3.7-6.6 19.1-5.4 15.4 1.2 64.8 3.3 70.2 4.2s7.5-2.1 7.5 1.2 5 8.7 23.3 10 12.9 3.3 21.2 1.2c8.3-2.1 26.6-4.2 33.6-1.2 7.1 2.9 17.4 12 31.1 12 13.7 0 24.3 5.8 28.8 14.9s2.3 4.2 8.5 8.3c6.2 4.2 17.8 1.7 23.2 10.8 5.4 9.1 6.6 11.6 0 19.5s-36.9 10.8-46.1 11.2-15.4-2.9-28.2 2.5c-12.9 5.4 19.5 5.8-19.1 5.4-38.6-.4-27.4 2.1-46.1-3.3-18.7-5.4-50.6-10.4-50.6-10.4s-81.4-19.1-85.5-20.3c-4.2-1.2-18.3-8.7-23.7-7.9-5.4.8-2.9 5.8-13.7.8-10.7-4.9-14.9-9.9-14.9-9.9z' fill='%23666'/%3E%3Cpath d='M114.9 350.4s-.6.6 5.4 2.6 5.5 1.4 8.9 2c3.4.6 4.2.3 8.7 3.8s2.6 1.4 8.4 2.5c5.7 1 19.2 15.1 47.8 17.2 28.6 2.1 23.7 1.3 34.7 2.5 11 1.2 8.2-2.9 27.4 5s16.8 5.8 27 8.7 40.4-3.8 47-3.8c6.5 0 3.3-6.2 15.9-1.7 12.7 4.6 18.8 6.2 22.9 6.2 4.1 0 7-5.4 15.5-7.9 8.6-2.5 18.8-10 24.9-11.2 6.1-1.2 9.8-7.9 9.8-7.9l7-13.4s2.5-1.1 2.3-2.5c-.2-1.4-5.4-6.7-5.4-6.7l-3-8.6-.6 1.9c3.2 6.1 3.2 8.9-2.3 15.5-.8 1-2.1 2-3.6 2.8-10.3 5.8-33.9 8-41.7 8.4-9 .4-15.1-2.9-27.8 2.5-8.6 3.7-2.8 5.1-6.3 5.5-1.6.2-.4 0-12.5-.1-38-.4-27 2.1-45.4-3.3S230 360 230 360s-80.1-19.1-84.2-20.3c-4.1-1.2-18-8.7-23.3-7.9-5.3.8-2.9 5.8-13.5.8s-14.7-10-14.7-10l2-19.1s-1-3.1-.9-6.4c-1.8 1.4-4.1 3.1-5.1 7.6-1 4.6-6.7 16.8-6.7 19.9 0 3.1 4.7 8.5 8 13.9s14.3 12.7 14.3 12.7l9-.8z' fill='%235c5c5c'/%3E%3Cpath d='M284 247.5s16.6-24.4 21.3-45.6c4.8-21.2 4.8-25.5 4.8-25.5l17.5-25.2c1.2-1.8 2.6-8.9-1.1-11.9-5-6.5-10.8-7.8-15.8-4.8-3.7 2.3-7.7 9.5-7.3 13l-.4 6.1c-9.6 10-8.8 10.4-12.7 14.2-1.5 1.4-5.9-2.5-5.8-2.4.5.6 4.2-14.5 4.2-22.4 0-7.9 13.2-23 16.9-31.9 3.7-9-8.8-19.6-18-8.2-9.2 11.4-13.9 25-13.9 25l-8.7 31.6-7-.4s-1.3-26.4-.4-31c.5-2.3 3.9-11.6 7-20.8 3-8.9 5.7-17.8 5.2-20.5-1.1-5.6-13.2-10.4-19.5-.8-6.4 9.5-12.7 46.4-13.2 48-.5 1.6-1.6 11.7-1.6 16.2s-.1 10.1-.1 10.1l-5.3-2.5c-3.6-1.7-3.2-8.5-3.9-15.6-.8-8.8-2-18.4.3-22.8 4.1-7.6 1.5-10.4-4.4-16.2-2.1-.1-12.7-3.3-16.5 3-4.8 6.9-3.2 12.1-2.9 17.9.6 10.5 3.7 20.5 3.7 28.9 0 1.2 1.7 6.2 3.3 11.2-3.6 2-8.1 4.3-11.3 5.5-6.4 2.4-7.2 5-7.2 7.9 0 2.9 3.7 16.7 4.2 25.7.4 7.2 10.4 15 17.2 19 6.1 15.2 12.4 10.3 14 21.3 2.5 16.4-2 48.6-6.2 79.9 9.2-2 17.6-2.2 24.1-.6 7.3 1.8 5.4 4.7 16.2-2.9s1-10.5 19.1-2.6l2.3 1 1.9-70.9z' fill='%23edd6c0'/%3E%3Cpath fill='%23999' d='M253 82.9c1.8-3.9 7.1-14.9 13.2-18 6.1-3 7.1 2.4 7.1 2.4-.4 3.3-3 13.1-4.3 17.4-2.6-3.9-10.3-6.4-16-1.8zM211.1 106.4s9.4 15.5 16.7 15c4-.3.1-9.2-1-10.9l-2.9-4.2c-4.7-1.3-9-1.5-12.8.1zM305.7 146.8l6.2-9.7c2.6-1.2 9.1.5 9.7 4.1.3 1.7-5 6.5-9.4 11.9-4.8 5.9-14.6 7.6-6.5-6.3zM292.9 98.6c3.4-4.2 8.5-9.5 13.1-10.8 6.5-1.8 6.6 3.7 6.6 3.7-.9 2.7-4.1 9.5-6.4 14-1.2-5.3-7.2-9.5-13.3-6.9z'/%3E%3Cpath d='M221.9 164s4.4 9.5 1.6 14.6c-2.8 5-9.5 6.8-12 8.5l1.5 3.2s27.9-4.5 36.9 13.4 9 18.1 9 18.1 7.8-18.5 13.4-27.3c5.7-8.8 8.4-11.3 8.4-11.3s-9 1.6-15.6-1c-6.6-2.5-7-3.7-13.4-3.3-5.1.3-16.8 7.3-25 1.2 2.1-10-2.5-14.7-4.8-16.1z' fill='%23ddc8b5'/%3E%3Cpath fill='%23999' d='M205.8 167.3s22.3-10.3 24.5-10.6 3.2 1.3.7 4.2-22.7 13.3-22.7 13.3-1.7-5.8-2.5-6.9z'/%3E%3C/svg%3E%0A";

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: ValidAccountId, total_supply: U128) -> Self {
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "ZomMine Token".to_string(),
                symbol: "ZML".to_string(),
                icon: Some(FT_IMAGE_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(
        owner_id: ValidAccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
        };
        this.token.internal_register_account(owner_id.as_ref());
        this.token.internal_deposit(owner_id.as_ref(), total_supply.into());
        this
    }

    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }

    #[payable]
    pub fn ft_mint(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
    ) {
        //get initial storage usage
        assert_eq!(amount.0, 0, "Cannot mint tokens, just 0 for approve");

        let initial_storage_usage = env::storage_usage();

        let mut amount_for_account = self.token.accounts.get(&receiver_id).unwrap_or(0);
        amount_for_account += amount.0;

        self.token.accounts.insert(&receiver_id, &amount_for_account);
        self.token.total_supply = self
            .token
            .total_supply
            .checked_add(amount.0)
            .unwrap_or_else(|| env::panic(b"Total supply overflow"));

        //refund any excess storage
        let storage_used = env::storage_usage() - initial_storage_usage;
        let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
        let attached_deposit = env::attached_deposit();

        assert!(
            required_cost <= attached_deposit,
            "Must attach {} yoctoNEAR to cover storage", required_cost
        );

        let refund = attached_deposit - required_cost;
        if refund > 1 {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::{Balance, testing_env};
    use near_sdk::MockedBlockchain;
    use near_sdk::test_utils::{accounts, VMContextBuilder};

    use super::*;

    const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
        assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(2).into(), TOTAL_SUPPLY.into());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());
        let transfer_amount = TOTAL_SUPPLY / 3;
        contract.ft_transfer(accounts(1), transfer_amount.into(), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert_eq!(contract.ft_balance_of(accounts(2)).0, (TOTAL_SUPPLY - transfer_amount));
        assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
    }
}
