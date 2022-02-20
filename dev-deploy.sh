#!/bin/bash
set -e

near dev-deploy out/main.wasm

CONTRACT_ID=$(<neardev/dev-account)

near create-account nft-mine."$CONTRACT_ID" --masterAccount "$CONTRACT_ID" --initialBalance 5
near deploy --accountId nft-mine."$CONTRACT_ID" --wasmFile out/nft-mine.wasm

near create-account nft-stone."$CONTRACT_ID" --masterAccount "$CONTRACT_ID" --initialBalance 5
near deploy --accountId nft-stone."$CONTRACT_ID" --wasmFile out/nft-stone.wasm

near create-account ft."$CONTRACT_ID" --masterAccount "$CONTRACT_ID" --initialBalance 5
near deploy --accountId ft."$CONTRACT_ID" --wasmFile out/ft.wasm
