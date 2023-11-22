#!/bin/bash

set -e

cd ..
if [ -d polkadot-sdk ]
then
    cd polkadot-sdk/polkadot
    git pull
else
    git clone https://github.com/paritytech/polkadot-sdk.git
    cd polkadot-sdk/polkadot
fi

# ../res/raw-local-chainspec.json may have to be updated on
# branch change, see ./run_validator.sh
git checkout tags/polkadot-v1.3.0 -b polkadot-v1.3.0

cargo build --release
