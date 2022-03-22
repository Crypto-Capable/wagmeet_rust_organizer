./build.sh
near delete nft.meghaha101.testnet meghaha101.testnet
near create-account nft.meghaha101.testnet --masterAccount meghaha101.testnet --initialBalance 20
near deploy --wasmFile target/wasm32-unknown-unknown/release/nft_contract.wasm --accountId nft.meghaha101.testnet --initFunction 'new_default_meta' --initArgs '{"owner_id" : "nft.meghaha101.testnet"}'
near call nft.meghaha101.testnet nft_mint '{ "token_id": "nft_token101","metadata" : { "title" : "First NFT" , "description" : "This is the first minted NFT" }, "receiver_id" : "wagmeet101.meghaha101.testnet"}' --accountId wagmeet101.meghaha101.testnet --gas=6730000000000000000000
6730000000000000000000
75000000000000