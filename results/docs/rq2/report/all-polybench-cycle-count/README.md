



# Genetic run for metric cycle-count

## Programs

- polybench-2mm
- polybench-3mm
- polybench-adi
- polybench-atax
- polybench-bicg
- polybench-cholesky
- polybench-correlation
- polybench-covariance
- polybench-deriche
- polybench-doitgen
- polybench-durbin
- polybench-fdtd-2d
- polybench-floyd-warshall
- polybench-gemm
- polybench-gemver
- polybench-gesummv
- polybench-gramschmidt
- polybench-heat-3d
- polybench-jacobi-1d
- polybench-jacobi-2d
- polybench-lu
- polybench-ludcmp
- polybench-mvt
- polybench-nussinov
- polybench-seidel-2d
- polybench-symm
- polybench-syr2k
- polybench-syrk
- polybench-trisolv
- polybench-trmm

## zkVMs

- risc0
- sp1

## Best profile

- Best profile: Profile(profile_name='genetic', rustflags='-C opt-level=3 -Cllvm-args=-available-load-scan-limit=9 -Cllvm-args=-bonus-inst-threshold=1 -Cllvm-args=-inline-threshold=14951 -Cllvm-args=-jump-threading-implication-search-threshold=2 -Cllvm-args=-jump-threading-threshold=19 -Cllvm-args=-loop-interchange-threshold=0 -Cllvm-args=-max-dependences=81 -Cllvm-args=-max-nested-scalar-reduction-interleave=5 -Cllvm-args=-max-num-inline-blocks=12 -Cllvm-args=-max-speculation-depth=2 -Cllvm-args=-memdep-block-number-limit=2021 -Cllvm-args=-memdep-block-scan-limit=3', cflags='-O3 -mllvm -available-load-scan-limit=9 -mllvm -bonus-inst-threshold=1 -mllvm -inline-threshold=14951 -mllvm -jump-threading-implication-search-threshold=2 -mllvm -jump-threading-threshold=19 -mllvm -loop-interchange-threshold=0 -mllvm -max-dependences=81 -mllvm -max-nested-scalar-reduction-interleave=5 -mllvm -max-num-inline-blocks=12 -mllvm -max-speculation-depth=2 -mllvm -memdep-block-number-limit=2021 -mllvm -memdep-block-scan-limit=3', passes=['function(consthoist,float2int,gvn-sink,speculative-execution),module(wholeprogramdevirt),function(dse),module(attributor),function(guard-widening,correlated-propagation,separate-const-offset-from-gep,lower-switch),module(inline),function(sink,loop-mssa(licm),loop-fusion,partially-inline-libcalls,loop-sink),module(deadargelim,function-attrs,globalsplit,globaldce),function(simple-loop-unswitch,mergereturn,loop(loop-interchange,loop-unroll-full)),module(always-inline,ipsccp,rel-lookup-table-converter),function(indvars,reg2mem,loop(loop-predication),break-crit-edges,mldst-motion,sccp),module(extract-blocks),function(loop(loop-idiom)),module(strip-dead-prototypes,strip-gc-relocates),function(tailcallelim,reassociate,gvn-hoist,loop(loop-unroll-and-jam),nary-reassociate,loop(loop-versioning-licm),mem2reg),module(loop-extract),function(jump-threading)'], prepopulate_passes=True, lower_atomic_before=False)
- Metric: 27473855
- Mode: default
- Tune config: TuneConfig(tune_lto=True, tune_codegen_units=True, tune_opt_level=True, tune_prepopulate_passes=False, module_passes=['always-inline', 'inline', 'partial-inliner', 'attributor', 'add-discriminators', 'globalsplit', 'globaldce', 'globalopt', 'wholeprogramdevirt', 'lower-global-dtors', 'strip', 'strip-dead-debug-info', 'strip-dead-prototypes', 'bounds-checking', 'loop-extract', 'mergefunc', 'extract-blocks', 'constmerge', 'deadargelim', 'function-attrs', 'strip-gc-relocates', 'hotcoldsplit', 'argpromotion', 'ipsccp', 'synthetic-counts-propagation', 'rel-lookup-table-converter', 'aggressive-instcombine'], function_passes=['loop-mssa(licm)', 'instcombine', 'bdce', 'correlated-propagation', 'loop-sink', 'loop-data-prefetch', 'loop-fusion', 'mergeicmps', 'mldst-motion', 'newgvn', 'partially-inline-libcalls', 'sroa', 'sink', 'speculative-execution', 'slsr', 'sccp', 'gvn', 'tailcallelim', 'adce', 'dse', 'indvars', 'jump-threading', 'lcssa', 'loop-unroll', 'memcpyopt', 'loop-simplify', 'simplifycfg', 'reassociate', 'mem2reg', 'reg2mem', 'simple-loop-unswitch', 'mergereturn', 'break-crit-edges', 'dce', 'lower-invoke', 'lower-switch', 'callsite-splitting', 'consthoist', 'div-rem-pairs', 'early-cse', 'float2int', 'gvn-hoist', 'gvn-sink', 'guard-widening', 'irce', 'instsimplify', 'libcalls-shrinkwrap', 'nary-reassociate', 'separate-const-offset-from-gep'], loop_passes=['loop-idiom', 'loop-reduce', 'loop-rotate', 'loop-unroll-and-jam', 'loop-unroll-full', 'loop-deletion', 'loop-instsimplify', 'loop-interchange', 'loop-predication', 'loop-versioning-licm'], allowed_opt_levels=['2', '3'], default_prepopulate_passes=True, default_single_codegen_unit=True, allowed_lto=['off', 'thin', 'fat'])

