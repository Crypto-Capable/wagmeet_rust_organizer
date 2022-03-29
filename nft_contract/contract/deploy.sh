# delete if sub accounts exists
near delete my-nft-2.wagm.testnet $NEAR_ID
near delete my-nft-3.wagm.testnet $NEAR_ID
near delete my-nft-4.wagm.testnet $NEAR_ID

#creates sample 3 accounts for 3 events
near create-account my-nft-2.wagm.testnet --masterAccount $NEAR_ID --initialBalance 50
near create-account my-nft-3.wagm.testnet --masterAccount $NEAR_ID --initialBalance 50
near create-account my-nft-4.wagm.testnet --masterAccount $NEAR_ID --initialBalance 50


#export 3 contracts
export NEAR_CONTRACT_ID_2=my-nft-2.wagm.testnet
export NEAR_CONTRACT_ID_3=my-nft-3.wagm.testnet
export NEAR_CONTRACT_ID_4=my-nft-4.wagm.testnet

#deploy nft contract on each account
near deploy --wasmFile target/wasm32-unknown-unknown/release/nft_simple.wasm --accountId $NEAR_CONTRACT_ID_2
near deploy --wasmFile target/wasm32-unknown-unknown/release/nft_simple.wasm --accountId $NEAR_CONTRACT_ID_3
near deploy --wasmFile target/wasm32-unknown-unknown/release/nft_simple.wasm --accountId $NEAR_CONTRACT_ID_4
