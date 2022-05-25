use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::collections::UnorderedSet;
use near_sdk::log;
use near_sdk::{
    env, near_bindgen, serde_json::json, AccountId, Balance, BorshStorageKey, Gas, Promise,
};

const CODE: &[u8] = include_bytes!(
    "../../nft_contract/contract/target/wasm32-unknown-unknown/release/nft_simple.wasm"
);

mod structs;
mod test;

use crate::structs::Event;
pub use structs::*;

const NO_DEPOSIT: Balance = 0;
pub const XCC_GAS: Gas = 20000000000000;
const INITIAL_BALANCE: Balance = 4_500_000_000_000_000_000_000_000;

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
    pub owner_id: AccountId,
    // contract_public_key: PublicKey
}

impl Default for Contract {
    fn default() -> Self {
        // Check incase the contract is not initialized
        env::panic(b"The contract is not initialized.")
    }
}

#[near_bindgen]
impl Contract {
    // modifier to check whether actor calling is owner or not.
    pub fn assert_only_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only contract owner can call this method"
        );
    }

    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            // TODO: check prefix passed to new() is correct or not ?.... check starting commits to see past value
            event_list: UnorderedMap::<AccountId, UnorderedSet<Event>>::new(env::sha256(
                env::predecessor_account_id().as_bytes(),
            )),
            host_list: UnorderedSet::new(b"s".to_vec()),
            owner_id: env::predecessor_account_id(),
            // contract_public_key : env::signer_account_pk()
        }
    }

    pub fn add_event(&mut self, metadata: serde_json::Value) -> Promise {
        #[allow(unused_doc_comments)]
        /**
         * STEPS :
         * -> adds an event to 'event_list'
         * -> adds host to 'host_list'
         * -> creates a sub account for the event, transfer initial balance to cover storage costs,
         *    deploy contract on created sub account, initialize contract.
         */
        // get contract account
        let signer_id = env::signer_account_id();
        let event = Event::create_event(env::signer_account_id(), &metadata);
        let event_definations: Event = serde_json::from_str(&metadata.to_string()).unwrap();

        // event.set_date(date);
        if !(self.host_list.contains(&signer_id)) {
            self.host_list.insert(&signer_id);
        }

        match self.event_list.get(&signer_id) {
            Some(mut set_test) => {
                set_test.insert(&event);
                self.event_list.insert(&signer_id, &set_test);
            }
            None => {
                let _set: UnorderedSet<Event> = UnorderedSet::new(b"w".to_vec());
                self.event_list.insert(&signer_id, &_set);
            }
        }

        // create a sub account name using event name and contract ID.
        let event_account = event_definations.event_address.to_string();

        // check is sub-account name is valid or not
        assert!(
            env::is_valid_account_id(event_account.as_bytes()),
            "Invalid character in sub account"
        );

        // function name to be called to initialize NFT-EVENT-Contract
        let fn_name = b"new_default_meta".to_vec();

        // FIXME: Check below functionality - fix it
        // event.set_event_address(event_account.to_string());
        Promise::new(event_account.to_string())
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(INITIAL_BALANCE)
            .deploy_contract(CODE.to_vec())
            .then(Promise::new(event_account.to_string()).function_call(
                fn_name,
                json!({ "owner_id": env::predecessor_account_id(), "name" : event_definations.name.to_string(), "symbol" : event_definations.symbol.to_string(), "description" : event_definations.description.to_string(), "total_tickets":event_definations.total_tickets  })
                    .to_string()
                    .as_bytes()
                    .to_vec(),
                NO_DEPOSIT,
                XCC_GAS,
            ))
    }

    pub fn delete_event_account(&mut self, account: AccountId) -> Promise {
        Promise::new(account.to_string()).delete_account(env::current_account_id())
    }

    pub fn all_events_by_id(&self, hostid: AccountId) -> Vec<structs::Event> {
        if let Some(value) = self.event_list.get(&hostid) {
            //if events found
            value.to_vec()
        } else {
            // if no events found on passed hostis, return empty array
            let events: Vec<Event> = Vec::new();
            events
        }
    }

    pub fn get_event_by_id(&self, event_id: AccountId, hostid: AccountId) -> Event {
        let events = self.event_list.get(&hostid).unwrap().to_vec();
        log!("events : {:?}", events);
        let mut event: Event = Event::new();
        log!("Event : {:?}", event);
        for i in events {
            if i.event_address == event_id.to_string() {
                event = i;
            }
        }
        event
    }

    pub fn all_events(&self) -> Vec<Event> {
        let _values = self.event_list.keys_as_vector();
        let v1_iter = _values.iter();
        let mut ans = Vec::new();
        for i in v1_iter {
            let mut vec2 = self.event_list.get(&i).unwrap().to_vec();
            ans.append(&mut vec2);
        }
        ans
    }

    pub fn all_hosts(&self) -> Vec<AccountId> {
        self.host_list.to_vec()
    }

    pub fn delete_event(&mut self, metadata: serde_json::Value) -> bool {
        let hostid = env::signer_account_id();
        let mut events = self.event_list.get(&hostid).unwrap();
        let event: Event = Event::get_event(&metadata);
        let is_owner = hostid == event.host;
        assert!(events.contains(&event), "Event Not Found");
        assert!(is_owner, "Only event owner can delete the event.");
        events.remove(&event);
        self.event_list.insert(&hostid, &events);
        true
    }
}
