



# Genetic run for metric cycle-count

## Programs

- rsp

## zkVMs

- sp1

## Best profile

- Best profile: ProfileConfig(name='genetic', lto='off', single_codegen_unit=False, opt_level='0', prepopulate_passes=False, passes=['function(sroa,indvars),module(inline),function(loop(loop-unroll-full),sccp),module(aggressive-instcombine),function(sroa,simplifycfg,memcpyopt,jump-threading)'])
- Metric: 313038024
- Mode: depth-10
- Tune config: ProfileConfig(name='genetic', lto='off', single_codegen_unit=False, opt_level='0', prepopulate_passes=False, passes=['function(sroa,indvars),module(inline),function(loop(loop-unroll-full),sccp),module(aggressive-instcombine),function(sroa,simplifycfg,memcpyopt,jump-threading)'])

## Overview
  
![genetic-plot](./genetic-plot.png)
## Baseline values

- o1: [MetricValue(zkvm='sp1', program='rsp', metric=79300786, timeout=False)]
- o2: [MetricValue(zkvm='sp1', program='rsp', metric=60215357, timeout=False)]
- o3: [MetricValue(zkvm='sp1', program='rsp', metric=59700707, timeout=False)]
- o0: [MetricValue(zkvm='sp1', program='rsp', metric=607822737, timeout=False)]
