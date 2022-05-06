echo "-----------------------CONRTACT INITIALIZING....-----------------------"
# set the owner for contract
near call $NEAR_CONTRACT_ID_2 new_default_meta '{"owner_id": "'$NEAR_ID'", "name":"Web3 Hack", "description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3"}' --accountId $NEAR_ID --amount 1
near call $NEAR_CONTRACT_ID_3 new_default_meta '{"owner_id": "'$NEAR_ID'", "name":"ChainLink Hack", "description":"Scalability – transactions are slower on web3 because theyre decentralized. Changes to state, like a payment, need to be processed by a miner and propagated throughout the network.", "symbol":"CH"}' --accountId $NEAR_ID --amount 1
near call $NEAR_CONTRACT_ID_4 new_default_meta '{"owner_id": "'$NEAR_ID'", "name":"WAGMeet Hack", "description":"No single point of failure: network can still function even if a large proportion of participants are attacked/taken out.", "symbol":"CH"}' --accountId $NEAR_ID --amount 1




echo "-----------------------CONRTACT METADATA-----------------------"
# view metadata
near view $NEAR_CONTRACT_ID_2 nft_metadata
near view $NEAR_CONTRACT_ID_3 nft_metadata
near view $NEAR_CONTRACT_ID_4 nft_metadata


#to enable and disable minting
near call $NEAR_CONTRACT_ID_2 toggle_mint --accountId $NEAR_ID
near call $NEAR_CONTRACT_ID_3 toggle_mint --accountId $NEAR_ID
near call  he888.hey-wagmeet.testnet toggle_mint --accountId wag2.testnet




echo "-----------------------NFT MINTING....-----------------------"
# mint nft
near call $NEAR_CONTRACT_ID_2 nft_mint '{"token_id": "2", "receiver_id": "'$NEAR_ID'", "metadata": { "media": "https://ipfs.io/ipfs/QmbTtnMWsGHU2v57K3Uh8j4xbttm2sszVuzZTZHE3vyvZf", "copies": 1}}' --accountId $NEAR_ID --deposit 0.1
near call $NEAR_CONTRACT_ID_2 nft_mint '{"token_id": "3", "receiver_id": "'$NEAR_ID'", "metadata": { "media": "https://ipfs.io/ipfs/QmTV2buqsS9zmUDChuVXgRrqoLR1MNMZfsBW5mkEsmfH5i", "copies": 1}}' --accountId $NEAR_ID --deposit 0.1


near call $NEAR_CONTRACT_ID_3 nft_mint '{"token_id": "4", "receiver_id": "'$NEAR_ID'", "metadata": { "media": "https://ipfs.io/ipfs/QmYLpqy83PmtN76UR8V9MfJ5qRXqF3dFhMYB4yxDGaGtTT", "copies": 1}}' --accountId $NEAR_ID --deposit 0.1
near call $NEAR_CONTRACT_ID_3 nft_mint '{"token_id": "5", "receiver_id": "'$NEAR_ID'", "metadata": { "media": "https://ipfs.io/ipfs/QmTV2buqsS9zmUDChuVXgRrqoLR1MNMZfsBW5mkEsmfH5i", "copies": 1}}' --accountId $NEAR_ID --deposit 0.1


near call $NEAR_CONTRACT_ID_4 nft_mint '{"token_id": "6", "receiver_id": "'$NEAR_ID'", "metadata": { "media": "https://ipfs.io/ipfs/QmbTtnMWsGHU2v57K3Uh8j4xbttm2sszVuzZTZHE3vyvZf", "copies": 1}}' --accountId $NEAR_ID --deposit 0.1


echo "-----------------------NFTs By Owner-----------------------"
near view $NEAR_CONTRACT_ID_2 nft_tokens_for_owner '{"account_id": "'$NEAR_ID'"}'
near view $NEAR_CONTRACT_ID_4 nft_tokens_for_owner '{"account_id": "'$NEAR_ID'"}'


echo "---------------------------NFT - Total Supply----------------------"
near view $NEAR_CONTRACT_ID_2 nft_total_supply
near view $NEAR_CONTRACT_ID_3 nft_total_supply
near view $NEAR_CONTRACT_ID_4 nft_total_supply

echo "----------------TOKEN DATA----------------"
near view $NEAR_CONTRACT_ID_2 nft_token '{"token_id": "2"}'
near view $NEAR_CONTRACT_ID_3 nft_token '{"token_id": "4"}'
near view $NEAR_CONTRACT_ID_4 nft_token '{"token_id": "6"}'