## Overview
  
![genetic-plot](./genetic-plot.png)
## Overview by program

### Program polybench-2mm
  
![genetic-plot-polybench-2mm](./genetic-plot-polybench-2mm.png)
### Program polybench-3mm
  
![genetic-plot-polybench-3mm](./genetic-plot-polybench-3mm.png)
### Program polybench-adi
  
![genetic-plot-polybench-adi](./genetic-plot-polybench-adi.png)
### Program polybench-atax
  
![genetic-plot-polybench-atax](./genetic-plot-polybench-atax.png)
### Program polybench-bicg
  
![genetic-plot-polybench-bicg](./genetic-plot-polybench-bicg.png)
### Program polybench-cholesky
  
![genetic-plot-polybench-cholesky](./genetic-plot-polybench-cholesky.png)
### Program polybench-correlation
  
![genetic-plot-polybench-correlation](./genetic-plot-polybench-correlation.png)
### Program polybench-covariance
  
![genetic-plot-polybench-covariance](./genetic-plot-polybench-covariance.png)
### Program polybench-deriche
  
![genetic-plot-polybench-deriche](./genetic-plot-polybench-deriche.png)
### Program polybench-doitgen
  
![genetic-plot-polybench-doitgen](./genetic-plot-polybench-doitgen.png)
### Program polybench-durbin
  
![genetic-plot-polybench-durbin](./genetic-plot-polybench-durbin.png)
### Program polybench-fdtd-2d
  
![genetic-plot-polybench-fdtd-2d](./genetic-plot-polybench-fdtd-2d.png)
### Program polybench-floyd-warshall
  
![genetic-plot-polybench-floyd-warshall](./genetic-plot-polybench-floyd-warshall.png)
### Program polybench-gemm
  
![genetic-plot-polybench-gemm](./genetic-plot-polybench-gemm.png)
### Program polybench-gemver
  
![genetic-plot-polybench-gemver](./genetic-plot-polybench-gemver.png)
### Program polybench-gesummv
  
![genetic-plot-polybench-gesummv](./genetic-plot-polybench-gesummv.png)
### Program polybench-gramschmidt
  
![genetic-plot-polybench-gramschmidt](./genetic-plot-polybench-gramschmidt.png)
### Program polybench-heat-3d
  
![genetic-plot-polybench-heat-3d](./genetic-plot-polybench-heat-3d.png)
### Program polybench-jacobi-1d
  
![genetic-plot-polybench-jacobi-1d](./genetic-plot-polybench-jacobi-1d.png)
### Program polybench-jacobi-2d
  
![genetic-plot-polybench-jacobi-2d](./genetic-plot-polybench-jacobi-2d.png)
### Program polybench-lu
  
