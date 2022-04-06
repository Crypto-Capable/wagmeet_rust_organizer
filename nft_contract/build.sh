# cargo build --target wasm32-unknown-unknown --release 
# cp target/wasm32-unknown-unknown/release/nft_contract.wasm res
set -e && RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release && mkdir -p ../out && cp target/wasm32-unknown-unknown/release/*.wasm ../out/main.wasm
