use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::Base64VecU8;
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


mod modules;

pub use modules::*;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    event_list: UnorderedMap<AccountId, EventMetadata>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    EventMetadata,
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
            event_list: UnorderedMap::<AccountId, EventMetadata>::new(StorageKey::EventMetadata)
        }
    }
   
    pub fn add_event(&mut self, host_id: AccountId, metadata: serde_json::Value) {
        let event_definations: EventMetadata = serde_json:: from_str(&metadata.to_string()).unwrap();
        self.event_list.insert(&host_id, &event_definations);
    }

    // pub fn delete_event(&mut self, host_id: AccountId) {
    //     self.event_list.insert(&host_id, &event_definations);
    // }


}


#[near_bindgen]
impl EventMetadataProvider for Contract {
    
    fn all_events(&self) -> Vec<(AccountId, EventMetadata)> {
        self.event_list.to_vec()
    }

    fn view_event(&self, account_id : AccountId) -> EventMetadata {
        self.event_list.get(&account_id).unwrap()
    }
}