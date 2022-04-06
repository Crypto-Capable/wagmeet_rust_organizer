use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::collections::UnorderedMap;
use near_sdk::collections::UnorderedSet;
use near_sdk::log;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, serde_json::json, AccountId, Balance, BorshStorageKey, Gas, Promise,
};
const CODE: &[u8] = include_bytes!(
    "../../nft_contract/contract/target/wasm32-unknown-unknown/release/nft_simple.wasm"
);

mod structs;
mod traits;

use crate::structs::Event;
pub use structs::*;
pub use traits::*;

const NO_DEPOSIT: Balance = 0;
const MINT_PRICE: Balance = 0;
const GAS: Gas = 100_000_000_000_000;
pub const XCC_GAS: Gas = 20000000000000;
const INITIAL_BALANCE: Balance = 2_500_000_000_000_000_000_000_000; //

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

    pub fn add_event(
        &mut self,
        metadata: serde_json::Value,
    ) -> Promise {
        #[allow(unused_doc_comments)]
        /**
         * STEPS :
         * -> adds an event to 'event_list'
         * -> adds host to 'host_list'
         * -> creates a sub account for the event, transfer initial balance to cover storage costs,
         *    deploy contract on created sub account, initialize contract.
         */
        // get contract account
        let id = env::current_account_id();
        let mut event = Event::create_event(env::predecessor_account_id(), &metadata);
        let event_definations: Event = serde_json::from_str(&metadata.to_string()).unwrap();

        // event.set_date(date);
        if !(self.host_list.contains(&id)) {
            self.host_list.insert(&id);
            let _set: UnorderedSet<Event> = UnorderedSet::new(b"w".to_vec());
            self.event_list.insert(&id, &_set);
        }
        let mut set_test = self.event_list.get(&id).unwrap();
        set_test.insert(&event);
        self.event_list.insert(&id, &set_test);

        // create a sub account name using event name and contract ID.
        let event_name = String::from(event.get_name());
        let subaccount_name = format!("{}.{}", &event_name[0..5], id.clone());
        let event_account = &subaccount_name.to_lowercase().trim().to_string();

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
                json!({ "owner_id": env::predecessor_account_id(), "name" : event_definations.name.to_string(), "symbol" : event_definations.symbol.to_string(), "description" : event_definations.description.to_string()  })
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

    pub fn all_events_by_id(&mut self, hostid: AccountId) -> Vec<structs::Event> {
        self.event_list.get(&hostid).unwrap().to_vec()
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
}