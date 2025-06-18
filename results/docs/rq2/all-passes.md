# All passes (individual genetic tuning)

## Best

### Top 1-length sequences

- Sequence: ['inline'], Count: 609
- Sequence: ['sroa'], Count: 496
- Sequence: ['jump-threading'], Count: 436
- Sequence: ['ipsccp'], Count: 285
- Sequence: ['newgvn'], Count: 280
- Sequence: ['gvn'], Count: 277
- Sequence: ['licm'], Count: 267
- Sequence: ['memcpyopt'], Count: 244
- Sequence: ['simplifycfg'], Count: 243
- Sequence: ['dse'], Count: 243
- Sequence: ['tailcallelim'], Count: 211
- Sequence: ['attributor'], Count: 201

### Top 2-length sequences

- Sequence: ['inline', 'jump-threading'], Count: 65
- Sequence: ['inline', 'sroa'], Count: 50
- Sequence: ['jump-threading', 'sroa'], Count: 45
- Sequence: ['inline', 'gvn'], Count: 45
- Sequence: ['inline', 'licm'], Count: 45
- Sequence: ['sroa', 'ipsccp'], Count: 40
- Sequence: ['gvn', 'jump-threading'], Count: 40
- Sequence: ['licm', 'newgvn'], Count: 37
- Sequence: ['memcpyopt', 'sroa'], Count: 35
- Sequence: ['inline', 'newgvn'], Count: 35
- Sequence: ['sroa', 'licm'], Count: 35
- Sequence: ['newgvn', 'simplifycfg'], Count: 34

### Top 3-length sequences

- Sequence: ['inline', 'gvn', 'jump-threading'], Count: 15
- Sequence: ['break-crit-edges', 'sink', 'sroa'], Count: 15
- Sequence: ['licm', 'newgvn', 'jump-threading'], Count: 12
- Sequence: ['memcpyopt', 'sroa', 'sink'], Count: 10
- Sequence: ['reassociate', 'inline', 'gvn'], Count: 10
- Sequence: ['ipsccp', 'irce', 'indvars'], Count: 10
- Sequence: ['gvn', 'jump-threading', 'tailcallelim'], Count: 10
- Sequence: ['newgvn', 'simplifycfg', 'sink'], Count: 10
- Sequence: ['gvn', 'inline', 'jump-threading'], Count: 10
- Sequence: ['inline', 'jump-threading', 'globalopt'], Count: 10
- Sequence: ['simplifycfg', 'argpromotion', 'always-inline'], Count: 10
- Sequence: ['inline', 'tailcallelim', 'sroa'], Count: 10

### Top 4-length sequences

- Sequence: ['inline', 'gvn', 'jump-threading', 'tailcallelim'], Count: 10
- Sequence: ['inline', 'licm', 'memcpyopt', 'sroa'], Count: 10
- Sequence: ['hotcoldsplit', 'inline', 'newgvn', 'correlated-propagation'], Count: 10
- Sequence: ['reg2mem', 'irce', 'inline', 'gvn'], Count: 9
- Sequence: ['break-crit-edges', 'licm', 'newgvn', 'reg2mem'], Count: 6
- Sequence: ['inline', 'synthetic-counts-propagation', 'instsimplify', 'sccp'], Count: 5
- Sequence: ['synthetic-counts-propagation', 'instsimplify', 'sccp', 'loop-rotate'], Count: 5
- Sequence: ['instsimplify', 'sccp', 'loop-rotate', 'simplifycfg'], Count: 5
- Sequence: ['sccp', 'loop-rotate', 'simplifycfg', 'jump-threading'], Count: 5
- Sequence: ['loop-rotate', 'simplifycfg', 'jump-threading', 'sroa'], Count: 5
- Sequence: ['simplifycfg', 'jump-threading', 'sroa', 'memcpyopt'], Count: 5
- Sequence: ['jump-threading', 'sroa', 'memcpyopt', 'speculative-execution'], Count: 5

## Worst

### Worst 1-length sequences

