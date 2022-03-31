near delete wagmeet-owner.$NEAR_ID $NEAR_ID

near create-account wagmeet-owner.$NEAR_ID --masterAccount $NEAR_ID --initialBalance 100
near deploy --wasmFile target/wasm32-unknown-unknown/release/organizer.wasm --accountId $WAGMEET_CONTRACT --initFunction 'new' --initArgs '{}'


near create-account si-nft.$NEAR_ID --masterAccount $NEAR_ID --initialBalance 100

near delete si-nft.$NEAR_ID $NEAR_ID
