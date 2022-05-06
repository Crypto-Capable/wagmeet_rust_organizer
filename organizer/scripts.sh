# near call $CONTRACT add_event '{"metadata" : { "name" : "1WEB30HACK","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi", "total_tickets":100 } }' --accountId $NEAR_ID --gas=75000000000000
# near call $CONTRACT add_event '{"metadata" : { "name" : "2DAOHACK","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi", "total_tickets":50 } }' --accountId $NEAR_ID --gas=75000000000000
# near call $CONTRACT add_event '{"metadata" : { "name" : "3NFTHACK","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi", "total_tickets":10 } }' --accountId $NEAR_ID --gas=75000000000000

# near call $CONTRACT add_event '{"metadata" : { "name" : "4CODEME","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi", "total_tickets":5 } }' --accountId $NEAR_ID --gas=75000000000000
# near call $CONTRACT add_event '{"metadata" : { "name" : "5DEFIHACK","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi", "total_tickets":5 } }' --accountId $NEAR_ID --gas=75000000000000
# near call $CONTRACT add_event '{"metadata" : { "name" : "6WAGMI","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi", "total_tickets":5 } }' --accountId $NEAR_ID --gas=75000000000000
# near call $CONTRACT add_event '{"metadata" : { "name" : "7HACKMETWICE","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi", "total_tickets":5 } }' --accountId $NEAR_ID --gas=75000000000000
# near call $CONTRACT add_event '{"metadata" : { "name" : "8GOODCODE","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi", "total_tickets":500 } }' --accountId $NEAR_ID --gas=75000000000000

near call $CONTRACT add_event '{"metadata" : { "name" : "3HELLOWORLD","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi", "total_tickets":10 } }' --accountId $NEAR_ID --gas=75000000000000




# near call wagmeet-owner.$NEAR_ID nft_mint '{"token_id": "2","event_contract":"super.wagmeet-owner.test-nea2.testnet", "media":"https://ipfs.io/ipfs/QmW11eYCCmcWL6zxeVLvRYXmLr6Hbu2Bz45fshvLPyTKk2"}' --accountId $NEAR_ID --amount 3 --gas=300000000000000


# near call dakjh.wagmeet-owner.test-nea2.testnet nft_mint '{"token_id": "3", "receiver_id": "'$NEAR_ID'", "metadata": { "media": "https://ipfs.io/ipfs/QmW11eYCCmcWL6zxeVLvRYXmLr6Hbu2Bz45fshvLPyTKk2", "copies": 1}}' --accountId $NEAR_ID --deposit 0.1

near call $CONTRACT all_events  --accountId $NEAR_ID
near call 3hell.hey-wagmeet.testnet toggle_mint  --accountId $NEAR_ID

near call $CONTRACT all_hosts  --accountId $NEAR_ID
near call $CONTRACT get_event_by_id '{"event_id" : "3hell.hey-wagmeet.testnet", "hostid" : "wag2.testnet"}'  --accountId $NEAR_ID
near call $CONTRACT get_event_by_id '{"event_id" : "8good.hey-wagmeet.testnet", "hostid" : "krishna191.testnet"}'  --accountId $NEAR_ID

near call $CONTRACT all_events_by_id '{"hostid" : "wag2.testnet"}'  --accountId $NEAR_ID


# near call wagmeet.testnet all_events  --accountId $NEAR_ID

# near call wagmeet-owner.$NEAR_ID delete_event_account '{"account":"fdslk.wagmeet.testnet"}' --accountId $NEAR_ID
# near call wagmeet.testnet delete_event_account '{"account":"fjldh.wagmeet.testnet"}' --accountId $NEAR_ID


# near view he888.hey-wagmeet.testnet nft_metadata
# near view he888.hey-wagmeet.testnet get_mint_enabled()
# near view dakjh.wagmeet-owner.test-nea2.testnet nft_tokens
# near view hacki.wagmeet-owner.test-nea2.testnet nft_total_supply
# near view hacki.wagmeet-owner.test-nea2.testnet nft_token '{"token_id": "2"}'
# near view hacki.wagmeet-owner.test-nea2.testnet nft_tokens_for_owner '{"account_id": "'$NEAR_ID'"}'