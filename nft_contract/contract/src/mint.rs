use crate::*;
use near_sdk::log;
use std::convert::{TryFrom, TryInto};

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(&mut self, token_id: TokenId, metadata: TokenMetadata, receiver_id: AccountId) {

        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        // Check if minting is enabled on event or not. If not, throw error.
        // let is_mint_enabled : bool = self.is_mint_enabled;
        // log!("mint value :{}", is_mint_enabled);
        // assert!(
        //     is_mint_enabled,
        //     "Buying is not enabled on this event, Try after some time."
        // );

        let is_tickets_available : bool = self.total_tickets > self.token_metadata_by_id.len();
        assert!(is_tickets_available, "All tickets SOLD OUT.");

        //specify the token struct that contains the owner ID
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
        };


        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        #[allow(unused_doc_comments)]
        /**
         * Instead of passing of title and description for the event everytime, we only set title and des
         * at the time of contract initialization and those values will be fetched and used for every nft.
         * 
         * Title is displayed on collectibles tab under each NFT. Title is the combination of text "Ticket" and the current supply.
         * This is subjected to change.
         */
        let total = self.token_metadata_by_id.len() + 1; // if no nfts minted len will be 0, so increment total supply by one.
        let title: String = format!("{}-{:?}", "Ticket", total);
        let description: String = self.get_desc();

        let mut data = metadata;
        data.title = Some(title);
        data.description = Some(description);

        // v2 -> for marking attendence
        // data.is_attended = Some(false);

        // ----------checking price : START-----------//
        // let attached_deposit = env::attached_deposit();
        // let price = self.get_price();

        // let is_price_matched : bool = attached_deposit > price;
        // assert!(
        //     is_price_matched,
        //     "Attach more NEAR than the price to cover gas costs. (Unsed gas will be refunded)"
        // );
        // Promise::new(env::current_account_id()).transfer(price);
        // ----------checking price : END-----------//

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &data);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
    }
}
