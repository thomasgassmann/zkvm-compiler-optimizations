



# Genetic run for metric cycle-count

## Programs

- loop-sum
- factorial
- sha256
- keccak256
- tailcall
- bigmem
- fibonacci
- sha2-bench
- sha2-chain
- regex-match
- sha3-bench
- sha3-chain
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
- merkle
- ecdsa-verify
- eddsa-verify
- rsp
- zkvm-mnist
- spec-605
- spec-619
- npb-bt
- npb-cg
- npb-ep
- npb-ft
- npb-is
- npb-lu
- npb-mg
- npb-sp
- spec-631

## zkVMs

- risc0
- sp1

## Best profile

- Best profile: Profile(profile_name='genetic', rustflags='-C opt-level=3 -C lto=thin -C embed-bitcode -C codegen-units=1 -Cllvm-args=-inline-threshold=19539 -Cllvm-args=-jump-threading-implication-search-threshold=4 -Cllvm-args=-jump-threading-threshold=1 -Cllvm-args=-licm-versioning-max-depth-threshold=3 -Cllvm-args=-loop-distribute-scev-check-threshold=76 -Cllvm-args=-loop-load-elimination-scev-check-threshold=23 -Cllvm-args=-max-dependences=130 -Cllvm-args=-max-num-inline-blocks=7 -Cllvm-args=-max-speculation-depth=15 -Cllvm-args=-memdep-block-scan-limit=263', cflags='-O3 -mllvm -inline-threshold=19539 -mllvm -jump-threading-implication-search-threshold=4 -mllvm -jump-threading-threshold=1 -mllvm -licm-versioning-max-depth-threshold=3 -mllvm -loop-distribute-scev-check-threshold=76 -mllvm -loop-load-elimination-scev-check-threshold=23 -mllvm -max-dependences=130 -mllvm -max-num-inline-blocks=7 -mllvm -max-speculation-depth=15 -mllvm -memdep-block-scan-limit=263', passes=['module(strip-gc-relocates),function(sroa,separate-const-offset-from-gep,simplifycfg,loop-fusion,callsite-splitting,loop-fusion),module(add-discriminators,inline),function(loop(loop-unroll-and-jam),separate-const-offset-from-gep,simplifycfg,mem2reg,loop(loop-rotate),indvars)'], prepopulate_passes=True, lower_atomic_before=False)
- Metric: 1059829438
- Mode: depth-15
- Tune config: TuneConfig(tune_lto=True, tune_codegen_units=True, tune_opt_level=True, tune_prepopulate_passes=False, module_passes=['always-inline', 'inline', 'partial-inliner', 'attributor', 'add-discriminators', 'globalsplit', 'globaldce', 'globalopt', 'wholeprogramdevirt', 'lower-global-dtors', 'strip', 'strip-dead-debug-info', 'strip-dead-prototypes', 'bounds-checking', 'loop-extract', 'mergefunc', 'extract-blocks', 'constmerge', 'deadargelim', 'function-attrs', 'strip-gc-relocates', 'hotcoldsplit', 'argpromotion', 'ipsccp', 'synthetic-counts-propagation', 'rel-lookup-table-converter', 'aggressive-instcombine'], function_passes=['loop-mssa(licm)', 'instcombine', 'bdce', 'correlated-propagation', 'loop-sink', 'loop-data-prefetch', 'loop-fusion', 'mergeicmps', 'mldst-motion', 'newgvn', 'partially-inline-libcalls', 'sroa', 'sink', 'speculative-execution', 'slsr', 'sccp', 'gvn', 'tailcallelim', 'adce', 'dse', 'indvars', 'jump-threading', 'lcssa', 'loop-unroll', 'memcpyopt', 'loop-simplify', 'simplifycfg', 'reassociate', 'mem2reg', 'reg2mem', 'simple-loop-unswitch', 'mergereturn', 'break-crit-edges', 'dce', 'lower-invoke', 'lower-switch', 'callsite-splitting', 'consthoist', 'div-rem-pairs', 'early-cse', 'float2int', 'gvn-hoist', 'gvn-sink', 'guard-widening', 'irce', 'instsimplify', 'libcalls-shrinkwrap', 'nary-reassociate', 'separate-const-offset-from-gep'], loop_passes=['loop-idiom', 'loop-reduce', 'loop-rotate', 'loop-unroll-and-jam', 'loop-unroll-full', 'loop-deletion', 'loop-instsimplify', 'loop-interchange', 'loop-predication', 'loop-versioning-licm'], allowed_opt_levels=['2', '3'], default_prepopulate_passes=True, default_single_codegen_unit=True, allowed_lto=['off', 'thin', 'fat'])

## Overview
  
![genetic-plot](./genetic-plot.png)
## Overview by program

### Program loop-sum
  
![genetic-plot-loop-sum](./genetic-plot-loop-sum.png)
### Program factorial
  
![genetic-plot-factorial](./genetic-plot-factorial.png)
### Program sha256
  
![genetic-plot-sha256](./genetic-plot-sha256.png)
### Program keccak256
  
![genetic-plot-keccak256](./genetic-plot-keccak256.png)
### Program tailcall
  
![genetic-plot-tailcall](./genetic-plot-tailcall.png)
### Program bigmem
  
![genetic-plot-bigmem](./genetic-plot-bigmem.png)
### Program fibonacci
  
![genetic-plot-fibonacci](./genetic-plot-fibonacci.png)
### Program sha2-bench
  
![genetic-plot-sha2-bench](./genetic-plot-sha2-bench.png)
### Program sha2-chain
  
![genetic-plot-sha2-chain](./genetic-plot-sha2-chain.png)
### Program regex-match
  
![genetic-plot-regex-match](./genetic-plot-regex-match.png)
### Program sha3-bench
  
![genetic-plot-sha3-bench](./genetic-plot-sha3-bench.png)
### Program sha3-chain
  
![genetic-plot-sha3-chain](./genetic-plot-sha3-chain.png)
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
### Program merkle
  
![genetic-plot-merkle](./genetic-plot-merkle.png)
### Program ecdsa-verify
  
![genetic-plot-ecdsa-verify](./genetic-plot-ecdsa-verify.png)
### Program eddsa-verify
  
![genetic-plot-eddsa-verify](./genetic-plot-eddsa-verify.png)
### Program rsp
  
![genetic-plot-rsp](./genetic-plot-rsp.png)
### Program zkvm-mnist
  
