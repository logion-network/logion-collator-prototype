#!/bin/bash

set -e

cd ..
git clone https://github.com/paritytech/polkadot.git
git checkout release-v0.9.22
cargo build --release
