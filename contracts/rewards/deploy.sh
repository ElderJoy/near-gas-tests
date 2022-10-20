#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deploying contract"

# https://docs.near.org/tools/near-cli#near-dev-deploy
near dev-deploy --wasmFile ../../target/wasm32-unknown-unknown/release/rewards.wasm
# near call dev-1666276247392-15504856141483 init '{"hello_account": "dev-1666250989158-46070615461317"}' --accountId dblinov.testnet
# near call dev-1666276247392-15504856141483 query_greeting --accountId dblinov.testnet