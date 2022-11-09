use near_contract_standards::non_fungible_token::core::NonFungibleTokenCore;
use near_contract_standards::non_fungible_token::metadata::{NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata};
use near_contract_standards::non_fungible_token::{NonFungibleToken, Token, TokenId};
use near_sdk::{AccountId, env, near_bindgen, PanicOnDefault, Promise, PromiseOrValue, BorshStorageKey, Balance, assert_one_yocto};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupSet, UnorderedMap};
use near_sdk::json_types::U128;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct SBT {
    token_id: u128,
    token: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    approvals: LookupSet<AccountId>,
    donors: UnorderedMap<AccountId, Balance>
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl SBT {
    pub fn assert_only_owner(&self) {
        assert_one_yocto();
        assert_eq!(
            env::predecessor_account_id(),
            self.token.owner_id,
            "Only owner can call this method"
        );
    }

    pub fn assert_not_owner(&self) {
        assert_ne!(
            env::predecessor_account_id(),
            self.token.owner_id,
            "Owner can't call this method"
        );
    }

    /// Approval from the owner is required for minting.
    pub fn assert_mint_approved(&self) {
        assert!(
            self.approvals.contains(&env::predecessor_account_id()),
            "Mint was not approved by owner"
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists());
        metadata.assert_valid();

        Self {
            token_id: 0,
            token: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval)
            ),
            metadata: LazyOption::new(StorageKey::TokenMetadata, Some(&metadata)),
            approvals: LookupSet::new(b"a".to_vec()),
            donors: UnorderedMap::new(b"d".to_vec()),
        }
    }

    #[payable]
    pub fn approve_mint(&mut self, account_id: AccountId) {
        self.assert_only_owner();
        self.approvals.insert(&account_id);
    }

    #[payable]
    pub fn mint(
        &mut self,
        token_metadata: TokenMetadata,
    ) -> Token {
        self.assert_not_owner();
        self.assert_mint_approved();

        self.token_id += 1;
        let token = self.token.internal_mint(
            self.token_id.to_string(),
            self.token.owner_id.clone(),
            Some(token_metadata),
        );

        // Approval from the owner can only be used once, so remove minter from the approvals set.
        self.approvals.remove(&env::predecessor_account_id());

        token
    }

    pub fn update_metadata(&mut self, metadata: NFTContractMetadata) {
        metadata.assert_valid();
        self.metadata.set(&metadata);
    }

    pub fn get_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }

    #[payable]
    pub fn donate(&mut self) {
        let mut sum = match self.donors.get(&env::predecessor_account_id()) {
            Some(v) => v,
            None => 0,
        };
        sum += env::attached_deposit();

        self.donors.insert(&env::predecessor_account_id(), &sum);
    }
}

/*
 * Implement NonFungibleTokenCore for the Soulbound contract
 * to suppress token transfer.
 */
impl NonFungibleTokenCore for SBT {
    fn nft_transfer(&mut self, _: AccountId, _: TokenId, _: Option<u64>, _: Option<String>) {
        env::panic_str("Soulbound can not be transferred");
    }

    fn nft_transfer_call(&mut self, _: AccountId, _: TokenId, _: Option<u64>, _: Option<String>, _: String) -> PromiseOrValue<bool> {
        env::panic_str("Soulbound can not be transferred");
    }

    fn nft_token(&self, token_id: TokenId) -> Option<Token> {
        self.token.nft_token(token_id)
    }
}

near_contract_standards::impl_non_fungible_token_approval!(SBT, token);
near_contract_standards::impl_non_fungible_token_enumeration!(SBT, token);
