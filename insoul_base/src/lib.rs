mod user;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, AccountId};
use crate::user::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct InsoulBase {
    owner_id: AccountId,
    users: LookupMap<AccountId, User>,
}

#[near_bindgen]
impl InsoulBase {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");

        Self {
            owner_id,
            users: LookupMap::new(b"r".to_vec())
        }
    }

    pub fn create_user(&mut self, name: String) -> User {
        let account_id = env::signer_account_id();

        let user = User::new(name);
        self.users.insert(&account_id, &user);

        user
    }

    pub fn get_user(&self, account_id: AccountId) -> Option<User> {
        return self.users.get(&account_id);
    }
}

#[cfg(test)]
mod tests;