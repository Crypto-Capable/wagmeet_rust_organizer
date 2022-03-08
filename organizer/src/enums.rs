use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey};


use crate::*;

// #[derive(BorshSerialize, BorshStorageKey)]
// enum StorageKey {
//     Event,
//     Host,
// }