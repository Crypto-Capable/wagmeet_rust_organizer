use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::ValidAccountId;
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey};
use near_sdk::collections::UnorderedMap;
use near_sdk::collections::Vector;
use near_sdk::collections::LookupMap;
use serde_json::Map;
use serde_json::Value;
use near_sdk::collections::UnorderedSet;
mod modules;
mod structs;
mod enums;
mod traits;

use crate::structs::Event;

pub use modules::*;
pub use structs::*;
pub use enums::*;
pub use traits::*;

near_sdk::setup_alloc!();

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Event,
    Host,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    event_list: UnorderedMap<AccountId, UnorderedSet<Event>>,
    host_list: UnorderedMap<AccountId, Host>,
}



impl Default for Contract {
    fn default() -> Self {
        // Check incase the contract is not initialized
        env::panic(b"The contract is not initialized.")
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            event_list: UnorderedMap::<AccountId, UnorderedSet<Event>>::new(StorageKey::Event),
            host_list: UnorderedMap::<AccountId, Host>::new(StorageKey::Host),
        }
    }
   
    pub fn add_host(&mut self, hostid: ValidAccountId, metadata: serde_json::Value) -> String{
        let event = Event::create_event(hostid, metadata);
        event.get_date()
        // self.event_list.insert(&host_id,&event);
    }


}


#[near_bindgen]
impl EventMetadataProvider for Contract {
    
    fn all_events(&self) -> Vec<(std::string::String, structs::Host)> {
        self.host_list.to_vec()
    }

    // fn view_event(&self, account_id : AccountId) -> Vec<(std::string::String, UnorderedSet<structs::Event>)> {
    //     self.event_list.get(&account_id).unwrap()
    // }
}