use near_sdk::ext_contract;
use near_sdk::{
    near_bindgen, AccountId, Promise, Balance, Gas,
    serde_json::{json}
};

use crate::*;

const GAS: Gas = 10_000_000_000_000;
const NO_DEPOSIT: Balance = 0;

pub type TokenId = String;

//defines the payout type we'll be returning as a part of the royalty standards.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
} 

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
    pub spec: String,              // required, essentially a version like "nft-1.0.0"
    pub name: String,              // required, ex. "Mosaics"
    pub symbol: String,            // required, ex. "MOSIAC"
    
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    //owner of the token
    pub owner_id: AccountId,
}

//The Json token is what will be returned from view calls. 
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    //token ID
    pub token_id: TokenId,
    //owner of the token
    pub owner_id: AccountId,
    //token metadata
    pub metadata: TokenMetadata,
}

pub trait NonFungibleTokenMetadata {
    //view call for returning the contract metadata
    fn nft_metadata(&self) -> NFTContractMetadata;
    fn nft_metadata_call(&self, account_id: AccountId) -> Promise;
}

#[near_bindgen]
impl NonFungibleTokenMetadata for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }

    fn nft_metadata_call(&self, account_id: AccountId) -> Promise {
        Promise:: new(account_id).function_call(
            b"nft_metadata".to_vec(),
            "{}".to_string().as_bytes().to_vec(),
            NO_DEPOSIT,
            GAS,
        )  
    }
}