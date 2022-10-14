use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct InsoulCore {
    owner_id: AccountId,
    soul_token: AccountId,
    soulbounds: LookupMap<AccountId, AccountId>,
}

impl Default for InsoulCore {
    fn default() -> Self {
        env::panic_str("Not implemented yet.");
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

    pub fn create_soulbound(&mut self, _name: String) {
        let account_id = env::signer_account_id();

        assert!(!self.soulbounds.contains_key(&account_id), "Soulbound for account already exists");

        // TODO
        }

    pub fn get_soulbound(&self, account_id: AccountId) -> Option<AccountId> {
        return self.soulbounds.get(&account_id);
    }
}

#[cfg(test)]
mod tests;