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

use crate::*;



/// Metadata for the NFT contract itself.
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Event {
    name: String,              // required, ex. "Mosaics"
    location: String,  
    date: String,  
    host: AccountId,        // required, ex. "MOSIAC"
    bio: Option<String>,      // Data URL
    email: Option<String>, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
    eventid: String ,
}

impl Event {

    // pub fn new() -> Self {
    //     assert!(!env::state_exists(), "Already initialized");
    //     Self {
    //         // event_list: UnorderedMap::<AccountId, EventMetadata>::new(StorageKey::EventMetadata)
    //         // bio: Some(String),
    //         // email: Some(String),
    //         eventid: String::new(),
    //         name: String::new(),
    //         location: String::new(),
    //         date: String::new(),
    //     }
    // }
    
    pub fn create_event(hostid: ValidAccountId, metadata: serde_json::Value) -> Event{
        // let date_only = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?;
        // https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html#parse-string-into-datetime-struct
        let _event_definations: Event = serde_json:: from_str(&metadata.to_string()).unwrap();
        _event_definations
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_location(&self) -> String {
        self.location.clone()
    }

    pub fn get_date(&self) -> String {
        self.date.clone()
    }

    pub fn get_host(&self) -> String {
        self.host.to_string()
    }

    pub fn get_bio(&self) -> String {
        self.bio.as_ref().unwrap().to_string()
    }

    pub fn get_email(&self) -> String {
        self.email.as_ref().unwrap().to_string()
    }

    pub fn set_bio(&mut self, bio: String) {
        self.bio = Some(bio);
    }

    pub fn set_email(&mut self, email: String) {
        self.email = Some(email);
    }
     
}

impl Drop for Event {
    fn drop(&mut self) {
        println!("Deleting the event!");
    }
    // drop(obj);
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Host {
    profile_image : Option<String>,              // required, ex. "Mosaics"
    name: String,  
    weblinks: Option<Vec<String>>,  
    accountid: AccountId,        // required, ex. "MOSIAC"
    bio: Option<String>,      // Data URL
    email: Option<String>, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
    events: Option<Vec<Event>>,
}

impl Host {

    // pub fn new() -> Self {
    //     assert!(!env::state_exists(), "Already initialized");
    //     Self {
    //         weblinks: Some(Vec:: new()),
    //         events: Some(Vec:: new()),
    //         profile_image: Some(String),
    //         bio: Some(String),
    //         email: Some(String),
    //         accountid: String::new(),
    //         name: String::new(),
    //     }
    // }

    pub fn create_host(host_id: AccountId, metadata: serde_json::Value) -> Host {
        // let date_only = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?;
        // https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html#parse-string-into-datetime-struct
        let host_definations: Host = serde_json:: from_str(&metadata.to_string()).unwrap();
        host_definations
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_profile_image(&self) -> String {
        self.profile_image.as_ref().unwrap().to_string()
    }

    pub fn get_weblinks(&self) -> Vec<String> {
        self.weblinks.as_ref().unwrap().to_vec()
    }

    pub fn get_accountid(&self) -> String {
        self.accountid.clone()
    }

    pub fn get_bio(&self) -> String {
        self.bio.as_ref().unwrap().to_string()
    }

    pub fn get_events(&self) -> Vec<Event> {
        self.events.as_ref().unwrap().to_vec()
    }

    pub fn get_email(&self) -> String {
        self.email.as_ref().unwrap().to_string()
    }

    pub fn set_bio(&mut self, bio: String) {
        self.bio = Some(bio);
    }

    pub fn set_email(&mut self, email: String) {
        self.email = Some(email);
    }
     
}

impl Drop for Host {
    fn drop(&mut self) {
        println!("Deleting the host!");
    }
    // drop(obj);
}