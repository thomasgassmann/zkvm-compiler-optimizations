



# Genetic run for metric cycle-count

## Programs

- rsp

## zkVMs

- sp1

## Best profile

- Best profile: ProfileConfig(name='genetic', lto='fat', single_codegen_unit=True, opt_level='3', prepopulate_passes=True, passes=['module(synthetic-counts-propagation),function(jump-threading,sink,simplifycfg),module(deadargelim),function(gvn),module(argpromotion),function(mergereturn),module(partial-inliner,loop-extract,argpromotion),function(sroa,gvn-sink,nary-reassociate),module(ipsccp),function(nary-reassociate,memcpyopt,gvn-hoist,early-cse),module(wholeprogramdevirt)'])
- Metric: 56650515
- Mode: depth-20
- Tune config: ProfileConfig(name='genetic', lto='fat', single_codegen_unit=True, opt_level='3', prepopulate_passes=True, passes=['module(synthetic-counts-propagation),function(jump-threading,sink,simplifycfg),module(deadargelim),function(gvn),module(argpromotion),function(mergereturn),module(partial-inliner,loop-extract,argpromotion),function(sroa,gvn-sink,nary-reassociate),module(ipsccp),function(nary-reassociate,memcpyopt,gvn-hoist,early-cse),module(wholeprogramdevirt)'])

## Overview
  
![genetic-plot](./genetic-plot.png)
## Baseline values

- o1: [MetricValue(zkvm='sp1', program='rsp', metric=79297811, timeout=False)]
- o0: [MetricValue(zkvm='sp1', program='rsp', metric=607823116, timeout=False)]
- o2: [MetricValue(zkvm='sp1', program='rsp', metric=60214056, timeout=False)]
- o3: [MetricValue(zkvm='sp1', program='rsp', metric=59700827, timeout=False)]
