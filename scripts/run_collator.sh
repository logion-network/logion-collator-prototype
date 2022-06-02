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
    --ws-port 8844 \
    -- \
    --execution wasm \
    --chain ./res/rococo-custom-2-raw.json \
    --port 30343 \
    --ws-port 9977
