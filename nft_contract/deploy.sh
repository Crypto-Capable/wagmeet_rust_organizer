./build.sh
near delete nft.meghaha.testnet meghaha.testnet
near create-account nft.meghaha.testnet --masterAccount meghaha.testnet --initialBalance 20
near deploy --wasmFile target/wasm32-unknown-unknown/release/nft_contract.wasm --accountId nft.meghaha.testnet --initFunction 'new_default_meta' --initArgs '{"owner_id" : "nft.meghaha.testnet"}'