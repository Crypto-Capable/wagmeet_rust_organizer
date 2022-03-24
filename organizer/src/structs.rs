use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId};
use chrono::{ NaiveDate, NaiveDateTime, NaiveTime};


/// Metadata for the NFT contract itself.
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Event {
    name: String,              // required, ex. "Mosaics"
    location: String,  
    date: Option<i64>,  
    host: AccountId,        // required, ex. "MOSIAC"
    bio: Option<String>,      // Data URL
    email: Option<String>,
    total_tickets: i64,
    total_tickets_sold: i64,
    ticket_price: i64,
    is_mint_enabled: bool, 

}

impl Event {

    pub fn create_event(hostid: AccountId, metadata: serde_json::Value) -> Event {
        let event_definations: Event = serde_json:: from_str(&metadata.to_string()).unwrap();
        // let date_format = NaiveDate::parse_from_str(&event_definations.date.to_string(), "%d-%m-%Y").unwrap();
        Event {
            name: event_definations.name.to_string(),
            location: event_definations.location.to_string(),
            date: None,
            host: hostid,
            bio: None,
            email: None,
            total_tickets: event_definations.total_tickets,
            total_tickets_sold: 0,
            ticket_price: event_definations.ticket_price,
            is_mint_enabled: false,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_location(&self) -> String {
        self.location.clone()
    }

    pub fn get_date(&self) -> std::option::Option<i64> {
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

    pub fn get_total_tickets(&mut self) -> i64 {
        self.total_tickets.clone()
    }

    pub fn set_date(&mut self, date : String) {
        let d = NaiveDate::parse_from_str(&date.to_string(), "%d-%m-%Y").unwrap();
        let t = NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap();
        let dt = NaiveDateTime::new(d, t);

        self.date = Some(dt.timestamp());

    }
   
    pub fn set_bio(&mut self, bio: String) {
        self.bio = Some(bio);
    }

    pub fn set_email(&mut self, email: String) {
        self.email = Some(email);
    }

    pub fn set_total_tickets(&mut self, number : i64) {
        self.total_tickets = number;
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
pub struct TicketMetadata  {
    title: String,              // required, ex. "Mosaics"
    description: String,  
    organizer_id: AccountId,  
    event_id: String,
    created_at: Option<i64>,
    sold_at: Option<i64>,
    is_attendence_marked: bool,
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Ticket  {
    token_id: String,              // required, ex. "Mosaics"
    owner_id: AccountId,  
    metadata: TicketMetadata,  
}

impl Ticket {

    pub fn new(hostid: AccountId, metadata: serde_json::Value) -> Event {
        let event_definations: Event = serde_json:: from_str(&metadata.to_string()).unwrap();
        // let date_format = NaiveDate::parse_from_str(&event_definations.date.to_string(), "%d-%m-%Y").unwrap();
        Event {
            name: event_definations.name.to_string(),
            location: event_definations.location.to_string(),
            date: None,
            host: hostid,
            bio: None,
            email: None,
            total_tickets: event_definations.total_tickets,
            total_tickets_sold: 0,
            ticket_price: event_definations.ticket_price,
            is_mint_enabled: false,
        }
    }
}
