use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::LazyOption;
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey};



use crate::*;



pub trait EventProvider {
    // fn get_name(&self) -> String;
    // fn get_location(&self) -> String;
    // fn get_date(&self) -> String;
    // fn get_host(&self) -> AccountId;
    fn get_bio(&self) -> String;
    fn get_email(&self) -> String;
}

pub trait EventMetadataProvider {
    fn all_events(&self) -> Vec<(std::string::String, structs::Host)>;
    // fn view_event(&self, account_id : AccountId) -> Vec<(std::string::String, UnorderedSet<structs::Event>)>;
}