use std::borrow::Borrow;
use std::fmt::format;
use std::str::FromStr;
use near_contract_standards::non_fungible_token::metadata::NFTContractMetadata;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, AccountId, Promise, PublicKey, Gas};
use common::soulbound_init_args::SoulboundInitArgs;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct InsoulCore {
    owner_id: AccountId,
    soul_token: AccountId,
    soulbounds: LookupMap<AccountId, AccountId>,
    admin_public_key: PublicKey
}

impl Default for InsoulCore {
    fn default() -> Self {
        env::panic_str("Not implemented yet");
    }
}

#[near_bindgen]
impl InsoulCore {
    #[init]
    pub fn new(owner_id: AccountId, soul_token: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");

        Self {
            owner_id,
            soul_token,
            soulbounds: LookupMap::new(b"r".to_vec())
        }
    }

    #[private]
    pub fn on_create(
        &mut self,
        sb_creator_id: AccountId,
        metadata: NFTContractMetadata,
        owner_id: AccountId,
        sb_account_id: AccountId
    ) {
        // TODO
    }

    pub fn create_soulbound(&mut self, metadata: NFTContractMetadata) -> Promise {
        let account_id = env::signer_account_id();

        assert!(!self.soulbounds.contains_key(&account_id), "Soulbound for account already exists");

        let init_args = serde_json::to_vec(&SoulboundInitArgs {
            metadata: metadata.clone(),
            owner_id: account_id
        }).unwrap();

        let sb_account_id = AccountId::from_str(&*format!("{}.{}", metadata.name,
                                                          env::current_account_id())).unwrap();

        Promise::new(sb_account_id.clone())
            .create_account()
            .add_full_access_key(self.admin_public_key.clone())
            .deploy_contract(include_bytes!("../../wasm/soulbound.wasm").to_vec())
            // TODO: replace zero gas const
            .function_call("new".to_string(), init_args, 0, Gas::from(0))
            .then(self::on_create(
                env::predecessor_account_id(),
                metadata,
                account_id.clone(),
                sb_account_id
            ))
    }

    pub fn get_soulbound(&self, account_id: AccountId) -> Option<AccountId> {
        return self.soulbounds.get(&account_id);
    }
}

#[cfg(test)]
mod tests;