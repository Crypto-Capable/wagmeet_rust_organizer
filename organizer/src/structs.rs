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
    // date: Option<i64>,  
    pub host: AccountId, 
    pub event_address : Option<String> 
    // bio: Option<String>, 
    // email: Option<String>,
    // no_tickets: i64,
}
pub struct TokenMetadata {
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub copies: Option<u64>, // number of copies of this set of metadata in existence when token was minted.
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
    pub extra: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
    // pub is_attended: Option<bool>, -> V2
}

impl Event {

    pub fn create_event(hostid: AccountId, metadata: &serde_json::Value) -> Event {
        let id = env::current_account_id();
        let event_definations: Event = serde_json:: from_str(&metadata.to_string()).unwrap();

        let event_name = String::from(event_definations.name.to_string());
        let subaccount_name = format!("{}.{}", &event_name[0..5], id.clone());
        let event_account = &subaccount_name.to_lowercase().trim().to_string();
        // let date_format = NaiveDate::parse_from_str(&event_definations.date.to_string(), "%d-%m-%Y").unwrap();
        Event {
            name: event_definations.name.to_string(),
            location: event_definations.location.to_string(),
            description: event_definations.description.to_string(),
            symbol: event_definations.symbol.to_string(),
            // date: None,
            host: hostid,
            event_address : Some(event_account.to_string()),
            // bio: None,
            // email: None,
            // no_tickets: event_definations.no_tickets,
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
        log!("EVENT ADDRESS : {}", address);
        self.event_address = Some(address);
    }

    // pub fn get_date(&self) -> std::option::Option<i64> {
    //     self.date.clone()
    // }

    // pub fn get_bio(&self) -> String {
    //     self.bio.as_ref().unwrap().to_string()
    // }

    // pub fn get_email(&self) -> String {
    //     self.email.as_ref().unwrap().to_string()
    // }

    // pub fn get_no_tickets(&mut self) -> i64 {
    //     self.no_tickets.clone()
    // }

    // pub fn set_date(&mut self, date : String) {
    //     let d = NaiveDate::parse_from_str(&date.to_string(), "%d-%m-%Y").unwrap();
    //     let t = NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap();
    //     let dt = NaiveDateTime::new(d, t);

    //     self.date = Some(dt.timestamp());

    // }
    // // 2428113290324
    // // 30000000000000
    // pub fn set_bio(&mut self, bio: String) {
    //     self.bio = Some(bio);
    // }

    // pub fn set_email(&mut self, email: String) {
    //     self.email = Some(email);
    // }

    // pub fn set_no_tickets(&mut self, number : i64) {
    //     self.no_tickets = number;
    // }
     
}

impl Drop for Event {
    fn drop(&mut self) {
        println!("Deleting the event!");
    }
    // drop(obj);
}




// #[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
// #[serde(crate = "near_sdk::serde")]
// pub struct Host {
//     profile_image : Option<String>,              // required, ex. "Mosaics"
//     name: String,  
//     weblinks: Option<Vec<String>>,  
//     accountid: AccountId,        // required, ex. "MOSIAC"
//     bio: Option<String>,      // Data URL
//     email: Option<String>, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
//     // events: Vec<Event>,
//     events: Option<Event>,
// }

// impl Host {

//     pub fn create_host(hostid: AccountId, metadata: serde_json::Value) -> Host {
//         // let date_only = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?;
//         // https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html#parse-string-into-datetime-struct
//         let host_definations: Host = serde_json:: from_str(&metadata.to_string()).unwrap();
//         let host = Host {
//             profile_image: None,
//             name: host_definations.name.to_string(),
//             weblinks: None,
//             accountid: hostid,
//             bio: None,
//             email: None,
//             events: None,
//         };
//         host
//     }

    

//     pub fn get_name(&self) -> String {
//         self.name.clone()
//     }

//     pub fn get_profile_image(&self) -> String {
//         self.profile_image.as_ref().unwrap().to_string()
//     }

//     pub fn get_weblinks(&self) -> Vec<String> {
//         self.weblinks.as_ref().unwrap().to_vec()
//     }

//     pub fn get_accountid(&self) -> String {
//         self.accountid.to_string()
//     }

//     pub fn get_bio(&self) -> String {
//         self.bio.as_ref().unwrap().to_string()
//     }

//     // pub fn get_events(&self) -> Vec<Event> {
//     //     self.events.to_string()
//     // }

//     pub fn get_email(&self) -> String {
//         self.email.as_ref().unwrap().to_string()
//     }

//     pub fn set_bio(&mut self, bio: String) {
//         self.bio = Some(bio);
//     }

//     pub fn set_email(&mut self, email: String) {
//         self.email = Some(email);
//     }
     
// }

// impl Drop for Host {
//     fn drop(&mut self) {
//         println!("Deleting the host!");
//     }
//     // drop(obj);
// }