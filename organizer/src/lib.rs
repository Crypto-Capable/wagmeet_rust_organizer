use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, LookupSet};
use near_sdk::collections::UnorderedSet;
use near_sdk::{log, PublicKey};
use near_sdk::{
    env, near_bindgen, serde_json::json, AccountId, Balance, BorshStorageKey, Gas, Promise, assert_one_yocto
};
use constants::{
    storage_stake,
    storage_bytes
};
use near_sdk::json_types::{U128};

const CODE: &[u8] = include_bytes!(
    "../../nft_contract/contract/target/wasm32-unknown-unknown/release/nft_simple.wasm"
);

mod structs;
mod test;

use crate::structs::Event;
pub use structs::*;
pub mod constants;

const NO_DEPOSIT: Balance = 0;
/// Current price for one byte of on-chain storage, denominated in yoctoNEAR.
pub const YOCTO_PER_BYTE: Balance = 10_000_000_000_000_000_000;
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
    /// The Near-denominated price-per-byte of storage. As of April 2021, the
    /// price per bytes is set by default to 10^19, but this may change in the
    /// future, thus this future-proofing field.
    pub storage_price_per_byte: u128,
    /// Cost in yoctoNear to deploy a store. Changes if `storage_price_per_byte`
    /// changes.
    pub store_cost: u128,
    pub admin_public_key: PublicKey,
    pub event_stores: LookupSet<String>,
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
        assert_one_yocto();
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only contract owner can call this method"
        );
    }

    /// Sufficient attached deposit is defined as enough to deploy a `Store`,
    /// plus enough left over for the mintbase deployment cost.
    /// 
    /// if in future, if there is wagmeet fee add wagmeet fee to this. -> +self.wagmeet_fee
    pub fn assert_sufficient_attached_deposit(&self) {
        let min = storage_bytes::STORE as u128 * self.storage_price_per_byte;
        assert!(
            env::attached_deposit() >= min,
            "Not enough attached deposit to complete store deployment. Need: {}, got: {}",
            min,
            env::attached_deposit()
        );
    }

    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let storage_price_per_byte = YOCTO_PER_BYTE; // 10^19
        Self {
            event_list: UnorderedMap::<AccountId, UnorderedSet<Event>>::new(env::sha256(
                env::predecessor_account_id().as_bytes(),
            )),
            host_list: UnorderedSet::new(b"s".to_vec()),
            event_stores: LookupSet::new(b"t".to_vec()),
            owner_id: env::predecessor_account_id(),
            storage_price_per_byte,
            store_cost: storage_stake::STORE,
            admin_public_key: env::signer_account_pk(),
        }
    }

    #[payable]
    pub fn add_event(&mut self, metadata: serde_json::Value) -> Promise {
        let event_definations: Event = serde_json::from_str(&metadata.to_string()).unwrap();
        let event_account = event_definations.event_address.to_string();

        self.assert_sufficient_attached_deposit();
        self.assert_no_store_with_id(event_account.clone());

        // log!("Store are : {:?}", self.event_list);
        #[allow(unused_doc_comments)]

        /**
         * STEPS :
         * -> adds host to 'host_list' if host is not already there in the list
         * -> get the 'event_list' for the host_id :
         *      -> if 'event_list' is already created for the host, insert the event
         *      -> if not, then create a empty 'event_list' for the host.
         * -> create a sub account for the event, transfer initial balance to cover storage costs,
         *    deploy contract on created sub account, initialize contract.
         */
        // get signer account
        let signer_id = env::signer_account_id();
        let event = Event::create_event(env::signer_account_id(), &metadata);
        let event_definations: Event = serde_json::from_str(&metadata.to_string()).unwrap();
        // create a sub account name using event name and contract ID.
        let event_account = event_definations.event_address.to_string();

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
            .then(Promise::new(self.event_stores.insert(&event_account.to_string()).to_string()))
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

    pub fn assert_no_store_with_id(
        &self,
        store_id: String,
    ) {
        assert!(
            !self.check_contains_store(store_id),
            "Store with that ID already exists"
        );
    }

    /// If a `Store` with `store_id` has been produced by this `Factory`, return `true`.
    pub fn check_contains_store(
        &self,
        store_id: String,
    ) -> bool {
        self.event_stores.contains(&store_id)
    }

    /// Get the `owner_id` of this `Factory`.
    pub fn get_owner(&self) -> &AccountId {
        &self.owner_id
    }

    /// The sum of `mintbase_fee` and `STORE_STORAGE`.
    pub fn get_admin_public_key(&self) -> &PublicKey {
        &self.admin_public_key
    }

    /// The `STORE_STORAGE`.
    /// if in future, if there is wagmeet fee add wagmeet fee to this.-> +self.wagmeet_fee
    pub fn get_minimum_attached_balance(&self) -> U128 {
        (storage_bytes::STORE as u128 * self.storage_price_per_byte).into()
    }

    /// The Near Storage price per byte has changed in the past, and may change in
    /// the future. This method may never be used.
    #[payable]
    pub fn set_storage_price_per_byte(
        &mut self,
        new_price: U128,
    ) {
        self.assert_only_owner();
        self.storage_price_per_byte = new_price.into();
        self.store_cost = self.storage_price_per_byte * storage_bytes::STORE as u128;
    }
}
