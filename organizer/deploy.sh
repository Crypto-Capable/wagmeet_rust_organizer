./build.sh
near delete wagmeet101.meghaha.testnet meghaha.testnet
near create-account wagmeet101.meghaha.testnet --masterAccount meghaha.testnet --initialBalance 20
near deploy --wasmFile target/wasm32-unknown-unknown/release/organizer.wasm --accountId wagmeet101.meghaha.testnet --initFunction 'new' --initArgs '{}'
# near call wagmeet101.meghaha.testnet add_user '{"owner_id" : "wagmeet101.meghaha.testnet", "metadata" : "{\"name\" : \"Megha Agarwal\" , \"unique_handle\" : \"meghaha\", \"birthday\" : \"22-02-2022\", \"gender\" : \"Female\" , \"wallet_address\" : \"wagmeet101.meghaha.testnet\" }" }' --accountId wagmeet101.meghaha.testnet
near call wagmeet101.meghaha.testnet add_host '{"hostid" : "wagmeet101.meghaha.testnet", "metadata" : { "name" : "Megha Agarwal" , "accountid" : ""} }' --accountId wagmeet101.meghaha.testnet
near call wagmeet101.meghaha.testnet add_host '{"hostid" : "meghaha.testnet", "metadata" : { "name" : "Fawad Khan" , "accountid" : ""} }' --accountId wagmeet101.meghaha.testnet
near call wagmeet101.meghaha.testnet all_hosts --accountId wagmeet101.meghaha.testnet