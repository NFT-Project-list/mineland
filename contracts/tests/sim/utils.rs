use near_contract_standards::non_fungible_token::{TokenId};
use near_sdk_sim::{call, ContractAccount, deploy, init_simulator, STORAGE_AMOUNT, to_yocto, UserAccount};
use near_sdk_sim::runtime::GenesisConfig;

use ft::ContractContract as FtContract;
use main::{ContractContract as MainContract, Mine, MineType, Stone};
use nft_mine::ContractContract as MineContract;
use nft_stone::ContractContract as StoneContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    MAIN_CONTRACT_WASM_BYTES => "../out/main.wasm",
    FT_CONTRACT_WASM_BYTES => "../out/ft.wasm",
    LAND_CONTRACT_WASM_BYTES => "../out/nft-mine.wasm",
    ZOMBIE_CONTRACT_WASM_BYTES => "../out/nft-stone.wasm"
}

pub fn init() -> (
    UserAccount,
    ContractAccount<MainContract>,
    ContractAccount<FtContract>,
    ContractAccount<MineContract>,
    ContractAccount<StoneContract>,
    UserAccount
) {
    let mut genesis = GenesisConfig::default();
    genesis.genesis_time = 1000000000000000000;
    genesis.genesis_height = 1000;
    let root = init_simulator(Some(genesis));

    let main_contract = deploy! {
        contract: MainContract,
        contract_id: "main",
        bytes: &MAIN_CONTRACT_WASM_BYTES,
        signer_account: root,
        deposit: STORAGE_AMOUNT
    };

    let ft_contract = deploy! {
        contract: FtContract,
        contract_id: "ft.main",
        bytes: &FT_CONTRACT_WASM_BYTES,
        signer_account: main_contract.user_account,
        deposit: STORAGE_AMOUNT / 10,
        init_method: new_default_meta(
            main_contract.valid_account_id(),
            to_yocto("1000000000").into()
        )
    };

    let mine_contract = deploy! {
        contract: MineContract,
        contract_id: "nft-mine.main",
        bytes: &LAND_CONTRACT_WASM_BYTES,
        signer_account: main_contract.user_account,
        deposit: STORAGE_AMOUNT / 10,
        init_method: new_default_meta(
            main_contract.account_id()
        )
    };

    let stone_contract = deploy! {
        contract: StoneContract,
        contract_id: "nft-stone.main",
        bytes: &ZOMBIE_CONTRACT_WASM_BYTES,
        signer_account: main_contract.user_account,
        deposit: STORAGE_AMOUNT / 10,
        init_method: new_default_meta(
            main_contract.account_id()
        )
    };

    let alice = root.create_user(
        "alice".to_string(),
        to_yocto("20"),
    );

    // Seed data - Stone collections
    let collection_title = String::from("Collection 1");
    let collection_image = String::from("image-1");
    let stone_images = vec!["1-1".to_string(), "1-2".into(), "1-3".into()];
    let collection_add_result = call!(
        root,
        main_contract.add_collection(collection_title, collection_image, stone_images)
    );
    assert!(collection_add_result.is_ok());

    (root, main_contract, ft_contract, mine_contract, stone_contract, alice)
}


pub fn claim_mine(main_contract: &ContractAccount<MainContract>, user: &UserAccount, mine_type: MineType) -> Mine {
    let deposit: u128 = match mine_type {
        MineType::Small => to_yocto("0.01"),
        MineType::Medium => to_yocto("5"),
        MineType::Large => to_yocto("9"),
    };

    let mine_result = call!(
        user,
        main_contract.mint_mine_nft(),
        deposit = deposit
    );
    assert!(mine_result.is_ok());
    mine_result.unwrap_json()
}

pub fn claim_stones(
    main_contract: &ContractAccount<MainContract>,
    user: &UserAccount,
    mine_id: TokenId,
    mine_type: MineType,
) -> Vec<Stone> {
    let deposit: u128 = match mine_type {
        MineType::Small => to_yocto("0.01"),
        MineType::Medium => to_yocto("0.03"),
        MineType::Large => to_yocto("0.06"),
    };

    let stone_claim_result = call!(
        user,
        main_contract.mint_free_stone_nft(mine_id),
        deposit = deposit
    );

    assert!(stone_claim_result.is_ok());
    stone_claim_result.unwrap_json()
}
