near delete wagmeet-owner.$NEAR_ID $NEAR_ID
near delete fjldh.wagmeet.testnet wagmeet.testnet
near delete hey-wagmeet.testnet $NEAR_ID



near create-account wagmeet-owner.$NEAR_ID --masterAccount $NEAR_ID --initialBalance 50
near deploy --wasmFile target/wasm32-unknown-unknown/release/organizer.wasm --accountId $CONTRACT --initFunction 'new' --initArgs '{}'
near deploy --wasmFile target/wasm32-unknown-unknown/release/organizer.wasm --accountId 3-wagmmet.testnet --initFunction 'new' --initArgs '{}'



near create-account si-nft.$NEAR_ID --masterAccount $NEAR_ID --initialBalance 100

near delete si-nft.$NEAR_ID $NEAR_ID
