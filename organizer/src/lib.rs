use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::ValidAccountId;
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, Gas, Promise, Balance,
    serde_json::{json}};
use near_sdk::collections::UnorderedMap;
use near_sdk::collections::Vector;
use near_sdk::collections::LookupMap;
use serde_json::Map;
use serde_json::Value;
use near_sdk::collections::UnorderedSet;


mod structs;

mod traits;

use crate::structs::Event;


pub use structs::*;

pub use traits::*;

const NO_DEPOSIT: Balance = 0;
const GAS: Gas = 20_000_000_000_000;

near_sdk::setup_alloc!();

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Event,
    // Host,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    event_list: UnorderedMap<AccountId, UnorderedSet<Event>>,
    host_list: UnorderedSet<AccountId>,
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
            host_list: UnorderedSet::new(b"s".to_vec()),
        }
    }
   
    pub fn add_event(&mut self, hostid: AccountId, metadata: serde_json::Value, date: String) -> String{
        
        let mut event = Event::create_event(hostid.clone(),metadata);
        event.set_date(date);
        if !(self.host_list.contains(&hostid)) {
            
            self.host_list.insert(&hostid);
            let _set: UnorderedSet<Event> = UnorderedSet::new(b"w".to_vec());
            self.event_list.insert(&hostid, &_set);
        }

        let mut set_test = self.event_list.get(&hostid).unwrap();
        set_test.insert(&event);
        self.event_list.insert(&hostid, &set_test);

        // self.event_list.get(&hostid).insert(&event);
        
        event.get_name()
    }

    pub fn all_events(&mut self, hostid: AccountId) -> Vec<structs::Event> {
        self.event_list.get(&hostid).unwrap().to_vec()
    }

    // pub fn check_balance_contract_b(&mut self, contract_a: AccountId, contract_b: AccountId, account_id: AccountId) -> Promise {
    //     Promise::new(contract_b).function_call(
    //         b"check_balance".to_vec(),
    //         json!({ "contract_a": contract_a, "method_name": "check_balance_callback", "account_id": account_id }).to_string().as_bytes().to_vec(),
    //         NO_DEPOSIT,
    //         GAS,
    //     )
    // }

    pub fn mint_nft(contract_a: AccountId, contract_b: AccountId) -> Promise {
        Promise:: new(contract_b.clone()).function_call(
            b"nft_metadata_call".to_vec(),
            json!({ "account_id": contract_b }).to_string().as_bytes().to_vec(),
            NO_DEPOSIT,
            GAS,
        )
    }

    // pub fn delete_event(&mut self, hostid: AccountId) {
    //     let mut event = self.event_list.get(&hostid).unwrap().to_vec();
    //     self.event_list.get(&hostid).unwrap().remove(event[0]);
    //     drop(event[0])
    // }

    // pub fn add_host(&mut self, hostid: AccountId, metadata: serde_json::Value){
        
    //     let host = Host::create_host(hostid.clone(), metadata);
    //     self.host_list.insert(&hostid, &host);
        
    // }

    // pub fn all_hosts(&self) -> Vec<(std::string::String, structs::Host)> {
    //     self.host_list.to_vec()
    //     // let v: Value = serde_json::from_str(data)?;
    // }

    // #[result_serializer(borsh)]
    // fn all_event(&self, hostid: AccountId) -> Vec<Event> {
    //     let mut host = self.host_list.get(&hostid).unwrap();
    //     host.get_events()
    // }

    // fn delete_host(&mut self, hostid: AccountId) {
    //     let host = self.host_list.get(&hostid).unwrap();
    //     self.host_list.remove(&hostid);
    //     drop(host);
    // }



}


// #[near_bindgen]
// impl EventMetadataProvider for Contract {
    
//     // fn all_events(&self) -> Vec<(std::string::String, structs::Event)> {
//     //     self.event_list.to_vec()
//     // }

//     // fn view_event(&self, account_id : AccountId) -> Vec<(std::string::String, UnorderedSet<structs::Event>)> {
//     //     self.event_list.get(&account_id).unwrap()
//     // }
// }