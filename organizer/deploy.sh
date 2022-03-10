./build.sh
near delete $ACCOUNTID_TEST $ACCOUNTID
near create-account $ACCOUNTID_TEST --masterAccount $ACCOUNTID --initialBalance 20
near deploy --wasmFile target/wasm32-unknown-unknown/release/organizer.wasm --accountId wagmeet101.meghaha.testnet --initFunction 'new' --initArgs '{}'
# near call wagmeet101.meghaha.testnet add_user '{"owner_id" : "wagmeet101.meghaha.testnet", "metadata" : "{\"name\" : \"Megha Agarwal\" , \"unique_handle\" : \"meghaha\", \"birthday\" : \"22-02-2022\", \"gender\" : \"Female\" , \"wallet_address\" : \"wagmeet101.meghaha.testnet\" }" }' --accountId wagmeet101.meghaha.testnet
near call wagmeet101.meghaha.testnet add_event '{"hostid" : "wagmeet101.meghaha.testnet", "date" : "10-03-2020" , "metadata" : { "name" : "Breeze 2020" , "host" : "", "location" : "Delhi" } }' --accountId wagmeet101.meghaha.testnet
near call wagmeet101.meghaha.testnet add_event '{"hostid" : "wagmeet101.meghaha.testnet", "date" : "10-03-2021" ,"metadata" : { "name" : "Breeze 2021" , "host" : "" , "location" : "Dadri" } }' --accountId wagmeet101.meghaha.testnet
near call wagmeet101.meghaha.testnet all_events '{"hostid" : "wagmeet101.meghaha.testnet" }' --accountId wagmeet101.meghaha.testnet