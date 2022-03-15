./build.sh
near delete wagmeet101.meghaha.testnet meghaha.testnet
near create-account wagmeet101.meghaha.testnet --masterAccount meghaha.testnet --initialBalance 20
near deploy --wasmFile target/wasm32-unknown-unknown/release/organizer.wasm --accountId wagmeet101.meghaha.testnet --initFunction 'new' --initArgs '{}'
# near call wagmeet101.meghaha.testnet add_user '{"owner_id" : "wagmeet101.meghaha.testnet", "metadata" : "{\"name\" : \"Megha Agarwal\" , \"unique_handle\" : \"meghaha\", \"birthday\" : \"22-02-2022\", \"gender\" : \"Female\" , \"wallet_address\" : \"wagmeet101.meghaha.testnet\" }" }' --accountId wagmeet101.meghaha.testnet
near call wagmeet101.meghaha.testnet add_event '{"hostid" : "wagmeet101.meghaha.testnet", "date" : "10-03-2020" , "metadata" : { "name" : "Breeze 2020" , "host" : "", "location" : "Delhi" } }' --accountId wagmeet101.meghaha.testnet
near call wagmeet101.meghaha.testnet add_event '{"hostid" : "meghaha.testnet", "date" : "25-03-2022" ,"metadata" : { "name" : "Breeze 2022" , "host" : "" , "location" : "Dadri" } }' --accountId wagmeet101.meghaha.testnet
near call wagmeet101.meghaha.testnet all_events --accountId wagmeet101.meghaha.testnet
near call wagmeet101.meghaha.testnet mint_nft '{"contract_a" : "wagmeet101.meghaha.testnet", "contract_b" : "nft.meghaha.testnet" }' --accountId wagmeet101.meghaha.testnet