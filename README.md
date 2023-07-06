# Logion Collator Node

This project contains logion's collator node.

This project is originally a fork of the
[Substrate Cumulus Parachain Template](https://github.com/substrate-developer-hub/substrate-parachain-template/tree/a04589bc06143080982345acdf17635c4118fe48).

## Test locally

Below steps describe the "quick and dirty" way to run your collator node locally and, as a result, be able to test your developments
or play with the network. It does not describe the "production way" of registering a (logion) parachain.

### Prerequisites

Your environment must be ready for Substrate development, see
[here](https://docs.substrate.io/tutorials/v3/create-your-first-substrate-chain/#install-rust-and-the-rust-toolchain)
for a step-by-step guide.

### Setup

Below steps show how to instantiate a local logion parachain and its relay chain. If you already followed those steps
and did not clean-up the data, you can just start the nodes (steps 3, 4 and 10).

1. If not already done, build your relay chain node with command `./scripts/build_relay_chain.sh`

2. If not already done, build logion collator with command `cargo build --release`

3. Run validator alice with command `./scripts/run_validator.sh alice`

4. Run validator bob with command `./scripts/run_validator.sh bob`

5. Reserve para ID

- With [Polkadot.js](https://polkadot.js.org/apps), connect to the local relay chain (`ws://localhost:9944`)
- Go to Network > Parachains > Parathreads
- Click on "+ ParaID" and, with Charlie, register para ID 2000

6. (optional if you did not change the runtime) Generate plain chainspec:

```
./target/release/logion-collator build-spec --disable-default-bootnode > ./res/rococo-local-logion-plain.json
```

7. (optional if you did not change the runtime) Generate raw chainspec

```
./target/release/logion-collator build-spec --chain ./res/rococo-local-logion-plain.json --raw --disable-default-bootnode > ./res/rococo-local-logion-raw.json
```

8. Generate WASM and genesis state

```
./target/release/logion-collator export-genesis-wasm --chain ./res/rococo-local-logion-raw.json > ./bin/local-logion-wasm
```

```
./target/release/logion-collator export-genesis-state --chain ./res/rococo-local-logion-raw.json > ./bin/local-logion-genesis
```

9. Register parachain

- With [Polkadot.js](https://polkadot.js.org/apps), connect to the local relay chain (`ws://localhost:9944`)
- Go to Developer > Sudo
- Select extrinsic `paraSudoWrapper.sudoScheduleParaInitialize` and set the following parameters:
    - id: 2000
    - genesisHead: set file `./bin/local-logion-genesis` generated above
    - validationCode: set file `./bin/local-logion-wasm` generated above
    - parachain: Yes
- Submit the extrinsic

10. Run collator with command `./scripts/run_collator.sh`

11. Wait for the collator to start producing blocks (spy the parachain's best and finalized block in the logs
or via Polkadot.js's dashboard: Network > Parachains), this may take some time (around 3 minutes). Also, block production
may not be stable at the beginning. Again, waiting for a couple of minutes should be enough.

12. You may start interacting with the logion parachain using Polkadot.js and connecting to `ws://localhost:8844`.

### Clean-up

Once the two validators and the collator are stopped and you would like to wipe all previously created state,
you can run the following command:

```
rm -rf /tmp/relay /tmp/parachain/
```

## The Chimay parachain

Chimay is logion's test parachain connecting to logion's test relaychain Orval. Chimay's chainspec (plain and raw) can be found in the `res` folder.
They may be re-generated with the following commands:

```
./target/release/logion-collator build-spec --disable-default-bootnode --chain chimay > ./res/chimay-plain.json
./target/release/logion-collator build-spec --disable-default-bootnode --chain ./res/chimay-plain.json --raw > ./res/chimay-raw.json
```

When registering Chimay, the genesis WASM and state have to be provided. They can be generated as follows (do not forget to build the node binary first):

```
./target/release/logion-collator export-genesis-wasm --chain ./res/chimay-raw.json > ./bin/chimay-genesis-wasm
./target/release/logion-collator export-genesis-state --chain ./res/chimay-raw.json > ./bin/chimay-genesis-state
```

## Cross-compile for Debian 11

Below steps are required if you are building a binary that will run on Debian 11 systems from any Linux system with an incompatible GLIBC or a non-Linux system.

A pre-requisite to cross-compiling the collator node for Debian 11 based systems is to create a Debian/Rust builder image with the following command:

```
./scripts/build_debian_rust.sh
```

This image should be rebuilt on a regular time basis in order to update both Debian and Rust.

Once the image is built, you may run:

```
./scripts/build_debian_collator.sh
```