![genetic-plot-zkvm-mnist](./genetic-plot-zkvm-mnist.png)
### Program spec-605
  
![genetic-plot-spec-605](./genetic-plot-spec-605.png)
### Program spec-619
  
![genetic-plot-spec-619](./genetic-plot-spec-619.png)
### Program npb-bt
  
![genetic-plot-npb-bt](./genetic-plot-npb-bt.png)
### Program npb-cg
  
![genetic-plot-npb-cg](./genetic-plot-npb-cg.png)
### Program npb-ep
  
![genetic-plot-npb-ep](./genetic-plot-npb-ep.png)
### Program npb-ft
  
![genetic-plot-npb-ft](./genetic-plot-npb-ft.png)
### Program npb-is
  
![genetic-plot-npb-is](./genetic-plot-npb-is.png)
### Program npb-lu
  
![genetic-plot-npb-lu](./genetic-plot-npb-lu.png)
### Program npb-mg
  
![genetic-plot-npb-mg](./genetic-plot-npb-mg.png)
### Program npb-sp
  
![genetic-plot-npb-sp](./genetic-plot-npb-sp.png)
### Program spec-631
  
![genetic-plot-spec-631](./genetic-plot-spec-631.png)
## Overview by program group

### Group crypto
  
![genetic-plot-crypto](./genetic-plot-crypto.png)
### Group recursive
  
![genetic-plot-recursive](./genetic-plot-recursive.png)
### Group rust
  
![genetic-plot-rust](./genetic-plot-rust.png)
### Group memory-intensive
  
![genetic-plot-memory-intensive](./genetic-plot-memory-intensive.png)
### Group loop-intensive
  
![genetic-plot-loop-intensive](./genetic-plot-loop-intensive.png)
### Group c
  
![genetic-plot-c](./genetic-plot-c.png)
### Group polybench
  
![genetic-plot-polybench](./genetic-plot-polybench.png)
### Group npb
  
![genetic-plot-npb](./genetic-plot-npb.png)
### Group spec
  
![genetic-plot-spec](./genetic-plot-spec.png)
## Overview by zkVM

### zkVM risc0
  
![genetic-plot-risc0](./genetic-plot-risc0.png)
### zkVM sp1
  
![genetic-plot-sp1](./genetic-plot-sp1.png)
## Baseline values

