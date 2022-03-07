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

use crate::*;


/// Metadata for the NFT contract itself.
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct EventMetadata {
    name: String,              // required, ex. "Mosaics"
    location: String,  
    date: String,  
    host: AccountId,        // required, ex. "MOSIAC"
    bio: Option<String>,      // Data URL
    email: Option<String>, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
}

pub trait EventProvider {
    // fn get_name(&self) -> String;
    // fn get_location(&self) -> String;
    // fn get_date(&self) -> String;
    // fn get_host(&self) -> AccountId;
    fn get_bio(&self) -> String;
    fn get_email(&self) -> String;
}

impl EventProvider for EventMetadata {
    
    // fn get_name(&self) -> String {
    //     self.name
    // }
    // fn get_location(&self) -> String {
    //     self.location
    // }
    // fn get_date(&self) -> String {
    //     self.date
    // }
    // fn get_host(&self) -> AccountId {
    //     self.host
    // }
    fn get_bio(&self) -> String {
        self.bio.as_ref().unwrap().to_string()
    }
    fn get_email(&self) -> String {
        self.email.as_ref().unwrap().to_string()
    }
}

pub trait EventMetadataProvider {
    fn all_events(&self) -> Vec<(AccountId, EventMetadata)>;
    fn view_event(&self, account_id : AccountId) -> EventMetadata;
}