![genetic-plot-polybench-lu](./genetic-plot-polybench-lu.png)
### Program polybench-ludcmp
  
![genetic-plot-polybench-ludcmp](./genetic-plot-polybench-ludcmp.png)
### Program polybench-mvt
  
![genetic-plot-polybench-mvt](./genetic-plot-polybench-mvt.png)
### Program polybench-nussinov
  
![genetic-plot-polybench-nussinov](./genetic-plot-polybench-nussinov.png)
### Program polybench-seidel-2d
  
![genetic-plot-polybench-seidel-2d](./genetic-plot-polybench-seidel-2d.png)
### Program polybench-symm
  
![genetic-plot-polybench-symm](./genetic-plot-polybench-symm.png)
### Program polybench-syr2k
  
![genetic-plot-polybench-syr2k](./genetic-plot-polybench-syr2k.png)
### Program polybench-syrk
  
![genetic-plot-polybench-syrk](./genetic-plot-polybench-syrk.png)
### Program polybench-trisolv
  
![genetic-plot-polybench-trisolv](./genetic-plot-polybench-trisolv.png)
### Program polybench-trmm
  
![genetic-plot-polybench-trmm](./genetic-plot-polybench-trmm.png)
## Overview by program group

### Group polybench
  
![genetic-plot-polybench](./genetic-plot-polybench.png)
### Group rust
  
![genetic-plot-rust](./genetic-plot-rust.png)
## Overview by zkVM

### zkVM risc0
  
![genetic-plot-risc0](./genetic-plot-risc0.png)
### zkVM sp1
  
![genetic-plot-sp1](./genetic-plot-sp1.png)
## Baseline values

