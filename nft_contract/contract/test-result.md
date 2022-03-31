## Steps :
*   Login
*   create three sub-accounts to deploy 3 contracts. (one contract for each event. This contract keeps tracks all the owners, contract state, mints new nfts etc...). 
*   You can also use ```deploy.sh``` file to create sub accounts and deploy contracts.
*   export contract IDs as NEAR_CONTRACT_ID_2,NEAR_CONTRACT_ID_3,NEAR_CONTRACT_ID_4
*   Run tests.sh -> ```sh test.sh```
*   Below are the tests and thier results.
### -----------------------CONRTACT INITIALIZING....-----------------------

#### Command : `near call $NEAR_CONTRACT_ID_2 new_default_meta '{"owner_id": "'$NEAR_ID'", "name":"Web3 Hack", "description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3"}' --accountId $NEAR_ID --amount 1`

> Scheduling a call: my-nft-2.wagm.testnet.new_default_meta({"owner_id": "wagm.testnet", "name":"Web3 Hack", "description":"Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend", "symbol":"WEB3"}) with attached 1 NEAR

> Doing account.functionCall()
> Transaction Id 2T3biKhjyn8eUtD8W4udcyFWcJdStWDauhQ8h61Wtzaj
> To see the transaction in the transaction explorer, please open this url in your browser
> https://explorer.testnet.near.org/transactions/2T3biKhjyn8eUtD8W4udcyFWcJdStWDauhQ8h61Wtzaj
> ''

#### Command : `near call $NEAR_CONTRACT_ID_3 new_default_meta '{"owner_id": "'$NEAR_ID'", "name":"ChainLink Hack", "description":"Scalability – transactions are slower on web3 because theyre decentralized. Changes to state, like a payment, need to be processed by a miner and propagated throughout the network.", "symbol":"CH"}' --accountId $NEAR_ID --amount 1`

> Scheduling a call: my-nft-3.wagm.testnet.new_default_meta({"owner_id": "wagm.testnet", "name":"ChainLink Hack", "description":"Scalability – transactions are slower on web3 because theyre decentralized. Changes to state, like a payment, need to be processed by a miner and propagated throughout the network.", "symbol":"CH"}) with attached 1 NEAR

> Doing account.functionCall()
> Transaction Id 4XpbTLWQsMRJpJPEbwcqfBKtrUqEzv24XoquRrAvEa5e
> To see the transaction in the transaction explorer, please open this url in your browser
> https://explorer.testnet.near.org/transactions/4XpbTLWQsMRJpJPEbwcqfBKtrUqEzv24XoquRrAvEa5e
> ''

#### Command : `near call $NEAR_CONTRACT_ID_4 new_default_meta '{"owner_id": "'$NEAR_ID'", "name":"WAGMeet Hack", "description":"No single point of failure: network can still function even if a large proportion of participants are attacked/taken out.", "symbol":"CH"}' --accountId $NEAR_ID --amount 1`

> Scheduling a call: my-nft-4.wagm.testnet.new_default_meta({"owner_id": "wagm.testnet", "name":"WAGMeet Hack", "description":"No single point of failure: network can still function even if a large proportion of participants are attacked/taken out.", "symbol":"CH"}) with attached 1 NEAR

> Doing account.functionCall()
> Transaction Id 7yYgKNHX1K8gW41SkDm2joh6AWLFzYhNJpiK9DWaUyVr
> To see the transaction in the transaction explorer, please open this url in your browser
> https://explorer.testnet.near.org/transactions/7yYgKNHX1K8gW41SkDm2joh6AWLFzYhNJpiK9DWaUyVr
> ''

### -----------------------CONRTACT METADATA-----------------------

```
#1
near view $NEAR_CONTRACT_ID_2 nft_metadata

#2
near view $NEAR_CONTRACT_ID_3 nft_metadata

#3
near view $NEAR_CONTRACT_ID_4 nft_metadata
```
### Results :
```
#1
View call: my-nft-2.wagm.testnet.nft_metadata()
{
  spec: 'nft-1.0.0',
  name: 'Web3 Hack',
  symbol: 'WEB3',
  icon: null,
  base_uri: null,
  reference: null,
  reference_hash: null,
  desc: 'Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend'
}

#2
View call: my-nft-3.wagm.testnet.nft_metadata()
{
  spec: 'nft-1.0.0',
  name: 'ChainLink Hack',
  symbol: 'CH',
  icon: null,
  base_uri: null,
  reference: null,
  reference_hash: null,
  desc: 'Scalability – transactions are slower on web3 because theyre decentralized. Changes to state, like a payment, need to be processed by a miner and propagated throughout the network.'
}

#3
View call: my-nft-4.wagm.testnet.nft_metadata()
{
  spec: 'nft-1.0.0',
  name: 'WAGMeet Hack',
  symbol: 'CH',
  icon: null,
  base_uri: null,
  reference: null,
  reference_hash: null,
  desc: 'No single point of failure: network can still function even if a large proportion of participants are attacked/taken out.'
}
```

