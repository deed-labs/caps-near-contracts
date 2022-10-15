use near_contract_standards::non_fungible_token::metadata::NFTContractMetadata;
use near_sdk::AccountId;
use near_sdk::borsh::{
    self,
    BorshDeserialize,
    BorshSerialize,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, Serialize, BorshDeserialize, BorshSerialize)]
pub struct SoulboundInitArgs {
    pub owner_id: AccountId,
    pub metadata: NFTContractMetadata,
}