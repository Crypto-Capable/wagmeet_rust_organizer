near delete wagmeet-owner.meghaha101.testnet meghaha101.testnet
near create-account wagmeet-owner.meghaha101.testnet --masterAccount meghaha101.testnet --initialBalance 100
near deploy --wasmFile target/wasm32-unknown-unknown/release/organizer.wasm --accountId wagmeet-owner.meghaha101.testnet --initFunction 'new' --initArgs '{}'
near call wagmeet-owner.meghaha101.testnet add_event '{"metadata" : { "name" : "Birthday","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi" }, "total_tickets" : 200 }' --accountId meghaha101.testnet --gas=300000000000000
near call wagmeet-owner.meghaha101.testnet add_event '{"metadata" : { "name" : "1Newspapaer","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi" }, "total_tickets" : 200 }' --accountId meghaha101.testnet --gas=300000000000000
near call wagmeet-owner.meghaha101.testnet all_events --accountId meghaha101.testnet 
near call 1news.wagmeet-owner.meghaha101.testnet nft_mint '{"tokenid" : "Test102","receiver_id": "meghaha101.testnet" }' --accountId wagmeet-owner.meghaha101.testnet --gas=300000000000000 --amount=3 

near call 1news.wagmeet-owner.meghaha101.testnet toggle_mint --accountId wagmeet-owner.meghaha101.testnet 
near call wagmeet-owner.meghaha101.testnet delete_event_account '{"account":"breez.wagmeet-owner.meghaha101.testnet"}' --accountId wagmeet-owner.meghaha101.testnet

near create-account si-nft.meghaha101.testnet --masterAccount meghaha101.testnet --initialBalance 100
near delete si-nft.meghaha101.testnet meghaha101.testnet


near call check.wagmeet-owner.meghaha101.testnet nft_metadata
near view check.wagmeet-owner.meghaha101.testnet nft_total_supply

near view 1news.wagmeet-owner.meghaha101.testnet nft_tokens
near view 1news.wagmeet-owner.meghaha101.testnet nft_total_supply
near view 1news.wagmeet-owner.meghaha101.testnet nft_token '{"token_id": "Test101"}'
near view 1news.wagmeet-owner.meghaha101.testnet nft_tokens_for_owner '{"account_id": "'meghaha101.testnet'"}'