pub use core_interfaces::*;

mod core_interfaces {
    use near_sdk::AccountId;
    use near_sdk::{self, ext_contract};
    use near_sdk::json_types::U128;

    #[ext_contract(core_self)]
    pub trait OnCreateCallback {
        fn on_create(
            &mut self,
            sb_creator_id: AccountId,
            sb_account_id: AccountId,
            attached_deposit: U128
        );
    }
}