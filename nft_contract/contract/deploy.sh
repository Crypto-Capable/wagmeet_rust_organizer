# login and export logged in account as NEAR_ID
# delete if sub accounts exists
near delete my-nft-2.$NEAR_ID $NEAR_ID
near delete my-nft-3.$NEAR_ID $NEAR_ID
near delete my-nft-4.$NEAR_ID $NEAR_ID

#creates sample 3 accounts for 3 events
near create-account my-nft-2.$NEAR_ID --masterAccount $NEAR_ID --initialBalance 50
near create-account my-nft-3.$NEAR_ID --masterAccount $NEAR_ID --initialBalance 50
near create-account my-nft-4.$NEAR_ID --masterAccount $NEAR_ID --initialBalance 50

#export 3 accounts

#deploy nft contract on each account
near deploy --wasmFile target/wasm32-unknown-unknown/release/nft_simple.wasm --accountId $NEAR_CONTRACT_ID_2
near deploy --wasmFile target/wasm32-unknown-unknown/release/nft_simple.wasm --accountId $NEAR_CONTRACT_ID_3
near deploy --wasmFile target/wasm32-unknown-unknown/release/nft_simple.wasm --accountId $NEAR_CONTRACT_ID_4