- o2: [MetricValue(zkvm='risc0', program='polybench-2mm', metric=450621, timeout=False), MetricValue(zkvm='risc0', program='polybench-3mm', metric=768620, timeout=False), MetricValue(zkvm='risc0', program='polybench-adi', metric=1684177, timeout=False), MetricValue(zkvm='risc0', program='polybench-atax', metric=357742, timeout=False), MetricValue(zkvm='risc0', program='polybench-bicg', metric=347945, timeout=False), MetricValue(zkvm='risc0', program='polybench-cholesky', metric=2215049, timeout=False), MetricValue(zkvm='risc0', program='polybench-correlation', metric=760076, timeout=False), MetricValue(zkvm='risc0', program='polybench-covariance', metric=461808, timeout=False), MetricValue(zkvm='risc0', program='polybench-deriche', metric=1786829, timeout=False), MetricValue(zkvm='risc0', program='polybench-doitgen', metric=150616, timeout=False), MetricValue(zkvm='risc0', program='polybench-durbin', metric=141849, timeout=False), MetricValue(zkvm='risc0', program='polybench-fdtd-2d', metric=1109541, timeout=False), MetricValue(zkvm='risc0', program='polybench-floyd-warshall', metric=101993, timeout=False), MetricValue(zkvm='risc0', program='polybench-gemm', metric=433002, timeout=False), MetricValue(zkvm='risc0', program='polybench-gemver', metric=2824322, timeout=False), MetricValue(zkvm='risc0', program='polybench-gesummv', metric=1919858, timeout=False), MetricValue(zkvm='risc0', program='polybench-gramschmidt', metric=537758, timeout=False), MetricValue(zkvm='risc0', program='polybench-heat-3d', metric=785316, timeout=False), MetricValue(zkvm='risc0', program='polybench-jacobi-1d', metric=1569463, timeout=False), MetricValue(zkvm='risc0', program='polybench-jacobi-2d', metric=478817, timeout=False), MetricValue(zkvm='risc0', program='polybench-lu', metric=1105607, timeout=False), MetricValue(zkvm='risc0', program='polybench-ludcmp', metric=1184132, timeout=False), MetricValue(zkvm='risc0', program='polybench-mvt', metric=550838, timeout=False), MetricValue(zkvm='risc0', program='polybench-nussinov', metric=20701, timeout=False), MetricValue(zkvm='risc0', program='polybench-seidel-2d', metric=2551852, timeout=False), MetricValue(zkvm='risc0', program='polybench-symm', metric=1719217, timeout=False), MetricValue(zkvm='risc0', program='polybench-syr2k', metric=2321504, timeout=False), MetricValue(zkvm='risc0', program='polybench-syrk', metric=936007, timeout=False), MetricValue(zkvm='risc0', program='polybench-trisolv', metric=159681, timeout=False), MetricValue(zkvm='risc0', program='polybench-trmm', metric=761513, timeout=False), MetricValue(zkvm='sp1', program='polybench-2mm', metric=367920, timeout=False), MetricValue(zkvm='sp1', program='polybench-3mm', metric=623557, timeout=False), MetricValue(zkvm='sp1', program='polybench-adi', metric=1342238, timeout=False), MetricValue(zkvm='sp1', program='polybench-atax', metric=291794, timeout=False), MetricValue(zkvm='sp1', program='polybench-bicg', metric=283958, timeout=False), MetricValue(zkvm='sp1', program='polybench-cholesky', metric=1816783, timeout=False), MetricValue(zkvm='sp1', program='polybench-correlation', metric=607831, timeout=False), MetricValue(zkvm='sp1', program='polybench-covariance', metric=373985, timeout=False), MetricValue(zkvm='sp1', program='polybench-deriche', metric=1446965, timeout=False), MetricValue(zkvm='sp1', program='polybench-doitgen', metric=125411, timeout=False), MetricValue(zkvm='sp1', program='polybench-durbin', metric=115928, timeout=False), MetricValue(zkvm='sp1', program='polybench-fdtd-2d', metric=861915, timeout=False), MetricValue(zkvm='sp1', program='polybench-floyd-warshall', metric=99792, timeout=False), MetricValue(zkvm='sp1', program='polybench-gemm', metric=354108, timeout=False), MetricValue(zkvm='sp1', program='polybench-gemver', metric=2283724, timeout=False), MetricValue(zkvm='sp1', program='polybench-gesummv', metric=1550483, timeout=False), MetricValue(zkvm='sp1', program='polybench-gramschmidt', metric=432066, timeout=False), MetricValue(zkvm='sp1', program='polybench-heat-3d', metric=626255, timeout=False), MetricValue(zkvm='sp1', program='polybench-jacobi-1d', metric=1255480, timeout=False), MetricValue(zkvm='sp1', program='polybench-jacobi-2d', metric=381548, timeout=False), MetricValue(zkvm='sp1', program='polybench-lu', metric=904461, timeout=False), MetricValue(zkvm='sp1', program='polybench-ludcmp', metric=966364, timeout=False), MetricValue(zkvm='sp1', program='polybench-mvt', metric=448545, timeout=False), MetricValue(zkvm='sp1', program='polybench-nussinov', metric=22789, timeout=False), MetricValue(zkvm='sp1', program='polybench-seidel-2d', metric=2029306, timeout=False), MetricValue(zkvm='sp1', program='polybench-symm', metric=1391124, timeout=False), MetricValue(zkvm='sp1', program='polybench-syr2k', metric=1889530, timeout=False), MetricValue(zkvm='sp1', program='polybench-syrk', metric=762164, timeout=False), MetricValue(zkvm='sp1', program='polybench-trisolv', metric=130314, timeout=False), MetricValue(zkvm='sp1', program='polybench-trmm', metric=619541, timeout=False)]
- o1: [MetricValue(zkvm='risc0', program='polybench-2mm', metric=594779, timeout=False), MetricValue(zkvm='risc0', program='polybench-3mm', metric=855880, timeout=False), MetricValue(zkvm='risc0', program='polybench-adi', metric=2252010, timeout=False), MetricValue(zkvm='risc0', program='polybench-atax', metric=366307, timeout=False), MetricValue(zkvm='risc0', program='polybench-bicg', metric=355810, timeout=False), MetricValue(zkvm='risc0', program='polybench-cholesky', metric=2284353, timeout=False), MetricValue(zkvm='risc0', program='polybench-correlation', metric=776111, timeout=False), MetricValue(zkvm='risc0', program='polybench-covariance', metric=480497, timeout=False), MetricValue(zkvm='risc0', program='polybench-deriche', metric=1823430, timeout=False), MetricValue(zkvm='risc0', program='polybench-doitgen', metric=205519, timeout=False), MetricValue(zkvm='risc0', program='polybench-durbin', metric=147955, timeout=False), MetricValue(zkvm='risc0', program='polybench-fdtd-2d', metric=1123703, timeout=False), MetricValue(zkvm='risc0', program='polybench-floyd-warshall', metric=119570, timeout=False), MetricValue(zkvm='risc0', program='polybench-gemm', metric=598757, timeout=False), MetricValue(zkvm='risc0', program='polybench-gemver', metric=2866179, timeout=False), MetricValue(zkvm='risc0', program='polybench-gesummv', metric=1940192, timeout=False), MetricValue(zkvm='risc0', program='polybench-gramschmidt', metric=557572, timeout=False), MetricValue(zkvm='risc0', program='polybench-heat-3d', metric=801261, timeout=False), MetricValue(zkvm='risc0', program='polybench-jacobi-1d', metric=1599792, timeout=False), MetricValue(zkvm='risc0', program='polybench-jacobi-2d', metric=547539, timeout=False), MetricValue(zkvm='risc0', program='polybench-lu', metric=1138335, timeout=False), MetricValue(zkvm='risc0', program='polybench-ludcmp', metric=1218819, timeout=False), MetricValue(zkvm='risc0', program='polybench-mvt', metric=565391, timeout=False), MetricValue(zkvm='risc0', program='polybench-nussinov', metric=35182, timeout=False), MetricValue(zkvm='risc0', program='polybench-seidel-2d', metric=2518836, timeout=False), MetricValue(zkvm='risc0', program='polybench-symm', metric=1786389, timeout=False), MetricValue(zkvm='risc0', program='polybench-syr2k', metric=2336995, timeout=False), MetricValue(zkvm='risc0', program='polybench-syrk', metric=949716, timeout=False), MetricValue(zkvm='risc0', program='polybench-trisolv', metric=162764, timeout=False), MetricValue(zkvm='risc0', program='polybench-trmm', metric=780027, timeout=False), MetricValue(zkvm='sp1', program='polybench-2mm', metric=486684, timeout=False), MetricValue(zkvm='sp1', program='polybench-3mm', metric=698300, timeout=False), MetricValue(zkvm='sp1', program='polybench-adi', metric=1804637, timeout=False), MetricValue(zkvm='sp1', program='polybench-atax', metric=299239, timeout=False), MetricValue(zkvm='sp1', program='polybench-bicg', metric=291102, timeout=False), MetricValue(zkvm='sp1', program='polybench-cholesky', metric=1876164, timeout=False), MetricValue(zkvm='sp1', program='polybench-correlation', metric=622384, timeout=False), MetricValue(zkvm='sp1', program='polybench-covariance', metric=390844, timeout=False), MetricValue(zkvm='sp1', program='polybench-deriche', metric=1479648, timeout=False), MetricValue(zkvm='sp1', program='polybench-doitgen', metric=171520, timeout=False), MetricValue(zkvm='sp1', program='polybench-durbin', metric=120947, timeout=False), MetricValue(zkvm='sp1', program='polybench-fdtd-2d', metric=874337, timeout=False), MetricValue(zkvm='sp1', program='polybench-floyd-warshall', metric=117883, timeout=False), MetricValue(zkvm='sp1', program='polybench-gemm', metric=490765, timeout=False), MetricValue(zkvm='sp1', program='polybench-gemver', metric=2319255, timeout=False), MetricValue(zkvm='sp1', program='polybench-gesummv', metric=1567731, timeout=False), MetricValue(zkvm='sp1', program='polybench-gramschmidt', metric=450124, timeout=False), MetricValue(zkvm='sp1', program='polybench-heat-3d', metric=641703, timeout=False), MetricValue(zkvm='sp1', program='polybench-jacobi-1d', metric=1282303, timeout=False), MetricValue(zkvm='sp1', program='polybench-jacobi-2d', metric=437881, timeout=False), MetricValue(zkvm='sp1', program='polybench-lu', metric=932629, timeout=False), MetricValue(zkvm='sp1', program='polybench-ludcmp', metric=996261, timeout=False), MetricValue(zkvm='sp1', program='polybench-mvt', metric=461287, timeout=False), MetricValue(zkvm='sp1', program='polybench-nussinov', metric=35884, timeout=False), MetricValue(zkvm='sp1', program='polybench-seidel-2d', metric=1994384, timeout=False), MetricValue(zkvm='sp1', program='polybench-symm', metric=1448429, timeout=False), MetricValue(zkvm='sp1', program='polybench-syr2k', metric=1902090, timeout=False), MetricValue(zkvm='sp1', program='polybench-syrk', metric=772942, timeout=False), MetricValue(zkvm='sp1', program='polybench-trisolv', metric=133111, timeout=False), MetricValue(zkvm='sp1', program='polybench-trmm', metric=635664, timeout=False)]
- o3: [MetricValue(zkvm='risc0', program='polybench-2mm', metric=401200, timeout=False), MetricValue(zkvm='risc0', program='polybench-3mm', metric=736576, timeout=False), MetricValue(zkvm='risc0', program='polybench-adi', metric=1687777, timeout=False), MetricValue(zkvm='risc0', program='polybench-atax', metric=343956, timeout=False), MetricValue(zkvm='risc0', program='polybench-bicg', metric=335671, timeout=False), MetricValue(zkvm='risc0', program='polybench-cholesky', metric=2132847, timeout=False), MetricValue(zkvm='risc0', program='polybench-correlation', metric=737467, timeout=False), MetricValue(zkvm='risc0', program='polybench-covariance', metric=461808, timeout=False), MetricValue(zkvm='risc0', program='polybench-deriche', metric=1742315, timeout=False), MetricValue(zkvm='risc0', program='polybench-doitgen', metric=118538, timeout=False), MetricValue(zkvm='risc0', program='polybench-durbin', metric=141849, timeout=False), MetricValue(zkvm='risc0', program='polybench-fdtd-2d', metric=1109955, timeout=False), MetricValue(zkvm='risc0', program='polybench-floyd-warshall', metric=73677, timeout=False), MetricValue(zkvm='risc0', program='polybench-gemm', metric=370743, timeout=False), MetricValue(zkvm='risc0', program='polybench-gemver', metric=2824322, timeout=False), MetricValue(zkvm='risc0', program='polybench-gesummv', metric=1919858, timeout=False), MetricValue(zkvm='risc0', program='polybench-gramschmidt', metric=481182, timeout=False), MetricValue(zkvm='risc0', program='polybench-heat-3d', metric=731193, timeout=False), MetricValue(zkvm='risc0', program='polybench-jacobi-1d', metric=1573504, timeout=False), MetricValue(zkvm='risc0', program='polybench-jacobi-2d', metric=478817, timeout=False), MetricValue(zkvm='risc0', program='polybench-lu', metric=1070948, timeout=False), MetricValue(zkvm='risc0', program='polybench-ludcmp', metric=1141082, timeout=False), MetricValue(zkvm='risc0', program='polybench-mvt', metric=548070, timeout=False), MetricValue(zkvm='risc0', program='polybench-nussinov', metric=20321, timeout=False), MetricValue(zkvm='risc0', program='polybench-seidel-2d', metric=2551432, timeout=False), MetricValue(zkvm='risc0', program='polybench-symm', metric=1716703, timeout=False), MetricValue(zkvm='risc0', program='polybench-syr2k', metric=2317112, timeout=False), MetricValue(zkvm='risc0', program='polybench-syrk', metric=931644, timeout=False), MetricValue(zkvm='risc0', program='polybench-trisolv', metric=159681, timeout=False), MetricValue(zkvm='risc0', program='polybench-trmm', metric=759445, timeout=False), MetricValue(zkvm='sp1', program='polybench-2mm', metric=328032, timeout=False), MetricValue(zkvm='sp1', program='polybench-3mm', metric=597655, timeout=False), MetricValue(zkvm='sp1', program='polybench-adi', metric=1345838, timeout=False), MetricValue(zkvm='sp1', program='polybench-atax', metric=280559, timeout=False), MetricValue(zkvm='sp1', program='polybench-bicg', metric=274295, timeout=False), MetricValue(zkvm='sp1', program='polybench-cholesky', metric=1734581, timeout=False), MetricValue(zkvm='sp1', program='polybench-correlation', metric=588636, timeout=False), MetricValue(zkvm='sp1', program='polybench-covariance', metric=373985, timeout=False), MetricValue(zkvm='sp1', program='polybench-deriche', metric=1405729, timeout=False), MetricValue(zkvm='sp1', program='polybench-doitgen', metric=99453, timeout=False), MetricValue(zkvm='sp1', program='polybench-durbin', metric=115928, timeout=False), MetricValue(zkvm='sp1', program='polybench-fdtd-2d', metric=860744, timeout=False), MetricValue(zkvm='sp1', program='polybench-floyd-warshall', metric=71476, timeout=False), MetricValue(zkvm='sp1', program='polybench-gemm', metric=303762, timeout=False), MetricValue(zkvm='sp1', program='polybench-gemver', metric=2283724, timeout=False), MetricValue(zkvm='sp1', program='polybench-gesummv', metric=1550483, timeout=False), MetricValue(zkvm='sp1', program='polybench-gramschmidt', metric=386451, timeout=False), MetricValue(zkvm='sp1', program='polybench-heat-3d', metric=582856, timeout=False), MetricValue(zkvm='sp1', program='polybench-jacobi-1d', metric=1259521, timeout=False), MetricValue(zkvm='sp1', program='polybench-jacobi-2d', metric=381548, timeout=False), MetricValue(zkvm='sp1', program='polybench-lu', metric=869802, timeout=False), MetricValue(zkvm='sp1', program='polybench-ludcmp', metric=925084, timeout=False), MetricValue(zkvm='sp1', program='polybench-mvt', metric=445617, timeout=False), MetricValue(zkvm='sp1', program='polybench-nussinov', metric=22409, timeout=False), MetricValue(zkvm='sp1', program='polybench-seidel-2d', metric=2028886, timeout=False), MetricValue(zkvm='sp1', program='polybench-symm', metric=1388890, timeout=False), MetricValue(zkvm='sp1', program='polybench-syr2k', metric=1886053, timeout=False), MetricValue(zkvm='sp1', program='polybench-syrk', metric=758686, timeout=False), MetricValue(zkvm='sp1', program='polybench-trisolv', metric=130314, timeout=False), MetricValue(zkvm='sp1', program='polybench-trmm', metric=617548, timeout=False)]
- o3-lto: [MetricValue(zkvm='risc0', program='polybench-2mm', metric=400513, timeout=False), MetricValue(zkvm='risc0', program='polybench-3mm', metric=735937, timeout=False), MetricValue(zkvm='risc0', program='polybench-adi', metric=1687224, timeout=False), MetricValue(zkvm='risc0', program='polybench-atax', metric=343386, timeout=False), MetricValue(zkvm='risc0', program='polybench-bicg', metric=334973, timeout=False), MetricValue(zkvm='risc0', program='polybench-cholesky', metric=2132146, timeout=False), MetricValue(zkvm='risc0', program='polybench-correlation', metric=736842, timeout=False), MetricValue(zkvm='risc0', program='polybench-covariance', metric=461204, timeout=False), MetricValue(zkvm='risc0', program='polybench-deriche', metric=1741670, timeout=False), MetricValue(zkvm='risc0', program='polybench-doitgen', metric=117978, timeout=False), MetricValue(zkvm='risc0', program='polybench-durbin', metric=141278, timeout=False), MetricValue(zkvm='risc0', program='polybench-fdtd-2d', metric=1109027, timeout=False), MetricValue(zkvm='risc0', program='polybench-floyd-warshall', metric=73143, timeout=False), MetricValue(zkvm='risc0', program='polybench-gemm', metric=370173, timeout=False), MetricValue(zkvm='risc0', program='polybench-gemver', metric=2823766, timeout=False), MetricValue(zkvm='risc0', program='polybench-gesummv', metric=1919270, timeout=False), MetricValue(zkvm='risc0', program='polybench-gramschmidt', metric=480603, timeout=False), MetricValue(zkvm='risc0', program='polybench-heat-3d', metric=729708, timeout=False), MetricValue(zkvm='risc0', program='polybench-jacobi-1d', metric=1573050, timeout=False), MetricValue(zkvm='risc0', program='polybench-jacobi-2d', metric=478095, timeout=False), MetricValue(zkvm='risc0', program='polybench-lu', metric=1070413, timeout=False), MetricValue(zkvm='risc0', program='polybench-ludcmp', metric=1140368, timeout=False), MetricValue(zkvm='risc0', program='polybench-mvt', metric=547462, timeout=False), MetricValue(zkvm='risc0', program='polybench-nussinov', metric=19768, timeout=False), MetricValue(zkvm='risc0', program='polybench-seidel-2d', metric=2550893, timeout=False), MetricValue(zkvm='risc0', program='polybench-symm', metric=1716163, timeout=False), MetricValue(zkvm='risc0', program='polybench-syr2k', metric=2316511, timeout=False), MetricValue(zkvm='risc0', program='polybench-syrk', metric=931049, timeout=False), MetricValue(zkvm='risc0', program='polybench-trisolv', metric=159108, timeout=False), MetricValue(zkvm='risc0', program='polybench-trmm', metric=758878, timeout=False), MetricValue(zkvm='sp1', program='polybench-2mm', metric=327871, timeout=False), MetricValue(zkvm='sp1', program='polybench-3mm', metric=597548, timeout=False), MetricValue(zkvm='sp1', program='polybench-adi', metric=1345803, timeout=False), MetricValue(zkvm='sp1', program='polybench-atax', metric=280507, timeout=False), MetricValue(zkvm='sp1', program='polybench-bicg', metric=274123, timeout=False), MetricValue(zkvm='sp1', program='polybench-cholesky', metric=1734349, timeout=False), MetricValue(zkvm='sp1', program='polybench-correlation', metric=588447, timeout=False), MetricValue(zkvm='sp1', program='polybench-covariance', metric=373901, timeout=False), MetricValue(zkvm='sp1', program='polybench-deriche', metric=1404430, timeout=False), MetricValue(zkvm='sp1', program='polybench-doitgen', metric=99445, timeout=False), MetricValue(zkvm='sp1', program='polybench-durbin', metric=115875, timeout=False), MetricValue(zkvm='sp1', program='polybench-fdtd-2d', metric=859160, timeout=False), MetricValue(zkvm='sp1', program='polybench-floyd-warshall', metric=71456, timeout=False), MetricValue(zkvm='sp1', program='polybench-gemm', metric=303712, timeout=False), MetricValue(zkvm='sp1', program='polybench-gemver', metric=2283709, timeout=False), MetricValue(zkvm='sp1', program='polybench-gesummv', metric=1550421, timeout=False), MetricValue(zkvm='sp1', program='polybench-gramschmidt', metric=386391, timeout=False), MetricValue(zkvm='sp1', program='polybench-heat-3d', metric=581889, timeout=False), MetricValue(zkvm='sp1', program='polybench-jacobi-1d', metric=1259585, timeout=False), MetricValue(zkvm='sp1', program='polybench-jacobi-2d', metric=381361, timeout=False), MetricValue(zkvm='sp1', program='polybench-lu', metric=869764, timeout=False), MetricValue(zkvm='sp1', program='polybench-ludcmp', metric=924893, timeout=False), MetricValue(zkvm='sp1', program='polybench-mvt', metric=445536, timeout=False), MetricValue(zkvm='sp1', program='polybench-nussinov', metric=22374, timeout=False), MetricValue(zkvm='sp1', program='polybench-seidel-2d', metric=2028861, timeout=False), MetricValue(zkvm='sp1', program='polybench-symm', metric=1388870, timeout=False), MetricValue(zkvm='sp1', program='polybench-syr2k', metric=1885974, timeout=False), MetricValue(zkvm='sp1', program='polybench-syrk', metric=758608, timeout=False), MetricValue(zkvm='sp1', program='polybench-trisolv', metric=130261, timeout=False), MetricValue(zkvm='sp1', program='polybench-trmm', metric=617496, timeout=False)]
