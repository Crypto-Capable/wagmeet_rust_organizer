use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::collections::UnorderedMap;
use near_sdk::collections::UnorderedSet;
use near_sdk::log;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, serde_json::json, AccountId, Balance, BorshStorageKey, Gas, Promise,
};
const CODE: &[u8] =
    include_bytes!("../../nft_contract/target/wasm32-unknown-unknown/release/nft_contract.wasm");

mod structs;
mod traits;

use crate::structs::Event;
pub use structs::*;
pub use traits::*;

const NO_DEPOSIT: Balance = 0;
const MINT_PRICE: Balance = 3_000_000_000_000_000_000_000_000;
const GAS: Gas = 100_000_000_000_000;
pub const XCC_GAS: Gas = 20000000000000;
const INITIAL_BALANCE: Balance = 50_000_000_000_000_000_000_000_000; //

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
        hostid: AccountId,
        metadata: serde_json::Value,
        date: String,
    ) -> String {
        let mut event = Event::create_event(hostid.clone(), metadata);
        event.set_date(date);
        if !(self.host_list.contains(&hostid)) {
            self.host_list.insert(&hostid);
            let _set: UnorderedSet<Event> = UnorderedSet::new(b"w".to_vec());
            self.event_list.insert(&hostid, &_set);
        }

        let mut set_test = self.event_list.get(&hostid).unwrap();
        set_test.insert(&event);
        self.event_list.insert(&hostid, &set_test);
        // let subaccount_name = format!("{}.{}", event.get_name(), hostid.clone());
        let subaccount_name = "event8.wagmeetowner.test-nea.testnet";        
        Promise::new(subaccount_name.to_string())
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(INITIAL_BALANCE)
            .deploy_contract(CODE.to_vec());
        // self.event_list.get(&hostid).insert(&event);
        subaccount_name.to_string()
    }

    // #[private]
    // pub fn create_subaccount(subaccount: String) -> Promise {
    //     // let subaccount_id = AccountId::new_unchecked(
    //     //   format!("{}.{}", prefix, env::current_account_id())
    //     // );
    //     Promise::new(subaccount_name.to_string())
    //         .create_account()
    //         .add_full_access_key(env::signer_account_pk())
    //         .transfer(INITIAL_BALANCE)
    // }

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

    pub fn contract_initialize(contract_a: AccountId, contract_b: AccountId) -> Promise {
        // let fun_name = "nft_metadata_call".to_string();
        let fn_name = b"new_default_meta".to_vec();
        Promise::new(contract_b.clone()).function_call(
            fn_name,
            json!({ "owner_id": contract_b })
                .to_string()
                .as_bytes()
                .to_vec(),
            NO_DEPOSIT,
            XCC_GAS,
        )
    }

    #[payable]
    pub fn nft_mint(contract_a: AccountId, contract_b: AccountId) -> Promise {
        // let fun_name = "nft_mint".to_string();
        let fn_name = b"nft_mint".to_vec();
        Promise:: new(contract_b.clone()).function_call(
            fn_name,
            json!({ "token_id": "nft_token101","metadata" : { "title" : "First NFT" , "description" : "This is the first minted NFT" }, "receiver_id" : contract_a}).to_string().as_bytes().to_vec(),
            MINT_PRICE,
            GAS,
        )
    }
}
