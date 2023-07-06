#!/bin/bash

# This script runs a relay chain validator. It takes the assumption that the [polkadot repository](https://github.com/paritytech/polkadot) has been checked out
# into ../polkadot and built. Be sure to select the right branch before building (see runtime's Cargo.toml).
#
# In order to run a parachain, 2 validators must be available. This scripts takes as argument the name of the validator. Expected values: alice, bob.
#
# The chainspec file was downloaded from https://docs.substrate.io/assets/tutorials/relay-chain-specs/raw-local-chainspec.json

set -e

if [ "$1" != "alice" ] && [ "$1" != "bob" ]
then
    echo "Unexpected validator name: $1"
    exit 1
fi

if [ "$1" = "alice" ]
then
  P2P_PORT=30333
  RPC_PORT=9944
  OTHER_OPTIONS="--node-key c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a"
else
  P2P_PORT=30334
  RPC_PORT=9945
  OTHER_OPTIONS="--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2"
fi

../polkadot/target/release/polkadot \
    --$1 \
    --base-path /tmp/relay/$1 \
    --chain ./res/raw-local-chainspec.json \
    --port $P2P_PORT \
    --rpc-port $RPC_PORT \
    $OTHER_OPTIONS
