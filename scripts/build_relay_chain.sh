#!/bin/bash

set -e

cd ..
if [ -d polkadot ]
then
    cd polkadot
    git pull
else
    git clone https://github.com/paritytech/polkadot.git
    cd polkadot
fi

# ../res/raw-local-chainspec.json may have to be updated on
# branch change, see ./run_validator.sh
git checkout release-v0.9.40

cargo build --release
