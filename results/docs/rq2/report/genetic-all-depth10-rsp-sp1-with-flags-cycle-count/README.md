



# Genetic run for metric cycle-count

## Programs

- rsp

## zkVMs

- sp1

## Best profile

- Best profile: Profile(profile_name='genetic', rustflags='-C opt-level=3 -C lto=thin -C embed-bitcode -C codegen-units=1 -Cllvm-args=-available-load-scan-limit=11 -Cllvm-args=-bonus-inst-threshold=5 -Cllvm-args=-early-ifcvt-limit=11 -Cllvm-args=-inline-threshold=5575 -Cllvm-args=-jump-threading-implication-search-threshold=9 -Cllvm-args=-jump-threading-threshold=17 -Cllvm-args=-licm-max-num-uses-traversed=12 -Cllvm-args=-licm-versioning-max-depth-threshold=5 -Cllvm-args=-loop-distribute-scev-check-threshold=65 -Cllvm-args=-loop-interchange-threshold=2 -Cllvm-args=-loop-load-elimination-scev-check-threshold=24 -Cllvm-args=-max-dependences=96 -Cllvm-args=-max-nested-scalar-reduction-interleave=4 -Cllvm-args=-max-num-inline-blocks=11 -Cllvm-args=-max-speculation-depth=3 -Cllvm-args=-max-uses-for-sinking=9 -Cllvm-args=-memdep-block-number-limit=2185 -Cllvm-args=-memdep-block-scan-limit=20', cflags='-O3 -mllvm -available-load-scan-limit=11 -mllvm -bonus-inst-threshold=5 -mllvm -early-ifcvt-limit=11 -mllvm -inline-threshold=5575 -mllvm -jump-threading-implication-search-threshold=9 -mllvm -jump-threading-threshold=17 -mllvm -licm-max-num-uses-traversed=12 -mllvm -licm-versioning-max-depth-threshold=5 -mllvm -loop-distribute-scev-check-threshold=65 -mllvm -loop-interchange-threshold=2 -mllvm -loop-load-elimination-scev-check-threshold=24 -mllvm -max-dependences=96 -mllvm -max-nested-scalar-reduction-interleave=4 -mllvm -max-num-inline-blocks=11 -mllvm -max-speculation-depth=3 -mllvm -max-uses-for-sinking=9 -mllvm -memdep-block-number-limit=2185 -mllvm -memdep-block-scan-limit=20', passes=['module(partial-inliner,argpromotion,argpromotion),function(memcpyopt,separate-const-offset-from-gep,sink),module(partial-inliner),function(mergereturn,loop(loop-idiom),consthoist)'], prepopulate_passes=True, lower_atomic_before=False)
- Metric: 68764766
- Mode: depth-10
- Tune config: Profile(profile_name='genetic', rustflags='-C opt-level=3 -C lto=thin -C embed-bitcode -C codegen-units=1 -Cllvm-args=-available-load-scan-limit=11 -Cllvm-args=-bonus-inst-threshold=5 -Cllvm-args=-early-ifcvt-limit=11 -Cllvm-args=-inline-threshold=5575 -Cllvm-args=-jump-threading-implication-search-threshold=9 -Cllvm-args=-jump-threading-threshold=17 -Cllvm-args=-licm-max-num-uses-traversed=12 -Cllvm-args=-licm-versioning-max-depth-threshold=5 -Cllvm-args=-loop-distribute-scev-check-threshold=65 -Cllvm-args=-loop-interchange-threshold=2 -Cllvm-args=-loop-load-elimination-scev-check-threshold=24 -Cllvm-args=-max-dependences=96 -Cllvm-args=-max-nested-scalar-reduction-interleave=4 -Cllvm-args=-max-num-inline-blocks=11 -Cllvm-args=-max-speculation-depth=3 -Cllvm-args=-max-uses-for-sinking=9 -Cllvm-args=-memdep-block-number-limit=2185 -Cllvm-args=-memdep-block-scan-limit=20', cflags='-O3 -mllvm -available-load-scan-limit=11 -mllvm -bonus-inst-threshold=5 -mllvm -early-ifcvt-limit=11 -mllvm -inline-threshold=5575 -mllvm -jump-threading-implication-search-threshold=9 -mllvm -jump-threading-threshold=17 -mllvm -licm-max-num-uses-traversed=12 -mllvm -licm-versioning-max-depth-threshold=5 -mllvm -loop-distribute-scev-check-threshold=65 -mllvm -loop-interchange-threshold=2 -mllvm -loop-load-elimination-scev-check-threshold=24 -mllvm -max-dependences=96 -mllvm -max-nested-scalar-reduction-interleave=4 -mllvm -max-num-inline-blocks=11 -mllvm -max-speculation-depth=3 -mllvm -max-uses-for-sinking=9 -mllvm -memdep-block-number-limit=2185 -mllvm -memdep-block-scan-limit=20', passes=['module(partial-inliner,argpromotion,argpromotion),function(memcpyopt,separate-const-offset-from-gep,sink),module(partial-inliner),function(mergereturn,loop(loop-idiom),consthoist)'], prepopulate_passes=True, lower_atomic_before=False)

## Overview
  
![genetic-plot](./genetic-plot.png)
## Baseline values

- o1: [MetricValue(zkvm='sp1', program='rsp', metric=128272704, timeout=False)]
- o0: [MetricValue(zkvm='sp1', program='rsp', metric=953617677, timeout=False)]
- o2: [MetricValue(zkvm='sp1', program='rsp', metric=85142351, timeout=False)]
- o3: [MetricValue(zkvm='sp1', program='rsp', metric=83840956, timeout=False)]
- o3-lto: [MetricValue(zkvm='sp1', program='rsp', metric=75055801, timeout=False)]
