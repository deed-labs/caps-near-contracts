use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, AccountId};
use soulbound::Soulbound;
use user_token::SubToken;
use crate::user::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct InsoulBase {
    owner_id: AccountId,
    soul_token: AccountId,
    soulbounds: LookupMap<AccountId, Soulbound>,
}

#[near_bindgen]
impl InsoulBase {
    #[init]
    pub fn new(owner_id: AccountId, soul_token: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");

        Self {
            owner_id,
            soul_token,
            soulbounds: LookupMap::new(b"r".to_vec())
        }
    }

    pub fn create_soulbound(&mut self, name: String) -> User {
        let account_id = env::signer_account_id();

        // TODO

        user
    }

    pub fn get_soulbound(&self, account_id: AccountId) -> Option<User> {
        return self.soulbounds.get(&account_id);
    }
}

#[cfg(test)]
mod tests;