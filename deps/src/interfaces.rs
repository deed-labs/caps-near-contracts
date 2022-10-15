pub use core_interfaces::*;

mod core_interfaces {
    use near_contract_standards::non_fungible_token::metadata::NFTContractMetadata;
    use near_sdk::AccountId;
    use near_sdk::ext_contract;

    #[ext_contract(core_self)]
    pub trait OnCreateCallback {
        fn on_create(
            &mut self,
            sb_creator_id: AccountId,
            metadata: NFTContractMetadata,
            owner_id: AccountId,
            sb_account_id: AccountId
        );
    }
}