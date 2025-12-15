#!/bin/bash

cargo build --release --target wasm32-unknown-unknown

rm ~/.config/linera -rf
linera wallet init --faucet https://faucet.testnet-conway.linera.net

linera wallet request-chain --faucet https://faucet.testnet-conway.linera.net
linera wallet request-chain --faucet https://faucet.testnet-conway.linera.net

MODULE_ID=$(linera publish-module ./target/wasm32-unknown-unknown/release/prediction_market_{contract,service}.wasm)
linera create-application $MODULE_ID \
    --json-argument '{}'

linera service --port 8080
