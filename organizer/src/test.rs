use crate::Contract;
use near_sdk::{testing_env, VMContext};
use near_sdk::MockedBlockchain;
use serde_json::Map;
use serde_json::Value;
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, EpochHeight, Promise, PromiseResult,
    PublicKey, 
};

 
fn alice() -> AccountId {
    "alice.near".to_string()
}
fn carol() -> AccountId {
    "carol.near".to_string()
}
fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
    VMContext {
        current_account_id: "alice_near".to_string(),
        signer_account_id: "bob_near".to_string(),
        signer_account_pk: vec![0, 1, 2],
        predecessor_account_id: "carol_near".to_string(),
        input,
        block_index: 0,
        block_timestamp: 0,
        account_balance: 0,
        account_locked_balance: 0,
        storage_usage: 0,
        attached_deposit: 0,
        prepaid_gas: 10u64.pow(18),
        random_seed: vec![0, 1, 2],
        is_view,
        output_data_receivers: vec![],
        epoch_height: 0,
    }
}
 #[test]
fn test_get_initialized(){
    let context =get_context(vec![], false);
    testing_env!(context);
    let contract = Contract::default();
    contract.new();
    assert_eq!(predecessor_account_id,contract.owner_id);
}
#[test]
fn test_all_events_id(){
    let context =get_context(vec![], false);
    testing_env!(context);
    let contract = Contract::default();
    let event = contract.get_event_by_id(current_account_id);
    assert_eq!("Breeze",event.name);
}
#[test]
fn test_all_hosts(){
    let context =get_context(vec![], false);
    testing_env!(context);
    let contract = Contract::default();
    let hosts = contract.all_hosts();
    assert_eq!(current_account_id,hosts[0]);
}
// #[test]
//   fn test_withdraw(){
//     let context =get_context(vec![], false);
//     testing_env!(context);
//     let contract = ColdChain::withdraw();
//     assert_eq!(false,contract.withdraw);
// }
