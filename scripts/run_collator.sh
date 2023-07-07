#!/bin/bash

# This script runs a logion collator.

set -e

./target/release/logion-collator \
    --alice \
    --collator \
    --force-authoring \
    --chain ./res/rococo-local-logion-raw.json \
    --base-path /tmp/parachain/alice \
    --port 40333 \
    --rpc-port 8844 \
    -- \
    --execution wasm \
    --chain ./res/raw-local-chainspec.json \
    --port 30343 \
    --rpc-port 9977 \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2
