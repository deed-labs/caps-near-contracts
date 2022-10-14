use near_contract_standards::non_fungible_token::metadata::NFTContractMetadata;
use near_sdk::AccountId;
use near_sdk::borsh::{
    self,
    BorshDeserialize,
    BorshSerialize,
};
use near_sdk::serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "wasm", derive(BorshDeserialize, BorshSerialize))]
pub struct SoulboundInitArgs {
    pub metadata: NFTContractMetadata,
    pub owner_id: AccountId
}