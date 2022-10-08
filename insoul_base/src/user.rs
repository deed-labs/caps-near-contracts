use near_sdk::{near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct User {
    pub name: String,
    pub news: Vec<String>
}

impl User {
    pub fn new(name: String) -> User {
        User {
            name,
            news: Vec::new()
        }
    }

    pub fn add_news(&mut self, news: String) {
        self.news.push(news);
    }
}