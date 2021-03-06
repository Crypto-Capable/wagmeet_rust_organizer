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
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;
use std::ptr::null;
use std::fmt::Debug;

use crate::*;



/// Metadata for the NFT contract itself.
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Event {
    pub name: String,
    pub description : String,
    pub symbol : String,  
    pub location: String,  
    pub host: AccountId, 
    pub event_address : Option<String> 
}

impl Event {

    pub fn create_event(hostid: AccountId, metadata: &serde_json::Value) -> Event {
        let id = env::current_account_id();
        let event_definations: Event = serde_json:: from_str(&metadata.to_string()).unwrap();

        // FIXME: add event address from organizer contract and remove below lines.
        let event_name = String::from(event_definations.name.to_string());
        let subaccount_name = format!("{}.{}", &event_name[0..5], id.clone());
        let event_account = &subaccount_name.to_lowercase().trim().to_string();
        // let date_format = NaiveDate::parse_from_str(&event_definations.date.to_string(), "%d-%m-%Y").unwrap();
        Event {
            name: event_definations.name.to_string(),
            location: event_definations.location.to_string(),
            description: event_definations.description.to_string(),
            symbol: event_definations.symbol.to_string(),
            host: hostid,
            event_address : Some(event_account.to_string()),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_location(&self) -> String {
        self.location.clone()
    }

    pub fn get_host(&self) -> String {
        self.host.to_string()
    }

    pub fn set_event_address(&mut self, address : String) {
        self.event_address = Some(address);
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        println!("Deleting the event!");
    }
    // drop(obj);
}