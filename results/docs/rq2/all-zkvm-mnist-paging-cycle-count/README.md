



# Genetic run for metric paging-cycle-count

## Programs

- zkvm-mnist

## zkVMs

- risc0

## Best profile

- Best profile: Profile(profile_name='genetic', rustflags='-C opt-level=s -C lto=fat -C embed-bitcode -C codegen-units=1 -Cllvm-args=-available-load-scan-limit=13 -Cllvm-args=-early-ifcvt-limit=39 -Cllvm-args=-inline-threshold=4997 -Cllvm-args=-jump-threading-threshold=10 -Cllvm-args=-licm-max-num-uses-traversed=24 -Cllvm-args=-loop-distribute-scev-check-threshold=127 -Cllvm-args=-loop-interchange-threshold=-2 -Cllvm-args=-max-speculation-depth=13 -Cllvm-args=-max-uses-for-sinking=68 -Cllvm-args=-memdep-block-scan-limit=21', cflags='-Os -mllvm -available-load-scan-limit=13 -mllvm -early-ifcvt-limit=39 -mllvm -inline-threshold=4997 -mllvm -jump-threading-threshold=10 -mllvm -licm-max-num-uses-traversed=24 -mllvm -loop-distribute-scev-check-threshold=127 -mllvm -loop-interchange-threshold=-2 -mllvm -max-speculation-depth=13 -mllvm -max-uses-for-sinking=68 -mllvm -memdep-block-scan-limit=21', passes=['module(argpromotion),function(loop-data-prefetch,nary-reassociate,consthoist),module(argpromotion,deadargelim),function(break-crit-edges,slsr,sink,dse,loop-unroll,loop(loop-unroll-and-jam)),module(deadargelim,wholeprogramdevirt,strip-gc-relocates)'], prepopulate_passes=True, lower_atomic_before=False)
- Metric: 7970802
- Mode: depth-15
- Tune config: Profile(profile_name='genetic', rustflags='-C opt-level=s -C lto=fat -C embed-bitcode -C codegen-units=1 -Cllvm-args=-available-load-scan-limit=13 -Cllvm-args=-early-ifcvt-limit=39 -Cllvm-args=-inline-threshold=4997 -Cllvm-args=-jump-threading-threshold=10 -Cllvm-args=-licm-max-num-uses-traversed=24 -Cllvm-args=-loop-distribute-scev-check-threshold=127 -Cllvm-args=-loop-interchange-threshold=-2 -Cllvm-args=-max-speculation-depth=13 -Cllvm-args=-max-uses-for-sinking=68 -Cllvm-args=-memdep-block-scan-limit=21', cflags='-Os -mllvm -available-load-scan-limit=13 -mllvm -early-ifcvt-limit=39 -mllvm -inline-threshold=4997 -mllvm -jump-threading-threshold=10 -mllvm -licm-max-num-uses-traversed=24 -mllvm -loop-distribute-scev-check-threshold=127 -mllvm -loop-interchange-threshold=-2 -mllvm -max-speculation-depth=13 -mllvm -max-uses-for-sinking=68 -mllvm -memdep-block-scan-limit=21', passes=['module(argpromotion),function(loop-data-prefetch,nary-reassociate,consthoist),module(argpromotion,deadargelim),function(break-crit-edges,slsr,sink,dse,loop-unroll,loop(loop-unroll-and-jam)),module(deadargelim,wholeprogramdevirt,strip-gc-relocates)'], prepopulate_passes=True, lower_atomic_before=False)

## Overview
  
![genetic-plot](./genetic-plot.png)
## Baseline values

- o2: [MetricValue(zkvm='risc0', program='zkvm-mnist', metric=8760818, timeout=False)]
- o1: [MetricValue(zkvm='risc0', program='zkvm-mnist', metric=8622294, timeout=False)]
- o3: [MetricValue(zkvm='risc0', program='zkvm-mnist', metric=8674392, timeout=False)]
- o3-lto: [MetricValue(zkvm='risc0', program='zkvm-mnist', metric=8596984, timeout=False)]