-----------------------NFT MINTING....-----------------------
```
# minting on 3 contracts

#1
near call $NEAR_CONTRACT_ID_2 nft_mint '{"token_id": "2", "receiver_id": "'$NEAR_ID'", "metadata": { "media": "https://ipfs.io/ipfs/QmbTtnMWsGHU2v57K3Uh8j4xbttm2sszVuzZTZHE3vyvZf", "copies": 1}}' --accountId $NEAR_ID --deposit 0.1

#2
near call $NEAR_CONTRACT_ID_2 nft_mint '{"token_id": "3", "receiver_id": "'$NEAR_ID'", "metadata": { "media": "https://ipfs.io/ipfs/QmTV2buqsS9zmUDChuVXgRrqoLR1MNMZfsBW5mkEsmfH5i", "copies": 1}}' --accountId $NEAR_ID --deposit 0.1

#3
near call $NEAR_CONTRACT_ID_3 nft_mint '{"token_id": "4", "receiver_id": "'$NEAR_ID'", "metadata": { "media": "https://ipfs.io/ipfs/QmYLpqy83PmtN76UR8V9MfJ5qRXqF3dFhMYB4yxDGaGtTT", "copies": 1}}' --accountId $NEAR_ID --deposit 0.1

#4
near call $NEAR_CONTRACT_ID_3 nft_mint '{"token_id": "5", "receiver_id": "'$NEAR_ID'", "metadata": { "media": "https://ipfs.io/ipfs/QmTV2buqsS9zmUDChuVXgRrqoLR1MNMZfsBW5mkEsmfH5i", "copies": 1}}' --accountId $NEAR_ID --deposit 0.1

#5
near call $NEAR_CONTRACT_ID_4 nft_mint '{"token_id": "6", "receiver_id": "'$NEAR_ID'", "metadata": { "media": "https://ipfs.io/ipfs/QmbTtnMWsGHU2v57K3Uh8j4xbttm2sszVuzZTZHE3vyvZf", "copies": 1}}' --accountId $NEAR_ID --deposit 0.1
```

### Results :
```
#1
Scheduling a call: my-nft-2.wagm.testnet.nft_mint({"token_id": "2", "receiver_id": "wagm.testnet", "metadata": { "media": "https://ipfs.io/ipfs/QmbTtnMWsGHU2v57K3Uh8j4xbttm2sszVuzZTZHE3vyvZf", "copies": 1}}) with attached 0.1 NEAR
Doing account.functionCall()
Transaction Id FutCdajHjahzfj8iw2q2wCvo6kW7yxNdpNgM2HUqoCr4
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/FutCdajHjahzfj8iw2q2wCvo6kW7yxNdpNgM2HUqoCr4
''


#2
Scheduling a call: my-nft-2.wagm.testnet.nft_mint({"token_id": "3", "receiver_id": "wagm.testnet", "metadata": { "media": "https://ipfs.io/ipfs/QmTV2buqsS9zmUDChuVXgRrqoLR1MNMZfsBW5mkEsmfH5i", "copies": 1}}) with attached 0.1 NEAR
Doing account.functionCall()
Transaction Id AtWyJE1oEAYDKVnHgL6mvKoAqdFjkaaBVfVMr4JuBNNn
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/AtWyJE1oEAYDKVnHgL6mvKoAqdFjkaaBVfVMr4JuBNNn
''


#3
Scheduling a call: my-nft-3.wagm.testnet.nft_mint({"token_id": "4", "receiver_id": "wagm.testnet", "metadata": { "media": "https://ipfs.io/ipfs/QmYLpqy83PmtN76UR8V9MfJ5qRXqF3dFhMYB4yxDGaGtTT", "copies": 1}}) with attached 0.1 NEAR
Doing account.functionCall()
Transaction Id BH855bzki7aEi1aprkYFwPNWAvdzB1mPa3x3E4hMm5Qa
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/BH855bzki7aEi1aprkYFwPNWAvdzB1mPa3x3E4hMm5Qa
''


#4
Scheduling a call: my-nft-3.wagm.testnet.nft_mint({"token_id": "5", "receiver_id": "wagm.testnet", "metadata": { "media": "https://ipfs.io/ipfs/QmTV2buqsS9zmUDChuVXgRrqoLR1MNMZfsBW5mkEsmfH5i", "copies": 1}}) with attached 0.1 NEAR
Doing account.functionCall()
Transaction Id 2HKTb6jHPtjVFKV9czo6MsA4Vabe2xgyykCkCfYCe7L9
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/2HKTb6jHPtjVFKV9czo6MsA4Vabe2xgyykCkCfYCe7L9
''


#5
Scheduling a call: my-nft-4.wagm.testnet.nft_mint({"token_id": "6", "receiver_id": "wagm.testnet", "metadata": { "media": "https://ipfs.io/ipfs/QmbTtnMWsGHU2v57K3Uh8j4xbttm2sszVuzZTZHE3vyvZf", "copies": 1}}) with attached 0.1 NEAR
Doing account.functionCall()
Transaction Id C3vNLpizXNEvfyhQnkqsKU3W3EcMYHe1xnR3n6dAZtwX
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/C3vNLpizXNEvfyhQnkqsKU3W3EcMYHe1xnR3n6dAZtwX
''
```


