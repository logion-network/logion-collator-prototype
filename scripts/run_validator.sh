#!/bin/bash

# This script runs a relay chain validator. It takes the assumption that the [polkadot repository](https://github.com/paritytech/polkadot) has been checked out
# into ../polkadot and built. Be sure to select the right branch before building (see runtime's Cargo.toml).
#
# In order to run a parachain, 2 validators must be available. This scripts takes as argument the name of the validator. Expected values: alice, bob.

set -e

if [ "$1" != "alice" ] && [ "$1" != "bob" ]
then
    echo "Unexpected validator name: $1"
    exit 1
fi

../polkadot/target/release/polkadot \
    --$1 \
    --validator \
    --base-path /tmp/relay/$1 \
    --chain ./scripts/rococo-custom-2-raw.json \
    --port 30333 \
    --ws-port 9944
