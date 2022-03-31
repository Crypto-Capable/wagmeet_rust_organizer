near call wagmeet-owner.$NEAR_ID add_event '{"metadata" : { "name" : "FSDLKFLSD","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi" } }' --accountId $NEAR_ID --gas=75000000000000
near call wagmeet-owner.$NEAR_ID nft_mint '{"token_id": "2","event_contract":"super.wagmeet-owner.test-nea2.testnet", "media":"https://ipfs.io/ipfs/QmW11eYCCmcWL6zxeVLvRYXmLr6Hbu2Bz45fshvLPyTKk2"}' --accountId $NEAR_ID --amount 3 --gas=300000000000000


near call dakjh.wagmeet-owner.test-nea2.testnet nft_mint '{"token_id": "3", "receiver_id": "'$NEAR_ID'", "metadata": { "media": "https://ipfs.io/ipfs/QmW11eYCCmcWL6zxeVLvRYXmLr6Hbu2Bz45fshvLPyTKk2", "copies": 1}}' --accountId $NEAR_ID --deposit 0.1

near call wagmeet-owner.$NEAR_ID all_events  --accountId $NEAR_ID
near call wagmeet-owner.$NEAR_ID delete_event_account '{"account":"fsdlk.wagmeet-owner.test-nea2.testnet"}' --accountId $NEAR_ID

near view hacki.wagmeet-owner.test-nea2.testnet nft_metadata
near view dakjh.wagmeet-owner.test-nea2.testnet nft_tokens
near view hacki.wagmeet-owner.test-nea2.testnet nft_total_supply
near view hacki.wagmeet-owner.test-nea2.testnet nft_token '{"token_id": "2"}'
near view hacki.wagmeet-owner.test-nea2.testnet nft_tokens_for_owner '{"account_id": "'$NEAR_ID'"}'