# Running rsp

Tested on rsp (reth-1.3.10).

## Getting blocks

### Prerequisites

Create a new app on [Alchemy](https://dashboard.alchemy.com/) and enable the Ethereum network. You should get some URL like:

```text
https://eth-mainnet.g.alchemy.com/v2/<key here>
```

Make sure `rustup` and `sp1up` is available.

### Getting the block

1. Clone rsp: `git clone --branch reth-1.3.10 https://github.com/succinctlabs/rsp.git`
2. Install rsp binary: `cargo install --locked --path bin/host`
3. Extracting the block: `rsp --block-number <block number here> --rpc-url https://eth-mainnet.g.alchemy.com/v2/<alchemy key here> --cache-dir <cache dir>`
4. The block should now be in `<cache dir>/input/1` (named `<block number>.bin`).

When extracting multiple blocks I previously also used `rsp-tests` ([docs](https://github.com/succinctlabs/rsp-tests)). [Here](https://github.com/succinctlabs/rsp/tree/4081e40833aebece5958518724a327ede100f6cd#) also the docs for the rsp version above.

It is important that the block above is extracted with the same rsp version that is being run later, otherwise there will be serialization issues.

## Setting up thesis repo

`rustup`, `sp1up` and `rzup` needs to be installed. For Python, `uv` is also required. For the specific versions used, refer to the top-level README.

Clone the repo and build the benchmark runner:

```bash
git clone --branch rsp https://github.com/thomasgassmann/thesis.git
./scripts/build.sh
```

The `main` branch of the thesis repo contains an older version of `rsp`. The `rsp` branch contains the latest tagged version (but not the latest commit).

On the `rsp` branch only sp1 works for rsp. On the `main` branch, both sp1 and risc0 work for rsp.

## Building rsp

You can build `rsp` as follows:

```bash
./zkbench.sh --log-level DEBUG build --program rsp --zkvm sp1 --profile o3 
```

The log-level is optional. Any other profile defined in `config.json` can be used instead of `o3` as well. This places the binary in the `./bin/rsp/sp1` directory. If the binary already exists, use the `--force` flag to overwrite.

## Running

This assumes the thesis repo has been set up and you have a valid block to prove on. All available blocks are in the `./inputs/rsp` directory of the thesis repo. Every block should be named `<block number>.bin`. Any block extracted as described above can be placed in this directory to run benchmarks with it.

To run a benchmark with criterion using a specific block:

```bash
./zkbench.sh bench --program rsp --zkvm sp1 --profile o3 --input-override <block-number>
```

Or benchmarking on a GPU:

```bash
SP1_PROVER=cuda ./zkbench.sh bench --program rsp --zkvm sp1 --profile o3 --input-override <block-number>
```

Note that benchmarking on the GPU differs compared to what "standard" sp1 does, as we override the `sp1-cuda` crate to run the moongate server without docker.

## Running opentuner

> Note: genetic tuning currently does not have a `--input-override` flag, so manually updating the block id in `input.rs` is necessary (followed by a rebuild using `./scripts/build.sh`).

```bash
./zkbench.sh tune --zkvm sp1 --program rsp --metric cycle-count --config ./configs/tune/all.json --out outputdir genetic --mode depth --depth 5 --baseline o3
```

Use `--help` to see all options for the above command (and all supported metrics). Additionally, the config file may also be updated to reduce/expand the search space.
