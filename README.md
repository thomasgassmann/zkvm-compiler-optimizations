# Evaluating Compiler Optimization Impacts on zkVM Performance

This repository contains the code for the paper [Evaluating Compiler Optimization Impacts on zkVM Performance](https://arxiv.org/abs/2508.17518), which resulted from my Bachelor's thesis at ETH Zürich.

## Getting started

To get started, build the runner:

```bash
$ ./scripts/build.sh
```

Alternatively, with CUDA support, use:

```bash
$ ./scripts/build_cuda.sh
```

After the runner is built, most commands are available via the `zkbench` cli:

```bash
$ ./zkbench.sh --help
Usage: python -m zkbench [OPTIONS] COMMAND [ARGS]...

Options:
  --log-level TEXT
  --log-file TEXT
  --help            Show this message and exit.

Commands:
  asm
  bench
  build
  clean
  plot
  plot-tune
  run
  tune
```

### Rerunning benchmarks

To rerun all benchmarks, first build the binaries:

```bash
$ ./zkbench.sh build -j$(nproc)
```

Next, run the benchmarks via `./zkbench.sh bench`. To see all available options, run `./zkbench.sh bench --help`.

### Autotuning a binary

To explore the optimization space, use the `tune` subcommand.

Examples:

```bash
$ ./zkbench.sh tune --program fibonacci --config ./configs/tune/all-passes.json --metric cycle-count --out test genetic --depth 10 --baseline o3 --mode depth
$ ./zkbench.sh tune --program fibonacci --config ./configs/tune/all-passes-o3-codegen.json --metric cycle-count --out test ffd --resolution 7
$ ./zkbench.sh tune --program fibonacci --config ./configs/tune/all-passes.json --metric cycle-count --out test exhaustive --depth 2
```

## Results

The plots generated from the raw data for the respective experiments in the paper are located in the [results](./results/) directory. All of the raw data is available [here](https://polybox.ethz.ch/index.php/s/CjaTzdHssQyoSa4). For documentation on how to plot these results, refer to [the plotting documentation](./docs/plotting.md).

## Versions

| Environment | Tool           | Version                       |
| ----------- | -------------- | ----------------------------- |
| risc0       | rust           | 1.85.0                        |
| risc0       | cpp            | 2024.1.5                      |
| risc0       | r0vm           | 2.0.0                         |
| risc0       | cargo-risczero | 2.0.0                         |
| sp1         | sp1-sdk        | 4.1.4                         |
| x86         | rust           | 1.85-x86_64-unknown-linux-gnu |