- o2: [MetricValue(zkvm='risc0', program='loop-sum', metric=117616, timeout=False), MetricValue(zkvm='risc0', program='factorial', metric=2274, timeout=False), MetricValue(zkvm='risc0', program='sha256', metric=209274, timeout=False), MetricValue(zkvm='risc0', program='keccak256', metric=37358, timeout=False), MetricValue(zkvm='risc0', program='tailcall', metric=506540, timeout=False), MetricValue(zkvm='risc0', program='bigmem', metric=770375, timeout=False), MetricValue(zkvm='risc0', program='fibonacci', metric=423498, timeout=False), MetricValue(zkvm='risc0', program='sha2-bench', metric=1390361, timeout=False), MetricValue(zkvm='risc0', program='sha2-chain', metric=41001, timeout=False), MetricValue(zkvm='risc0', program='regex-match', metric=1425870, timeout=False), MetricValue(zkvm='risc0', program='sha3-bench', metric=2171834, timeout=False), MetricValue(zkvm='risc0', program='sha3-chain', metric=136510, timeout=False), MetricValue(zkvm='risc0', program='polybench-2mm', metric=450621, timeout=False), MetricValue(zkvm='risc0', program='polybench-3mm', metric=768620, timeout=False), MetricValue(zkvm='risc0', program='polybench-adi', metric=1684177, timeout=False), MetricValue(zkvm='risc0', program='polybench-atax', metric=357742, timeout=False), MetricValue(zkvm='risc0', program='polybench-bicg', metric=347945, timeout=False), MetricValue(zkvm='risc0', program='polybench-cholesky', metric=2215049, timeout=False), MetricValue(zkvm='risc0', program='polybench-correlation', metric=760076, timeout=False), MetricValue(zkvm='risc0', program='polybench-covariance', metric=461808, timeout=False), MetricValue(zkvm='risc0', program='polybench-deriche', metric=1786829, timeout=False), MetricValue(zkvm='risc0', program='polybench-doitgen', metric=150616, timeout=False), MetricValue(zkvm='risc0', program='polybench-durbin', metric=141849, timeout=False), MetricValue(zkvm='risc0', program='polybench-fdtd-2d', metric=1109541, timeout=False), MetricValue(zkvm='risc0', program='polybench-floyd-warshall', metric=101993, timeout=False), MetricValue(zkvm='risc0', program='polybench-gemm', metric=433002, timeout=False), MetricValue(zkvm='risc0', program='polybench-gemver', metric=2824322, timeout=False), MetricValue(zkvm='risc0', program='polybench-gesummv', metric=1919858, timeout=False), MetricValue(zkvm='risc0', program='polybench-gramschmidt', metric=537758, timeout=False), MetricValue(zkvm='risc0', program='polybench-heat-3d', metric=785316, timeout=False), MetricValue(zkvm='risc0', program='polybench-jacobi-1d', metric=1569463, timeout=False), MetricValue(zkvm='risc0', program='polybench-jacobi-2d', metric=478817, timeout=False), MetricValue(zkvm='risc0', program='polybench-lu', metric=1105607, timeout=False), MetricValue(zkvm='risc0', program='polybench-ludcmp', metric=1184132, timeout=False), MetricValue(zkvm='risc0', program='polybench-mvt', metric=550838, timeout=False), MetricValue(zkvm='risc0', program='polybench-nussinov', metric=20701, timeout=False), MetricValue(zkvm='risc0', program='polybench-seidel-2d', metric=2551852, timeout=False), MetricValue(zkvm='risc0', program='polybench-symm', metric=1719217, timeout=False), MetricValue(zkvm='risc0', program='polybench-syr2k', metric=2321504, timeout=False), MetricValue(zkvm='risc0', program='polybench-syrk', metric=936007, timeout=False), MetricValue(zkvm='risc0', program='polybench-trisolv', metric=159681, timeout=False), MetricValue(zkvm='risc0', program='polybench-trmm', metric=761513, timeout=False), MetricValue(zkvm='risc0', program='merkle', metric=654793, timeout=False), MetricValue(zkvm='risc0', program='ecdsa-verify', metric=235949, timeout=False), MetricValue(zkvm='risc0', program='eddsa-verify', metric=5113785, timeout=False), MetricValue(zkvm='risc0', program='rsp', metric=174635101, timeout=False), MetricValue(zkvm='risc0', program='zkvm-mnist', metric=123208432, timeout=False), MetricValue(zkvm='risc0', program='spec-605', metric=22035023, timeout=False), MetricValue(zkvm='risc0', program='spec-619', metric=32113623, timeout=False), MetricValue(zkvm='risc0', program='npb-bt', metric=65971535, timeout=False), MetricValue(zkvm='risc0', program='npb-cg', metric=23708210, timeout=False), MetricValue(zkvm='risc0', program='npb-ep', metric=52388190, timeout=False), MetricValue(zkvm='risc0', program='npb-ft', metric=23677672, timeout=False), MetricValue(zkvm='risc0', program='npb-is', metric=13854988, timeout=False), MetricValue(zkvm='risc0', program='npb-lu', metric=32590599, timeout=False), MetricValue(zkvm='risc0', program='npb-mg', metric=16744787, timeout=False), MetricValue(zkvm='risc0', program='npb-sp', metric=46789644, timeout=False), MetricValue(zkvm='risc0', program='spec-631', metric=38953538, timeout=False), MetricValue(zkvm='sp1', program='loop-sum', metric=43468, timeout=False), MetricValue(zkvm='sp1', program='factorial', metric=4650, timeout=False), MetricValue(zkvm='sp1', program='sha256', metric=181098, timeout=False), MetricValue(zkvm='sp1', program='keccak256', metric=10256, timeout=False), MetricValue(zkvm='sp1', program='tailcall', metric=471290, timeout=False), MetricValue(zkvm='sp1', program='bigmem', metric=772847, timeout=False), MetricValue(zkvm='sp1', program='fibonacci', metric=365911, timeout=False), MetricValue(zkvm='sp1', program='sha2-bench', metric=627058, timeout=False), MetricValue(zkvm='sp1', program='sha2-chain', metric=154688, timeout=False), MetricValue(zkvm='sp1', program='regex-match', metric=1315270, timeout=False), MetricValue(zkvm='sp1', program='sha3-bench', metric=1153858, timeout=False), MetricValue(zkvm='sp1', program='sha3-chain', metric=552814, timeout=False), MetricValue(zkvm='sp1', program='polybench-2mm', metric=367920, timeout=False), MetricValue(zkvm='sp1', program='polybench-3mm', metric=623557, timeout=False), MetricValue(zkvm='sp1', program='polybench-adi', metric=1342238, timeout=False), MetricValue(zkvm='sp1', program='polybench-atax', metric=291794, timeout=False), MetricValue(zkvm='sp1', program='polybench-bicg', metric=283958, timeout=False), MetricValue(zkvm='sp1', program='polybench-cholesky', metric=1816783, timeout=False), MetricValue(zkvm='sp1', program='polybench-correlation', metric=607831, timeout=False), MetricValue(zkvm='sp1', program='polybench-covariance', metric=373985, timeout=False), MetricValue(zkvm='sp1', program='polybench-deriche', metric=1446965, timeout=False), MetricValue(zkvm='sp1', program='polybench-doitgen', metric=125411, timeout=False), MetricValue(zkvm='sp1', program='polybench-durbin', metric=115928, timeout=False), MetricValue(zkvm='sp1', program='polybench-fdtd-2d', metric=861915, timeout=False), MetricValue(zkvm='sp1', program='polybench-floyd-warshall', metric=99792, timeout=False), MetricValue(zkvm='sp1', program='polybench-gemm', metric=354108, timeout=False), MetricValue(zkvm='sp1', program='polybench-gemver', metric=2283724, timeout=False), MetricValue(zkvm='sp1', program='polybench-gesummv', metric=1550483, timeout=False), MetricValue(zkvm='sp1', program='polybench-gramschmidt', metric=432066, timeout=False), MetricValue(zkvm='sp1', program='polybench-heat-3d', metric=626255, timeout=False), MetricValue(zkvm='sp1', program='polybench-jacobi-1d', metric=1255480, timeout=False), MetricValue(zkvm='sp1', program='polybench-jacobi-2d', metric=381548, timeout=False), MetricValue(zkvm='sp1', program='polybench-lu', metric=904461, timeout=False), MetricValue(zkvm='sp1', program='polybench-ludcmp', metric=966364, timeout=False), MetricValue(zkvm='sp1', program='polybench-mvt', metric=448545, timeout=False), MetricValue(zkvm='sp1', program='polybench-nussinov', metric=22789, timeout=False), MetricValue(zkvm='sp1', program='polybench-seidel-2d', metric=2029306, timeout=False), MetricValue(zkvm='sp1', program='polybench-symm', metric=1391124, timeout=False), MetricValue(zkvm='sp1', program='polybench-syr2k', metric=1889530, timeout=False), MetricValue(zkvm='sp1', program='polybench-syrk', metric=762164, timeout=False), MetricValue(zkvm='sp1', program='polybench-trisolv', metric=130314, timeout=False), MetricValue(zkvm='sp1', program='polybench-trmm', metric=619541, timeout=False), MetricValue(zkvm='sp1', program='merkle', metric=454151, timeout=False), MetricValue(zkvm='sp1', program='ecdsa-verify', metric=4685127, timeout=False), MetricValue(zkvm='sp1', program='eddsa-verify', metric=2193153, timeout=False), MetricValue(zkvm='sp1', program='rsp', metric=60214784, timeout=False), MetricValue(zkvm='sp1', program='zkvm-mnist', metric=99544769, timeout=False), MetricValue(zkvm='sp1', program='spec-605', metric=22021455, timeout=False), MetricValue(zkvm='sp1', program='spec-619', metric=25711397, timeout=False), MetricValue(zkvm='sp1', program='npb-bt', metric=53345974, timeout=False), MetricValue(zkvm='sp1', program='npb-cg', metric=19190281, timeout=False), MetricValue(zkvm='sp1', program='npb-ep', metric=41722254, timeout=False), MetricValue(zkvm='sp1', program='npb-ft', metric=18978175, timeout=False), MetricValue(zkvm='sp1', program='npb-is', metric=11079374, timeout=False), MetricValue(zkvm='sp1', program='npb-lu', metric=26201461, timeout=False), MetricValue(zkvm='sp1', program='npb-mg', metric=13379826, timeout=False), MetricValue(zkvm='sp1', program='npb-sp', metric=37644958, timeout=False), MetricValue(zkvm='sp1', program='spec-631', metric=33732578, timeout=False)]
- o1: [MetricValue(zkvm='risc0', program='loop-sum', metric=138324, timeout=False), MetricValue(zkvm='risc0', program='factorial', metric=3419, timeout=False), MetricValue(zkvm='risc0', program='sha256', metric=228517, timeout=False), MetricValue(zkvm='risc0', program='keccak256', metric=47100, timeout=False), MetricValue(zkvm='risc0', program='tailcall', metric=715002, timeout=False), MetricValue(zkvm='risc0', program='bigmem', metric=771554, timeout=False), MetricValue(zkvm='risc0', program='fibonacci', metric=514674, timeout=False), MetricValue(zkvm='risc0', program='sha2-bench', metric=1548591, timeout=False), MetricValue(zkvm='risc0', program='sha2-chain', metric=49691, timeout=False), MetricValue(zkvm='risc0', program='regex-match', metric=1638383, timeout=False), MetricValue(zkvm='risc0', program='sha3-bench', metric=2672522, timeout=False), MetricValue(zkvm='risc0', program='sha3-chain', metric=174440, timeout=False), MetricValue(zkvm='risc0', program='polybench-2mm', metric=594779, timeout=False), MetricValue(zkvm='risc0', program='polybench-3mm', metric=855880, timeout=False), MetricValue(zkvm='risc0', program='polybench-adi', metric=2252010, timeout=False), MetricValue(zkvm='risc0', program='polybench-atax', metric=366307, timeout=False), MetricValue(zkvm='risc0', program='polybench-bicg', metric=355810, timeout=False), MetricValue(zkvm='risc0', program='polybench-cholesky', metric=2284353, timeout=False), MetricValue(zkvm='risc0', program='polybench-correlation', metric=776111, timeout=False), MetricValue(zkvm='risc0', program='polybench-covariance', metric=480497, timeout=False), MetricValue(zkvm='risc0', program='polybench-deriche', metric=1823430, timeout=False), MetricValue(zkvm='risc0', program='polybench-doitgen', metric=205519, timeout=False), MetricValue(zkvm='risc0', program='polybench-durbin', metric=147955, timeout=False), MetricValue(zkvm='risc0', program='polybench-fdtd-2d', metric=1123703, timeout=False), MetricValue(zkvm='risc0', program='polybench-floyd-warshall', metric=119570, timeout=False), MetricValue(zkvm='risc0', program='polybench-gemm', metric=598757, timeout=False), MetricValue(zkvm='risc0', program='polybench-gemver', metric=2866179, timeout=False), MetricValue(zkvm='risc0', program='polybench-gesummv', metric=1940192, timeout=False), MetricValue(zkvm='risc0', program='polybench-gramschmidt', metric=557572, timeout=False), MetricValue(zkvm='risc0', program='polybench-heat-3d', metric=801261, timeout=False), MetricValue(zkvm='risc0', program='polybench-jacobi-1d', metric=1599792, timeout=False), MetricValue(zkvm='risc0', program='polybench-jacobi-2d', metric=547539, timeout=False), MetricValue(zkvm='risc0', program='polybench-lu', metric=1138335, timeout=False), MetricValue(zkvm='risc0', program='polybench-ludcmp', metric=1218819, timeout=False), MetricValue(zkvm='risc0', program='polybench-mvt', metric=565391, timeout=False), MetricValue(zkvm='risc0', program='polybench-nussinov', metric=35182, timeout=False), MetricValue(zkvm='risc0', program='polybench-seidel-2d', metric=2518836, timeout=False), MetricValue(zkvm='risc0', program='polybench-symm', metric=1786389, timeout=False), MetricValue(zkvm='risc0', program='polybench-syr2k', metric=2336995, timeout=False), MetricValue(zkvm='risc0', program='polybench-syrk', metric=949716, timeout=False), MetricValue(zkvm='risc0', program='polybench-trisolv', metric=162764, timeout=False), MetricValue(zkvm='risc0', program='polybench-trmm', metric=780027, timeout=False), MetricValue(zkvm='risc0', program='merkle', metric=726852, timeout=False), MetricValue(zkvm='risc0', program='ecdsa-verify', metric=256326, timeout=False), MetricValue(zkvm='risc0', program='eddsa-verify', metric=5470252, timeout=False), MetricValue(zkvm='risc0', program='rsp', metric=245484658, timeout=False), MetricValue(zkvm='risc0', program='zkvm-mnist', metric=124423285, timeout=False), MetricValue(zkvm='risc0', program='spec-605', metric=22057504, timeout=False), MetricValue(zkvm='risc0', program='spec-619', metric=32160872, timeout=False), MetricValue(zkvm='risc0', program='npb-bt', metric=66137375, timeout=False), MetricValue(zkvm='risc0', program='npb-cg', metric=23803882, timeout=False), MetricValue(zkvm='risc0', program='npb-ep', metric=52490595, timeout=False), MetricValue(zkvm='risc0', program='npb-ft', metric=24905668, timeout=False), MetricValue(zkvm='risc0', program='npb-is', metric=13877060, timeout=False), MetricValue(zkvm='risc0', program='npb-lu', metric=32610790, timeout=False), MetricValue(zkvm='risc0', program='npb-mg', metric=16821695, timeout=False), MetricValue(zkvm='risc0', program='npb-sp', metric=46881068, timeout=False), MetricValue(zkvm='risc0', program='spec-631', metric=40225711, timeout=False), MetricValue(zkvm='sp1', program='loop-sum', metric=109334, timeout=False), MetricValue(zkvm='sp1', program='factorial', metric=5966, timeout=False), MetricValue(zkvm='sp1', program='sha256', metric=200465, timeout=False), MetricValue(zkvm='sp1', program='keccak256', metric=16276, timeout=False), MetricValue(zkvm='sp1', program='tailcall', metric=651747, timeout=False), MetricValue(zkvm='sp1', program='bigmem', metric=774108, timeout=False), MetricValue(zkvm='sp1', program='fibonacci', metric=427172, timeout=False), MetricValue(zkvm='sp1', program='sha2-bench', metric=1086004, timeout=False), MetricValue(zkvm='sp1', program='sha2-chain', metric=190401, timeout=False), MetricValue(zkvm='sp1', program='regex-match', metric=1495001, timeout=False), MetricValue(zkvm='sp1', program='sha3-bench', metric=1943720, timeout=False), MetricValue(zkvm='sp1', program='sha3-chain', metric=771755, timeout=False), MetricValue(zkvm='sp1', program='polybench-2mm', metric=486684, timeout=False), MetricValue(zkvm='sp1', program='polybench-3mm', metric=698300, timeout=False), MetricValue(zkvm='sp1', program='polybench-adi', metric=1804637, timeout=False), MetricValue(zkvm='sp1', program='polybench-atax', metric=299239, timeout=False), MetricValue(zkvm='sp1', program='polybench-bicg', metric=291102, timeout=False), MetricValue(zkvm='sp1', program='polybench-cholesky', metric=1876164, timeout=False), MetricValue(zkvm='sp1', program='polybench-correlation', metric=622384, timeout=False), MetricValue(zkvm='sp1', program='polybench-covariance', metric=390844, timeout=False), MetricValue(zkvm='sp1', program='polybench-deriche', metric=1479648, timeout=False), MetricValue(zkvm='sp1', program='polybench-doitgen', metric=171520, timeout=False), MetricValue(zkvm='sp1', program='polybench-durbin', metric=120947, timeout=False), MetricValue(zkvm='sp1', program='polybench-fdtd-2d', metric=874337, timeout=False), MetricValue(zkvm='sp1', program='polybench-floyd-warshall', metric=117883, timeout=False), MetricValue(zkvm='sp1', program='polybench-gemm', metric=490765, timeout=False), MetricValue(zkvm='sp1', program='polybench-gemver', metric=2319255, timeout=False), MetricValue(zkvm='sp1', program='polybench-gesummv', metric=1567731, timeout=False), MetricValue(zkvm='sp1', program='polybench-gramschmidt', metric=450124, timeout=False), MetricValue(zkvm='sp1', program='polybench-heat-3d', metric=641703, timeout=False), MetricValue(zkvm='sp1', program='polybench-jacobi-1d', metric=1282303, timeout=False), MetricValue(zkvm='sp1', program='polybench-jacobi-2d', metric=437881, timeout=False), MetricValue(zkvm='sp1', program='polybench-lu', metric=932629, timeout=False), MetricValue(zkvm='sp1', program='polybench-ludcmp', metric=996261, timeout=False), MetricValue(zkvm='sp1', program='polybench-mvt', metric=461287, timeout=False), MetricValue(zkvm='sp1', program='polybench-nussinov', metric=35884, timeout=False), MetricValue(zkvm='sp1', program='polybench-seidel-2d', metric=1994384, timeout=False), MetricValue(zkvm='sp1', program='polybench-symm', metric=1448429, timeout=False), MetricValue(zkvm='sp1', program='polybench-syr2k', metric=1902090, timeout=False), MetricValue(zkvm='sp1', program='polybench-syrk', metric=772942, timeout=False), MetricValue(zkvm='sp1', program='polybench-trisolv', metric=133111, timeout=False), MetricValue(zkvm='sp1', program='polybench-trmm', metric=635664, timeout=False), MetricValue(zkvm='sp1', program='merkle', metric=516560, timeout=False), MetricValue(zkvm='sp1', program='ecdsa-verify', metric=4627722, timeout=False), MetricValue(zkvm='sp1', program='eddsa-verify', metric=2630724, timeout=False), MetricValue(zkvm='sp1', program='rsp', metric=79295930, timeout=False), MetricValue(zkvm='sp1', program='zkvm-mnist', metric=101160109, timeout=False), MetricValue(zkvm='sp1', program='spec-605', metric=22041398, timeout=False), MetricValue(zkvm='sp1', program='spec-619', metric=25757898, timeout=False), MetricValue(zkvm='sp1', program='npb-bt', metric=53494263, timeout=False), MetricValue(zkvm='sp1', program='npb-cg', metric=19340136, timeout=False), MetricValue(zkvm='sp1', program='npb-ep', metric=41824857, timeout=False), MetricValue(zkvm='sp1', program='npb-ft', metric=19921536, timeout=False), MetricValue(zkvm='sp1', program='npb-is', metric=11101596, timeout=False), MetricValue(zkvm='sp1', program='npb-lu', metric=26218227, timeout=False), MetricValue(zkvm='sp1', program='npb-mg', metric=13459342, timeout=False), MetricValue(zkvm='sp1', program='npb-sp', metric=37719051, timeout=False), MetricValue(zkvm='sp1', program='spec-631', metric=34738576, timeout=False)]
- o3: [MetricValue(zkvm='risc0', program='loop-sum', metric=117613, timeout=False), MetricValue(zkvm='risc0', program='factorial', metric=2274, timeout=False), MetricValue(zkvm='risc0', program='sha256', metric=207320, timeout=False), MetricValue(zkvm='risc0', program='keccak256', metric=37313, timeout=False), MetricValue(zkvm='risc0', program='tailcall', metric=506540, timeout=False), MetricValue(zkvm='risc0', program='bigmem', metric=770375, timeout=False), MetricValue(zkvm='risc0', program='fibonacci', metric=423498, timeout=False), MetricValue(zkvm='risc0', program='sha2-bench', metric=1388003, timeout=False), MetricValue(zkvm='risc0', program='sha2-chain', metric=40973, timeout=False), MetricValue(zkvm='risc0', program='regex-match', metric=1392885, timeout=False), MetricValue(zkvm='risc0', program='sha3-bench', metric=2168376, timeout=False), MetricValue(zkvm='risc0', program='sha3-chain', metric=136300, timeout=False), MetricValue(zkvm='risc0', program='polybench-2mm', metric=401200, timeout=False), MetricValue(zkvm='risc0', program='polybench-3mm', metric=736576, timeout=False), MetricValue(zkvm='risc0', program='polybench-adi', metric=1687777, timeout=False), MetricValue(zkvm='risc0', program='polybench-atax', metric=343956, timeout=False), MetricValue(zkvm='risc0', program='polybench-bicg', metric=335671, timeout=False), MetricValue(zkvm='risc0', program='polybench-cholesky', metric=2132847, timeout=False), MetricValue(zkvm='risc0', program='polybench-correlation', metric=737467, timeout=False), MetricValue(zkvm='risc0', program='polybench-covariance', metric=461808, timeout=False), MetricValue(zkvm='risc0', program='polybench-deriche', metric=1742315, timeout=False), MetricValue(zkvm='risc0', program='polybench-doitgen', metric=118538, timeout=False), MetricValue(zkvm='risc0', program='polybench-durbin', metric=141849, timeout=False), MetricValue(zkvm='risc0', program='polybench-fdtd-2d', metric=1109955, timeout=False), MetricValue(zkvm='risc0', program='polybench-floyd-warshall', metric=73677, timeout=False), MetricValue(zkvm='risc0', program='polybench-gemm', metric=370743, timeout=False), MetricValue(zkvm='risc0', program='polybench-gemver', metric=2824322, timeout=False), MetricValue(zkvm='risc0', program='polybench-gesummv', metric=1919858, timeout=False), MetricValue(zkvm='risc0', program='polybench-gramschmidt', metric=481182, timeout=False), MetricValue(zkvm='risc0', program='polybench-heat-3d', metric=731193, timeout=False), MetricValue(zkvm='risc0', program='polybench-jacobi-1d', metric=1573504, timeout=False), MetricValue(zkvm='risc0', program='polybench-jacobi-2d', metric=478817, timeout=False), MetricValue(zkvm='risc0', program='polybench-lu', metric=1070948, timeout=False), MetricValue(zkvm='risc0', program='polybench-ludcmp', metric=1141082, timeout=False), MetricValue(zkvm='risc0', program='polybench-mvt', metric=548070, timeout=False), MetricValue(zkvm='risc0', program='polybench-nussinov', metric=20321, timeout=False), MetricValue(zkvm='risc0', program='polybench-seidel-2d', metric=2551432, timeout=False), MetricValue(zkvm='risc0', program='polybench-symm', metric=1716703, timeout=False), MetricValue(zkvm='risc0', program='polybench-syr2k', metric=2317112, timeout=False), MetricValue(zkvm='risc0', program='polybench-syrk', metric=931644, timeout=False), MetricValue(zkvm='risc0', program='polybench-trisolv', metric=159681, timeout=False), MetricValue(zkvm='risc0', program='polybench-trmm', metric=759445, timeout=False), MetricValue(zkvm='risc0', program='merkle', metric=654564, timeout=False), MetricValue(zkvm='risc0', program='ecdsa-verify', metric=233316, timeout=False), MetricValue(zkvm='risc0', program='eddsa-verify', metric=5091191, timeout=False), MetricValue(zkvm='risc0', program='rsp', metric=174086577, timeout=False), MetricValue(zkvm='risc0', program='zkvm-mnist', metric=123068947, timeout=False), MetricValue(zkvm='risc0', program='spec-605', metric=22032683, timeout=False), MetricValue(zkvm='risc0', program='spec-619', metric=32112544, timeout=False), MetricValue(zkvm='risc0', program='npb-bt', metric=65820034, timeout=False), MetricValue(zkvm='risc0', program='npb-cg', metric=23680624, timeout=False), MetricValue(zkvm='risc0', program='npb-ep', metric=52345743, timeout=False), MetricValue(zkvm='risc0', program='npb-ft', metric=23497181, timeout=False), MetricValue(zkvm='risc0', program='npb-is', metric=13854473, timeout=False), MetricValue(zkvm='risc0', program='npb-lu', metric=30242084, timeout=False), MetricValue(zkvm='risc0', program='npb-mg', metric=16434270, timeout=False), MetricValue(zkvm='risc0', program='npb-sp', metric=46628552, timeout=False), MetricValue(zkvm='risc0', program='spec-631', metric=38870349, timeout=False), MetricValue(zkvm='sp1', program='loop-sum', metric=43464, timeout=False), MetricValue(zkvm='sp1', program='factorial', metric=4650, timeout=False), MetricValue(zkvm='sp1', program='sha256', metric=179144, timeout=False), MetricValue(zkvm='sp1', program='keccak256', metric=10216, timeout=False), MetricValue(zkvm='sp1', program='tailcall', metric=471288, timeout=False), MetricValue(zkvm='sp1', program='bigmem', metric=772847, timeout=False), MetricValue(zkvm='sp1', program='fibonacci', metric=365911, timeout=False), MetricValue(zkvm='sp1', program='sha2-bench', metric=626968, timeout=False), MetricValue(zkvm='sp1', program='sha2-chain', metric=154688, timeout=False), MetricValue(zkvm='sp1', program='regex-match', metric=1290388, timeout=False), MetricValue(zkvm='sp1', program='sha3-bench', metric=1150639, timeout=False), MetricValue(zkvm='sp1', program='sha3-chain', metric=551572, timeout=False), MetricValue(zkvm='sp1', program='polybench-2mm', metric=328032, timeout=False), MetricValue(zkvm='sp1', program='polybench-3mm', metric=597655, timeout=False), MetricValue(zkvm='sp1', program='polybench-adi', metric=1345838, timeout=False), MetricValue(zkvm='sp1', program='polybench-atax', metric=280559, timeout=False), MetricValue(zkvm='sp1', program='polybench-bicg', metric=274295, timeout=False), MetricValue(zkvm='sp1', program='polybench-cholesky', metric=1734581, timeout=False), MetricValue(zkvm='sp1', program='polybench-correlation', metric=588636, timeout=False), MetricValue(zkvm='sp1', program='polybench-covariance', metric=373985, timeout=False), MetricValue(zkvm='sp1', program='polybench-deriche', metric=1405729, timeout=False), MetricValue(zkvm='sp1', program='polybench-doitgen', metric=99453, timeout=False), MetricValue(zkvm='sp1', program='polybench-durbin', metric=115928, timeout=False), MetricValue(zkvm='sp1', program='polybench-fdtd-2d', metric=860744, timeout=False), MetricValue(zkvm='sp1', program='polybench-floyd-warshall', metric=71476, timeout=False), MetricValue(zkvm='sp1', program='polybench-gemm', metric=303762, timeout=False), MetricValue(zkvm='sp1', program='polybench-gemver', metric=2283724, timeout=False), MetricValue(zkvm='sp1', program='polybench-gesummv', metric=1550483, timeout=False), MetricValue(zkvm='sp1', program='polybench-gramschmidt', metric=386451, timeout=False), MetricValue(zkvm='sp1', program='polybench-heat-3d', metric=582856, timeout=False), MetricValue(zkvm='sp1', program='polybench-jacobi-1d', metric=1259521, timeout=False), MetricValue(zkvm='sp1', program='polybench-jacobi-2d', metric=381548, timeout=False), MetricValue(zkvm='sp1', program='polybench-lu', metric=869802, timeout=False), MetricValue(zkvm='sp1', program='polybench-ludcmp', metric=925084, timeout=False), MetricValue(zkvm='sp1', program='polybench-mvt', metric=445617, timeout=False), MetricValue(zkvm='sp1', program='polybench-nussinov', metric=22409, timeout=False), MetricValue(zkvm='sp1', program='polybench-seidel-2d', metric=2028886, timeout=False), MetricValue(zkvm='sp1', program='polybench-symm', metric=1388890, timeout=False), MetricValue(zkvm='sp1', program='polybench-syr2k', metric=1886053, timeout=False), MetricValue(zkvm='sp1', program='polybench-syrk', metric=758686, timeout=False), MetricValue(zkvm='sp1', program='polybench-trisolv', metric=130314, timeout=False), MetricValue(zkvm='sp1', program='polybench-trmm', metric=617548, timeout=False), MetricValue(zkvm='sp1', program='merkle', metric=453875, timeout=False), MetricValue(zkvm='sp1', program='ecdsa-verify', metric=4675762, timeout=False), MetricValue(zkvm='sp1', program='eddsa-verify', metric=2193097, timeout=False), MetricValue(zkvm='sp1', program='rsp', metric=59700255, timeout=False), MetricValue(zkvm='sp1', program='zkvm-mnist', metric=99405262, timeout=False), MetricValue(zkvm='sp1', program='spec-605', metric=22019212, timeout=False), MetricValue(zkvm='sp1', program='spec-619', metric=25710322, timeout=False), MetricValue(zkvm='sp1', program='npb-bt', metric=53213671, timeout=False), MetricValue(zkvm='sp1', program='npb-cg', metric=19169113, timeout=False), MetricValue(zkvm='sp1', program='npb-ep', metric=41688116, timeout=False), MetricValue(zkvm='sp1', program='npb-ft', metric=18817691, timeout=False), MetricValue(zkvm='sp1', program='npb-is', metric=11078859, timeout=False), MetricValue(zkvm='sp1', program='npb-lu', metric=24284905, timeout=False), MetricValue(zkvm='sp1', program='npb-mg', metric=13117592, timeout=False), MetricValue(zkvm='sp1', program='npb-sp', metric=37503578, timeout=False), MetricValue(zkvm='sp1', program='spec-631', metric=33663873, timeout=False)]
- o3-lto: [MetricValue(zkvm='risc0', program='loop-sum', metric=68831, timeout=False), MetricValue(zkvm='risc0', program='factorial', metric=1718, timeout=False), MetricValue(zkvm='risc0', program='sha256', metric=205414, timeout=False), MetricValue(zkvm='risc0', program='keccak256', metric=32672, timeout=False), MetricValue(zkvm='risc0', program='tailcall', metric=505442, timeout=False), MetricValue(zkvm='risc0', program='bigmem', metric=769803, timeout=False), MetricValue(zkvm='risc0', program='fibonacci', metric=422717, timeout=False), MetricValue(zkvm='risc0', program='sha2-bench', metric=1108374, timeout=False), MetricValue(zkvm='risc0', program='sha2-chain', metric=37766, timeout=False), MetricValue(zkvm='risc0', program='regex-match', metric=1315853, timeout=False), MetricValue(zkvm='risc0', program='sha3-bench', metric=1880280, timeout=False), MetricValue(zkvm='risc0', program='sha3-chain', metric=134257, timeout=False), MetricValue(zkvm='risc0', program='polybench-2mm', metric=400513, timeout=False), MetricValue(zkvm='risc0', program='polybench-3mm', metric=735937, timeout=False), MetricValue(zkvm='risc0', program='polybench-adi', metric=1687224, timeout=False), MetricValue(zkvm='risc0', program='polybench-atax', metric=343386, timeout=False), MetricValue(zkvm='risc0', program='polybench-bicg', metric=334973, timeout=False), MetricValue(zkvm='risc0', program='polybench-cholesky', metric=2132146, timeout=False), MetricValue(zkvm='risc0', program='polybench-correlation', metric=736842, timeout=False), MetricValue(zkvm='risc0', program='polybench-covariance', metric=461204, timeout=False), MetricValue(zkvm='risc0', program='polybench-deriche', metric=1741670, timeout=False), MetricValue(zkvm='risc0', program='polybench-doitgen', metric=117978, timeout=False), MetricValue(zkvm='risc0', program='polybench-durbin', metric=141278, timeout=False), MetricValue(zkvm='risc0', program='polybench-fdtd-2d', metric=1109027, timeout=False), MetricValue(zkvm='risc0', program='polybench-floyd-warshall', metric=73143, timeout=False), MetricValue(zkvm='risc0', program='polybench-gemm', metric=370173, timeout=False), MetricValue(zkvm='risc0', program='polybench-gemver', metric=2823766, timeout=False), MetricValue(zkvm='risc0', program='polybench-gesummv', metric=1919270, timeout=False), MetricValue(zkvm='risc0', program='polybench-gramschmidt', metric=480603, timeout=False), MetricValue(zkvm='risc0', program='polybench-heat-3d', metric=729708, timeout=False), MetricValue(zkvm='risc0', program='polybench-jacobi-1d', metric=1573050, timeout=False), MetricValue(zkvm='risc0', program='polybench-jacobi-2d', metric=478095, timeout=False), MetricValue(zkvm='risc0', program='polybench-lu', metric=1070413, timeout=False), MetricValue(zkvm='risc0', program='polybench-ludcmp', metric=1140368, timeout=False), MetricValue(zkvm='risc0', program='polybench-mvt', metric=547462, timeout=False), MetricValue(zkvm='risc0', program='polybench-nussinov', metric=19768, timeout=False), MetricValue(zkvm='risc0', program='polybench-seidel-2d', metric=2550893, timeout=False), MetricValue(zkvm='risc0', program='polybench-symm', metric=1716163, timeout=False), MetricValue(zkvm='risc0', program='polybench-syr2k', metric=2316511, timeout=False), MetricValue(zkvm='risc0', program='polybench-syrk', metric=931049, timeout=False), MetricValue(zkvm='risc0', program='polybench-trisolv', metric=159108, timeout=False), MetricValue(zkvm='risc0', program='polybench-trmm', metric=758878, timeout=False), MetricValue(zkvm='risc0', program='merkle', metric=638640, timeout=False), MetricValue(zkvm='risc0', program='ecdsa-verify', metric=210004, timeout=False), MetricValue(zkvm='risc0', program='eddsa-verify', metric=4430616, timeout=False), MetricValue(zkvm='risc0', program='rsp', metric=134222172, timeout=False), MetricValue(zkvm='risc0', program='zkvm-mnist', metric=121709356, timeout=False), MetricValue(zkvm='risc0', program='spec-605', metric=22022080, timeout=False), MetricValue(zkvm='risc0', program='spec-619', metric=32104743, timeout=False), MetricValue(zkvm='risc0', program='npb-bt', metric=65885661, timeout=False), MetricValue(zkvm='risc0', program='npb-cg', metric=23619619, timeout=False), MetricValue(zkvm='risc0', program='npb-ep', metric=52315453, timeout=False), MetricValue(zkvm='risc0', program='npb-ft', metric=23392514, timeout=False), MetricValue(zkvm='risc0', program='npb-is', metric=13851615, timeout=False), MetricValue(zkvm='risc0', program='npb-lu', metric=30233592, timeout=False), MetricValue(zkvm='risc0', program='npb-mg', metric=16319182, timeout=False), MetricValue(zkvm='risc0', program='npb-sp', metric=46693865, timeout=False), MetricValue(zkvm='risc0', program='spec-631', metric=38776933, timeout=False), MetricValue(zkvm='sp1', program='loop-sum', metric=43157, timeout=False), MetricValue(zkvm='sp1', program='factorial', metric=4597, timeout=False), MetricValue(zkvm='sp1', program='sha256', metric=177244, timeout=False), MetricValue(zkvm='sp1', program='keccak256', metric=6748, timeout=False), MetricValue(zkvm='sp1', program='tailcall', metric=470644, timeout=False), MetricValue(zkvm='sp1', program='bigmem', metric=772727, timeout=False), MetricValue(zkvm='sp1', program='fibonacci', metric=365577, timeout=False), MetricValue(zkvm='sp1', program='sha2-bench', metric=583127, timeout=False), MetricValue(zkvm='sp1', program='sha2-chain', metric=149465, timeout=False), MetricValue(zkvm='sp1', program='regex-match', metric=1209980, timeout=False), MetricValue(zkvm='sp1', program='sha3-bench', metric=1098163, timeout=False), MetricValue(zkvm='sp1', program='sha3-chain', metric=550806, timeout=False), MetricValue(zkvm='sp1', program='polybench-2mm', metric=327871, timeout=False), MetricValue(zkvm='sp1', program='polybench-3mm', metric=597548, timeout=False), MetricValue(zkvm='sp1', program='polybench-adi', metric=1345803, timeout=False), MetricValue(zkvm='sp1', program='polybench-atax', metric=280507, timeout=False), MetricValue(zkvm='sp1', program='polybench-bicg', metric=274123, timeout=False), MetricValue(zkvm='sp1', program='polybench-cholesky', metric=1734349, timeout=False), MetricValue(zkvm='sp1', program='polybench-correlation', metric=588447, timeout=False), MetricValue(zkvm='sp1', program='polybench-covariance', metric=373901, timeout=False), MetricValue(zkvm='sp1', program='polybench-deriche', metric=1404430, timeout=False), MetricValue(zkvm='sp1', program='polybench-doitgen', metric=99445, timeout=False), MetricValue(zkvm='sp1', program='polybench-durbin', metric=115875, timeout=False), MetricValue(zkvm='sp1', program='polybench-fdtd-2d', metric=859160, timeout=False), MetricValue(zkvm='sp1', program='polybench-floyd-warshall', metric=71456, timeout=False), MetricValue(zkvm='sp1', program='polybench-gemm', metric=303712, timeout=False), MetricValue(zkvm='sp1', program='polybench-gemver', metric=2283709, timeout=False), MetricValue(zkvm='sp1', program='polybench-gesummv', metric=1550421, timeout=False), MetricValue(zkvm='sp1', program='polybench-gramschmidt', metric=386391, timeout=False), MetricValue(zkvm='sp1', program='polybench-heat-3d', metric=581889, timeout=False), MetricValue(zkvm='sp1', program='polybench-jacobi-1d', metric=1259585, timeout=False), MetricValue(zkvm='sp1', program='polybench-jacobi-2d', metric=381361, timeout=False), MetricValue(zkvm='sp1', program='polybench-lu', metric=869764, timeout=False), MetricValue(zkvm='sp1', program='polybench-ludcmp', metric=924893, timeout=False), MetricValue(zkvm='sp1', program='polybench-mvt', metric=445536, timeout=False), MetricValue(zkvm='sp1', program='polybench-nussinov', metric=22374, timeout=False), MetricValue(zkvm='sp1', program='polybench-seidel-2d', metric=2028861, timeout=False), MetricValue(zkvm='sp1', program='polybench-symm', metric=1388870, timeout=False), MetricValue(zkvm='sp1', program='polybench-syr2k', metric=1885974, timeout=False), MetricValue(zkvm='sp1', program='polybench-syrk', metric=758608, timeout=False), MetricValue(zkvm='sp1', program='polybench-trisolv', metric=130261, timeout=False), MetricValue(zkvm='sp1', program='polybench-trmm', metric=617496, timeout=False), MetricValue(zkvm='sp1', program='merkle', metric=441844, timeout=False), MetricValue(zkvm='sp1', program='ecdsa-verify', metric=4326266, timeout=False), MetricValue(zkvm='sp1', program='eddsa-verify', metric=2116029, timeout=False), MetricValue(zkvm='sp1', program='rsp', metric=56825052, timeout=False), MetricValue(zkvm='sp1', program='zkvm-mnist', metric=98642480, timeout=False), MetricValue(zkvm='sp1', program='spec-605', metric=22009559, timeout=False), MetricValue(zkvm='sp1', program='spec-619', metric=25702814, timeout=False), MetricValue(zkvm='sp1', program='npb-bt', metric=53262904, timeout=False), MetricValue(zkvm='sp1', program='npb-cg', metric=19106630, timeout=False), MetricValue(zkvm='sp1', program='npb-ep', metric=41654255, timeout=False), MetricValue(zkvm='sp1', program='npb-ft', metric=18734601, timeout=False), MetricValue(zkvm='sp1', program='npb-is', metric=11076307, timeout=False), MetricValue(zkvm='sp1', program='npb-lu', metric=24273849, timeout=False), MetricValue(zkvm='sp1', program='npb-mg', metric=13024171, timeout=False), MetricValue(zkvm='sp1', program='npb-sp', metric=37552785, timeout=False), MetricValue(zkvm='sp1', program='spec-631', metric=33570223, timeout=False)]
