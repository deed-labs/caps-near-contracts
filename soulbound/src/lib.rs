use near_contract_standards::non_fungible_token::core::NonFungibleTokenCore;
use near_contract_standards::non_fungible_token::metadata::{NFTContractMetadata, NonFungibleTokenMetadataProvider};
use near_contract_standards::non_fungible_token::{NonFungibleToken, Token, TokenId};
use near_sdk::{AccountId, env, near_bindgen, PanicOnDefault, Promise, PromiseOrValue, BorshStorageKey};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Soulbound {
    token: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>
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
impl Soulbound {
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists());
        metadata.assert_valid();

        Self {
            token: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval)
            ),
            metadata: LazyOption::new(StorageKey::TokenMetadata, Some(&metadata))
        }
    }
}

/*
 * Implement NonFungibleTokenCore for the Soulbound contract
 * to suppress token transfer.
 */
impl NonFungibleTokenCore for Soulbound {
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

near_contract_standards::impl_non_fungible_token_approval!(Soulbound, token);
near_contract_standards::impl_non_fungible_token_enumeration!(Soulbound, token);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Soulbound {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}