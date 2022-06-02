# Logion Collator Node

This project contains logion's collator node.

This project is originally a fork of the
[Substrate Cumulus Parachain Template](https://github.com/substrate-developer-hub/substrate-parachain-template/tree/ffb52cf5ba20eb824a792c927092196edd424f4d).

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

Do not forget to set `para_id` and `parachainId` fields to 2000.

7. (optional if you did not change the runtime) Generate raw chainspec

```
./target/release/logion-collator build-spec --chain ./res/rococo-local-logion-plain.json --raw --disable-default-bootnode > ./res/rococo-local-logion-raw.json
```

8. Generate WASM and genesis state

```
./target/release/logion-collator export-genesis-wasm --chain ./res/rococo-local-logion-raw.json > ./bin/logion-wasm
```

```
./target/release/logion-collator export-genesis-state --chain ./res/rococo-local-logion-raw.json > ./bin/logion-genesis
```

9. Register parachain

- With [Polkadot.js](https://polkadot.js.org/apps), connect to the local relay chain (`ws://localhost:9944`)
- Go to Developer > Sudo
- Select extrinsic `paraSudoWrapper.sudoScheduleParaInitialize` and set the following parameters:
    - id: 2000
    - genesisHead: set file `./bin/logion-genesis` generated above
    - validationCode: set file `./bin/logion-wasm` generated above
    - parachain: Yes
- Submit the extrinsic

10. Run collator with command `./scripts/run_collator.sh`

11. Wait for the collator to start producing blocks (spy the parachain's best and finalized block in the logs
or via Polkadot.js's dashboard: Network > Parachains ), this may take some time (around 3 minutes).

12. You may start interacting with the logion parachain using Polkadot.js and connecting to `ws://localhost:8844`.

### Clean-up

Once the two validators and the collator are stopped and you would like to wipe all previously created state,
you can run the following command:

```
rm -rf /tmp/relay /tmp/parachain/
```
