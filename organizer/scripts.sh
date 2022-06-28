# near call $CONTRACT add_event '{"metadata" : { "name" : "1WEB30HACK","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi", "total_tickets":100 } }' --accountId $NEAR_ID --gas=75000000000000
# near call $CONTRACT add_event '{"metadata" : { "name" : "2DAOHACK","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi", "total_tickets":50 } }' --accountId $NEAR_ID --gas=75000000000000
# near call $CONTRACT add_event '{"metadata" : { "name" : "3NFTHACK","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi", "total_tickets":10 } }' --accountId $NEAR_ID --gas=75000000000000
# near call 3-wagmmet.testnet add_event '{"metadata" : { "name" : "MEHACK","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi", "total_tickets":10,  "is_deleted":false } }' --accountId krishna191.testnet --gas=75000000000000
# near call 3-wagmmet.testnet add_event '{"metadata" : { "name" : "DAOHACK","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi", "total_tickets":10, "is_deleted":false } }' --accountId wag2.testnet --gas=75000000000000


# near call $CONTRACT add_event '{"metadata" : { "name" : "4CODEME","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi", "total_tickets":5 } }' --accountId $NEAR_ID --gas=75000000000000
# near call $CONTRACT add_event '{"metadata" : { "name" : "5DEFIHACK","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi", "total_tickets":5 } }' --accountId $NEAR_ID --gas=75000000000000
# near call $CONTRACT add_event '{"metadata" : { "name" : "6WAGMI","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi", "total_tickets":5 } }' --accountId $NEAR_ID --gas=75000000000000
# near call $CONTRACT add_event '{"metadata" : { "name" : "7HACKMETWICE","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi", "total_tickets":5 } }' --accountId $NEAR_ID --gas=75000000000000
# near call $CONTRACT add_event '{"metadata" : { "name" : "WAGMEE","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"","location" : "Delhi", "total_tickets":500 } }' --accountId $NEAR_ID --gas=75000000000000

near call v1-wagmeet.testnet add_event '{"metadata" : { "name" : "Web3 Hack","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"","event_address":"13wsd.v1-wagmeet.testnet","location" : "Delhi", "total_tickets":10, "is_deleted":false } }' --accountId mint-wag.testnet --amount 6 --gas=75000000000000




# near call wagmeet-owner.$NEAR_ID nft_mint '{"token_id": "2","event_contract":"super.wagmeet-owner.test-nea2.testnet", "media":"https://ipfs.io/ipfs/QmW11eYCCmcWL6zxeVLvRYXmLr6Hbu2Bz45fshvLPyTKk2"}' --accountId $NEAR_ID --amount 3 --gas=300000000000000


near call 13wsd.v1-wagmeet.testnet nft_mint '{"token_id": "3", "receiver_id": "1sd.mint-wag.testnet", "metadata": { "media": "https://ipfs.io/ipfs/QmW11eYCCmcWL6zxeVLvRYXmLr6Hbu2Bz45fshvLPyTKk2", "copies": 1}}' --accountId mint-wag.testnet --deposit 0.1

near call $CONTRACT all_events  --accountId mint-wag.testnet
near call 1sd.mint-wag.testnet toggle_mint  --accountId mint-wag.testnet

near call $CONTRACT all_hosts  --accountId mint-wag.testnet
near call $CONTRACT get_event_by_id '{"event_id" : "3hell.hey-wagmeet.testnet", "hostid" : "wag2.testnet"}'  --accountId $NEAR_ID
near call $CONTRACT get_event_by_id '{"event_id" : "8good.hey-wagmeet.testnet", "hostid" : "krishna191.testnet"}'  --accountId $NEAR_ID

near call new1-wagmeet.testnet all_events_by_id '{"hostid" : "latest-events.testnet"}'  --accountId latest-events.testnet
near call 3-wagmmet.testnet all_events_by_id '{"hostid" : "krishna191.testnet"}'  --accountId krishna191.testnet



# near call new1-wagmeet.testnet all_events  --accountId latest-events.testnet
# near call latest-wagmeet.testnet all_events  --accountId wag2.testnet
# near call new1-wagmeet.testnet delete_event '{"metadata" : { "name" : "Hello","description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3", "host":"wagt1.testnet","event_address":"123wsd.new1-wagmeet.testnet","location" : "Delhi", "total_tickets":10,  "is_deleted":false } }' --accountId wagt1.testnet


# near call wagmeet-owner.$NEAR_ID delete_event_account '{"account":"fdslk.wagmeet.testnet"}' --accountId $NEAR_ID
# near call wagmeet.testnet delete_event_account '{"account":"fjldh.wagmeet.testnet"}' --accountId $NEAR_ID


# near view he888.hey-wagmeet.testnet nft_metadata
# near view he888.hey-wagmeet.testnet get_mint_enabled()
# near view dakjh.wagmeet-owner.test-nea2.testnet nft_tokens
# near view hacki.wagmeet-owner.test-nea2.testnet nft_total_supply
# near view hacki.wagmeet-owner.test-nea2.testnet nft_token '{"token_id": "2"}'
# near view hacki.wagmeet-owner.test-nea2.testnet nft_tokens_for_owner '{"account_id": "'$NEAR_ID'"}'