# Plots

- [RQ1](./rq1/report/README.md)
- [RQ2](./rq2/README.md)

## Main takeaways

- [inlining almost always beneficial](./rq1/profiles/inline.md)
  - idea: genetic run over inline-threshold
  - does it keep being beneficial as we increase inlining threshold?
  - [partial-inliner](./rq1/profiles/partial-inliner.md), [partially-inline-libcalls](./rq1/profiles/partially-inline-libcalls.md) only on risc0, not for sp1
- [sroa](./rq1/profiles/sroa.md) and [gvn](./rq1/profiles/gvn.md) (as well as [newgvn](./rq1/profiles/newgvn.md)) also almost always beneficial
- [licm](./rq1/profiles/licm.md) seems to have a negative effect for most programs, in some cases substantially
  - holds for all programs in the group [loop-intensive](./rq1/program-groups/loop-intensive.md)
- [cycle count and execution/proving duration are correlated](./rq1/programs.md)
- [we can optimize a given program to minimize cycle count](./rq2/genetic-all-depth20-rsp-sp1/README.md)
  - NOTE: we also tune the compilers opt-level, lto and codegen units!
  - about 5-6% percentage improvement in cycle count for rsp on sp1 in o3 (lto off)
- [only two optimizations already result in almost 30% cycle count reduction](./rq2/top15-abs-cycle-count/README.md)
- [for c benchmarks mem2reg and sroa seem to be more beneficial](./rq2/top15-abs-cycle-count/README.md#group-c)
  - no such relationship for rust programs
  - NOTE: sample size small