### ----------------------NFTs By Owner-----------------------
```
#1
near view $NEAR_CONTRACT_ID_2 nft_tokens_for_owner '{"account_id": "'$NEAR_ID'"}'

#2
near view $NEAR_CONTRACT_ID_4 nft_tokens_for_owner '{"account_id": "'$NEAR_ID'"}'
```

### Results :
```
#1
View call: my-nft-2.wagm.testnet.nft_tokens_for_owner({"account_id": "wagm.testnet"})
[
  {
    token_id: '2',
    owner_id: 'wagm.testnet',
    metadata: {
      title: 'Ticket-1',
      description: 'Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend',
      media: 'https://ipfs.io/ipfs/QmbTtnMWsGHU2v57K3Uh8j4xbttm2sszVuzZTZHE3vyvZf',
      media_hash: null,
      copies: 1,
      issued_at: null,
      expires_at: null,
      starts_at: null,
      updated_at: null,
      extra: null,
      reference: null,
      reference_hash: null
    }
  },
  {
    token_id: '3',
    owner_id: 'wagm.testnet',
    metadata: {
      title: 'Ticket-2',
      description: 'Web3 servers cant go down – they use Ethereum, a decentralized network of 1000s of computers as their backend',
      media: 'https://ipfs.io/ipfs/QmTV2buqsS9zmUDChuVXgRrqoLR1MNMZfsBW5mkEsmfH5i',
      media_hash: null,
      copies: 1,
      issued_at: null,
      expires_at: null,
      starts_at: null,
      updated_at: null,
      extra: null,
      reference: null,
      reference_hash: null
    }
  }
]


#2
View call: my-nft-4.wagm.testnet.nft_tokens_for_owner({"account_id": "wagm.testnet"})
[
  {
    token_id: '6',
    owner_id: 'wagm.testnet',
    metadata: {
      title: 'Ticket-1',
      description: 'No single point of failure: network can still function even if a large proportion of participants are attacked/taken out.',
      media: 'https://ipfs.io/ipfs/QmbTtnMWsGHU2v57K3Uh8j4xbttm2sszVuzZTZHE3vyvZf',
      media_hash: null,
      copies: 1,
      issued_at: null,
      expires_at: null,
      starts_at: null,
      updated_at: null,
      extra: null,
      reference: null,
      reference_hash: null
    }
  }
]
```
### -------------------------NFT - Total Supply----------------------
```
View call: my-nft-2.wagm.testnet.nft_total_supply()
'2'
View call: my-nft-3.wagm.testnet.nft_total_supply()
'2'
View call: my-nft-4.wagm.testnet.nft_total_supply()
'1'
```
### View specific token :
```near view $NEAR_CONTRACT_ID_4 nft_token '{"token_id": "6"}'```

```
View call: my-nft-4.wagm.testnet.nft_token({"token_id": "6"})
{
  token_id: '6',
  owner_id: 'wagm.testnet',
  metadata: {
    title: 'Ticket-1',
    description: 'No single point of failure: network can still function even if a large proportion of participants are attacked/taken out.',
    media: 'https://ipfs.io/ipfs/QmbTtnMWsGHU2v57K3Uh8j4xbttm2sszVuzZTZHE3vyvZf',
    media_hash: null,
    copies: 1,
    issued_at: null,
    expires_at: null,
    starts_at: null,
    updated_at: null,
    extra: null,
    reference: null,
    reference_hash: null
  }
}
```

>   Now go to collectibles tab in near wallet to see NFTs.
