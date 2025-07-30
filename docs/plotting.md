# Plotting

The following gives a brief guide of how to reproduce the generated plots.

## Requirements and setup

Make sure you have `uv` installed.

First clone the [thesis repo](https://github.com/thomasgassmann/thesis). Next download the [raw data](https://polybox.ethz.ch/index.php/s/CWbkyejZLmMtGBD) and extract it to `<thesis repo>/results`.

The `<thesis repo>/results/bench` directory contains the raw data from the first run. This data should in general NOT be used as it was run on different (and shared) GPUs.

### Directory layout

The current raw data (all tested with the hardware listed in the report) is located in the `<thesis repo>/results/bench-rerun` directory.

`<thesis repo>/results/bench-rerun` contains the data used in the motivating examples.

`<thesis repo>/results/exhaustive` contains the raw results for the exhaustive testing of depth 2.

`<thesis repo>/results/ffd` contains the raw results for the FFD.

`<thesis repo>/results/genetic` contains the raw data for all opentuner experiments.

`<thesis repo>/results/root-cause-analysis-data` contains supporting data for the root cause analysis.

## Plotting scripts

All plots are generated via the `zkbench` python module, which can be run via the `./zkbench.sh` script. The plotting command are under `./zkbench.sh plot` for RQ1 and `./zkbench.sh plot-tune` for RQ2.

### Examples

Each figure in the thesis also contains the command that was used to generate it as a comment (e.g. `./zkbench.sh plot --remove-ox --dir results/bench-rerun --font-size 20 cycle-count --global-average --drop-below 0.1`).

Some examples:

- Create export of RQ1 data (e.g. recreate the data in `./results/docs/rq1/report`): `./zkbench.sh plot --dir ./results/bench-rerun export --out ./results/docs/rq1/report`
- Show exhaustive heatmap: `./zkbench.sh plot-tune exhaustive-depth2 --stats results/exhaustive/all-cycle-count/stats.json --relative`
- Plot average improvement for proving, zkVM execution and x86 execution: `./zkbench.sh plot --dir ./results/bench-rerun average-improvement --show-x86`

Running `./zkbench.sh plot --help` also prints a list of all possible plots along with a brief description.
