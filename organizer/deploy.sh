./build.sh
near delete wagmeet101.meghaha101.testnet meghaha101.testnet
near create-account wagmeet101.meghaha101.testnet --masterAccount meghaha101.testnet --initialBalance 50
near deploy --wasmFile target/wasm32-unknown-unknown/release/organizer.wasm --accountId wagmeet101.meghaha101.testnet --initFunction 'new' --initArgs '{}'
# near call wagmeet101.meghaha.testnet add_user '{"owner_id" : "wagmeet101.meghaha.testnet", "metadata" : "{\"name\" : \"Megha Agarwal\" , \"unique_handle\" : \"meghaha\", \"birthday\" : \"22-02-2022\", \"gender\" : \"Female\" , \"wallet_address\" : \"wagmeet101.meghaha.testnet\" }" }' --accountId wagmeet101.meghaha.testnet
near call wagmeet101.meghaha101.testnet add_event '{"hostid" : "wagmeet101.meghaha101.testnet", "date" : "10-03-2020" , "metadata" : { "name" : "Breeze_2020" , "host" : "", "location" : "Delhi", "no_tickets" : 200  } }' --accountId wagmeet101.meghaha101.testnet
# near call wagmeet101.meghaha.testnet add_event '{"hostid" : "meghaha101.testnet", "date" : "25-03-2022" ,"metadata" : { "name" : "Breeze 2022" , "host" : "" , "location" : "Dadri" , "no_tickets" : 100 } }' --accountId wagmeet101.meghaha101.testnet --gas=300000000000000
near call wagmeet101.meghaha101.testnet all_events --accountId wagmeet101.meghaha101.testnet
# near call wagmeet101.meghaha.testnet create_subaccount '{"prefix" : "wagmeet101.meghaha.testnet"}' --accountId wagmeet101.meghaha.testnet

# near call wagmeet101.meghaha.testnet mint_nft '{"contract_a" : "wagmeet101.meghaha.testnet", "contract_b" : "nft.meghaha.testnet" }' --accountId wagmeet101.meghaha.testnet
near call wagmeet101.meghaha101.testnet nft_mint '{"contract_a" : "wagmeet101.meghaha101.testnet", "contract_b" : "nft.meghaha101.testnet" }' --accountId wagmeet101.meghaha101.testnet --amount 3 --gas=75000000000000