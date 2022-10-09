use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::{testing_env};

use super::*;

// Allows for modifying the environment of the mocked blockchain
fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
        .current_account_id(accounts(0))
        .signer_account_id(predecessor_account_id.clone())
        .predecessor_account_id(predecessor_account_id);
    builder
}

#[test]
fn create_user() {
    let mut context = get_context(accounts(1));
    // Initialize the mocked blockchain
    testing_env!(context.build());

    // Set the testing environment for the subsequent calls
    testing_env!(context
            .predecessor_account_id(accounts(1))
            .build());

    let mut contract = InsoulBase::new(accounts(1), accounts(2));
    contract.create_soulbound("John Snow".to_string());

    assert_eq!(
        "John Snow".to_string(),
        contract.get_soulbound(accounts(1)).unwrap().name
    );
}

#[test]
fn add_news() {
    let mut context = get_context(accounts(1));
    // Initialize the mocked blockchain
    testing_env!(context.build());

    // Set the testing environment for the subsequent calls
    testing_env!(context
            .predecessor_account_id(accounts(1))
            .build());

    let mut contract = InsoulBase::new(accounts(1), accounts(2));
    let mut user = contract.create_soulbound("John Snow".to_string());

    assert_eq!(1, user.news.len());
}