- Sequence: ['licm'], Count: 383
- Sequence: ['reg2mem'], Count: 247
- Sequence: ['loop-extract'], Count: 157
- Sequence: ['argpromotion'], Count: 120
- Sequence: ['loop-rotate'], Count: 115
- Sequence: ['mldst-motion'], Count: 107
- Sequence: ['bounds-checking'], Count: 102
- Sequence: ['irce'], Count: 101
- Sequence: ['sink'], Count: 97
- Sequence: ['reassociate'], Count: 97
- Sequence: ['loop-reduce'], Count: 97
- Sequence: ['inline'], Count: 95

### Worst 2-length sequences

- Sequence: ['reg2mem', 'licm'], Count: 30
- Sequence: ['inline', 'licm'], Count: 19
- Sequence: ['licm', 'reg2mem'], Count: 17
- Sequence: ['mergeicmps', 'licm'], Count: 9
- Sequence: ['licm', 'strip-dead-prototypes'], Count: 9
- Sequence: ['licm', 'add-discriminators'], Count: 9
- Sequence: ['break-crit-edges', 'licm'], Count: 9
- Sequence: ['licm', 'sink'], Count: 8
- Sequence: ['licm', 'separate-const-offset-from-gep'], Count: 8
- Sequence: ['consthoist', 'licm'], Count: 8
- Sequence: ['callsite-splitting', 'licm'], Count: 8
- Sequence: ['simplifycfg', 'reg2mem'], Count: 8

### Worst 3-length sequences

- Sequence: ['reg2mem', 'inline', 'licm'], Count: 6
- Sequence: ['reg2mem', 'licm', 'loop-unroll'], Count: 4
- Sequence: ['argpromotion', 'lcssa', 'licm'], Count: 4
- Sequence: ['instsimplify', 'reg2mem', 'licm'], Count: 4
- Sequence: ['inline', 'ipsccp', 'newgvn'], Count: 4
- Sequence: ['jump-threading', 'tailcallelim', 'loop-rotate'], Count: 3
- Sequence: ['loop-instsimplify', 'break-crit-edges', 'bounds-checking'], Count: 3
- Sequence: ['break-crit-edges', 'bounds-checking', 'gvn'], Count: 3
- Sequence: ['bounds-checking', 'gvn', 'break-crit-edges'], Count: 3
- Sequence: ['add-discriminators', 'loop-interchange', 'loop-predication'], Count: 3
- Sequence: ['gvn-sink', 'adce', 'sroa'], Count: 3
- Sequence: ['strip-gc-relocates', 'loop-idiom', 'function-attrs'], Count: 3

### Worst 4-length sequences

- Sequence: ['loop-instsimplify', 'break-crit-edges', 'bounds-checking', 'gvn'], Count: 3
- Sequence: ['break-crit-edges', 'bounds-checking', 'gvn', 'break-crit-edges'], Count: 3
- Sequence: ['strip-gc-relocates', 'loop-idiom', 'function-attrs', 'loop-deletion'], Count: 3
- Sequence: ['loop-idiom', 'function-attrs', 'loop-deletion', 'loop-sink'], Count: 3
- Sequence: ['bdce', 'loop-extract', 'bounds-checking', 'lower-switch'], Count: 3
- Sequence: ['loop-extract', 'bounds-checking', 'lower-switch', 'bdce'], Count: 3
- Sequence: ['loop-reduce', 'mergereturn', 'hotcoldsplit', 'globalsplit'], Count: 3
- Sequence: ['mergereturn', 'hotcoldsplit', 'globalsplit', 'separate-const-offset-from-gep'], Count: 3
- Sequence: ['hotcoldsplit', 'globalsplit', 'separate-const-offset-from-gep', 'loop-extract'], Count: 3
- Sequence: ['globalsplit', 'separate-const-offset-from-gep', 'loop-extract', 'dse'], Count: 3
- Sequence: ['separate-const-offset-from-gep', 'loop-extract', 'dse', 'loop-unroll-and-jam'], Count: 3
- Sequence: ['extract-blocks', 'bdce', 'constmerge', 'always-inline'], Count: 3
