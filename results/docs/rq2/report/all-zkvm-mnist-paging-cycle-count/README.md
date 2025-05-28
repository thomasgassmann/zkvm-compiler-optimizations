



# Genetic run for metric paging-cycle-count

## Programs

- zkvm-mnist

## zkVMs

- risc0

## Best profile

- Best profile: Profile(profile_name='genetic', rustflags='-C opt-level=s -C lto=fat -C embed-bitcode -C codegen-units=1 -Cllvm-args=-available-load-scan-limit=13 -Cllvm-args=-early-ifcvt-limit=39 -Cllvm-args=-inline-threshold=4997 -Cllvm-args=-jump-threading-threshold=10 -Cllvm-args=-licm-max-num-uses-traversed=24 -Cllvm-args=-loop-distribute-scev-check-threshold=127 -Cllvm-args=-loop-interchange-threshold=-2 -Cllvm-args=-max-speculation-depth=13 -Cllvm-args=-max-uses-for-sinking=68 -Cllvm-args=-memdep-block-scan-limit=21', cflags='-Os -mllvm -available-load-scan-limit=13 -mllvm -early-ifcvt-limit=39 -mllvm -inline-threshold=4997 -mllvm -jump-threading-threshold=10 -mllvm -licm-max-num-uses-traversed=24 -mllvm -loop-distribute-scev-check-threshold=127 -mllvm -loop-interchange-threshold=-2 -mllvm -max-speculation-depth=13 -mllvm -max-uses-for-sinking=68 -mllvm -memdep-block-scan-limit=21', passes=['module(argpromotion),function(loop-data-prefetch,nary-reassociate,consthoist),module(argpromotion,deadargelim),function(break-crit-edges,slsr,sink,dse,loop-unroll,loop(loop-unroll-and-jam)),module(deadargelim,wholeprogramdevirt,strip-gc-relocates)'], prepopulate_passes=True, lower_atomic_before=False)
- Metric: 7970802
- Mode: depth-15
- Tune config: TuneConfig(tune_lto=True, tune_codegen_units=True, tune_opt_level=True, tune_prepopulate_passes=True, module_passes=['always-inline', 'inline', 'partial-inliner', 'attributor', 'add-discriminators', 'globalsplit', 'globaldce', 'globalopt', 'wholeprogramdevirt', 'lower-global-dtors', 'strip', 'strip-dead-debug-info', 'strip-dead-prototypes', 'bounds-checking', 'loop-extract', 'mergefunc', 'extract-blocks', 'constmerge', 'deadargelim', 'function-attrs', 'strip-gc-relocates', 'hotcoldsplit', 'argpromotion', 'ipsccp', 'synthetic-counts-propagation', 'rel-lookup-table-converter', 'aggressive-instcombine'], function_passes=['loop-mssa(licm)', 'instcombine', 'bdce', 'correlated-propagation', 'loop-sink', 'loop-data-prefetch', 'loop-fusion', 'mergeicmps', 'mldst-motion', 'newgvn', 'partially-inline-libcalls', 'sroa', 'sink', 'speculative-execution', 'slsr', 'sccp', 'gvn', 'tailcallelim', 'adce', 'dse', 'indvars', 'jump-threading', 'lcssa', 'loop-unroll', 'memcpyopt', 'loop-simplify', 'simplifycfg', 'reassociate', 'mem2reg', 'reg2mem', 'simple-loop-unswitch', 'mergereturn', 'break-crit-edges', 'dce', 'lower-invoke', 'lower-switch', 'callsite-splitting', 'consthoist', 'div-rem-pairs', 'early-cse', 'float2int', 'gvn-hoist', 'gvn-sink', 'guard-widening', 'irce', 'instsimplify', 'libcalls-shrinkwrap', 'nary-reassociate', 'separate-const-offset-from-gep'], loop_passes=['loop-idiom', 'loop-reduce', 'loop-rotate', 'loop-unroll-and-jam', 'loop-unroll-full', 'loop-deletion', 'loop-instsimplify', 'loop-interchange', 'loop-predication', 'loop-versioning-licm'], allowed_opt_levels=['0', '1', '2', '3', 's', 'z'], default_prepopulate_passes=True, default_single_codegen_unit=False, allowed_lto=['off', 'thin', 'fat'])

## Overview
  
![genetic-plot](./genetic-plot.png)
## Baseline values

- o2: [MetricValue(zkvm='risc0', program='zkvm-mnist', metric=8760818, timeout=False)]
- o1: [MetricValue(zkvm='risc0', program='zkvm-mnist', metric=8622294, timeout=False)]
- o3: [MetricValue(zkvm='risc0', program='zkvm-mnist', metric=8674392, timeout=False)]
- o3-lto: [MetricValue(zkvm='risc0', program='zkvm-mnist', metric=8596984, timeout=False)]
