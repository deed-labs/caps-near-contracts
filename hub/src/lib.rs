use std::str::FromStr;
use near_contract_standards::non_fungible_token::metadata::NFTContractMetadata;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, AccountId, Promise, PublicKey, assert_one_yocto, is_promise_success, Balance};
use near_sdk::json_types::U128;
use deps::common::{SoulboundInitArgs};
use deps::common::soulbound_init_args::SoulboundUpdateArgs;
use deps::constants::{gas, storage_bytes, storage_cost, YOCTO_PER_BYTE};
use deps::interfaces::core_self;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Hub {
    pub owner_id: AccountId,
    pub admin_public_key: PublicKey,
    pub hub_fee: Balance,
    pub storage_price_per_byte: u128,
    pub soulbound_cost: u128,
    pub soulbounds: LookupMap<AccountId, AccountId>,
}

const SOULBOUND_CODE: &[u8] = include_bytes!("../../wasm/soulbound.wasm");

impl Default for Hub {
    fn default() -> Self {
        env::panic_str("Not implemented yet");
    }
}

#[near_bindgen]
impl Hub {
    pub fn assert_only_owner(&self) {
        assert_one_yocto();
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only contract owner can call this method"
        );
    }

    pub fn assert_sufficient_attached_deposit(&self) {
        let min = storage_bytes::SOULBOUND as u128 * self.storage_price_per_byte + self.hub_fee;
        assert!(
            env::attached_deposit() >= min,
            "Not enough attached deposit to complete store deployment. Need: {}, got: {}",
            min,
            env::attached_deposit()
        );
    }

    /// Only one `Soulbound` can be created and linked to the account.
    pub fn assert_soulbound_not_exists(&self) {
        assert!(
            !self.soulbounds.contains_key(&env::predecessor_account_id()),
            "Soulbound for the account already exists"
        );
    }

    pub fn assert_soulbound_exists(&self) {
        assert!(
            self.soulbounds.contains_key(&env::predecessor_account_id()),
            "Soulbound for the account not exists"
        );
    }

    #[payable]
    pub fn set_hub_fee(&mut self, amount: U128) {
        self.assert_only_owner();
        self.hub_fee = amount.into();
    }

    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");

        Self {
            owner_id: env::predecessor_account_id(),
            admin_public_key: env::signer_account_pk(),
            hub_fee: 0,
            storage_price_per_byte: YOCTO_PER_BYTE,
            soulbound_cost: storage_cost::STORE,
            soulbounds: LookupMap::new(b"r".to_vec()),
        }
    }

    #[private]
    pub fn on_create(
        &mut self,
        sb_creator_id: AccountId,
        sb_account_id: AccountId,
        attached_deposit: U128
    ) {
        let attached_deposit: u128 = attached_deposit.into();
        if is_promise_success() {
            self.soulbounds.insert(&sb_creator_id, &sb_account_id);
            Promise::new(self.owner_id.to_string().parse().unwrap())
                .transfer(attached_deposit - self.soulbound_cost);
       } else {
            Promise::new(sb_creator_id)
                .transfer(attached_deposit - self.soulbound_cost);
           env::log_str("soulbound deployment failed");
       }
    }

    #[payable]
    pub fn create_soulbound(&mut self, metadata: NFTContractMetadata) -> Promise {
        self.assert_soulbound_not_exists();
        self.assert_sufficient_attached_deposit();

        let metadata = NFTContractMetadata::new(metadata);
        let init_args = serde_json::to_vec(&SoulboundInitArgs {
            owner_id: env::predecessor_account_id(),
            metadata: metadata.clone(),
        }).unwrap();

        let sb_account_id = AccountId::from_str(&*format!("{}.{}", metadata.name,
                                                          env::current_account_id())).unwrap();

        let promise = Promise::new(sb_account_id.clone())
            .create_account()
            .transfer(self.soulbound_cost)
            .deploy_contract(SOULBOUND_CODE.to_vec())
            .add_full_access_key(env::signer_account_pk())
            .function_call("new".to_string(), init_args, 0, gas::CREATE_SOULBOUND);

        promise.then(
        core_self::ext(env::current_account_id())
            .with_static_gas(gas::ON_CREATE_CALLBACK)
            .on_create(
                env::predecessor_account_id(),
                sb_account_id,
                env::attached_deposit().into()
            )
        )
    }

    #[payable]
    pub fn update_soulbound(&mut self, metadata: NFTContractMetadata) -> Promise {
        self.assert_soulbound_exists();

        let metadata = NFTContractMetadata::new(metadata);
        let update_args = serde_json::to_vec(&SoulboundUpdateArgs {
            metadata: metadata.clone(),
        }).unwrap();

        let sb_account_id = self.soulbounds.get(&env::predecessor_account_id()).unwrap();

        Promise::new(sb_account_id)
            .function_call("update_metadata".to_string(), update_args, 0, gas::UPDATE_SOULBOUND)
    }

    pub fn get_soulbound_id_for_account(&self, account_id: AccountId) -> AccountId {
        self.soulbounds.get(&account_id).unwrap_or_else(|| env::panic_str("Not found"))
    }
}

pub trait New {
    fn new(arg: Self) -> Self;
}

impl New for NFTContractMetadata {
    fn new(args: NFTContractMetadata) -> Self {
        let soulbound_account = format!("{}.{}", args.name, env::current_account_id());
        assert!(
            env::is_valid_account_id(soulbound_account.as_bytes()),
            "Invalid character in soulbound name"
        );
        assert!(args.symbol.len() <= 6);

        Self {
            spec: args.spec,
            name: args.name,
            symbol: args.symbol,
            icon: args.icon,
            base_uri: args.base_uri,
            reference: args.reference,
            reference_hash: args.reference_hash,
        }
    }
}