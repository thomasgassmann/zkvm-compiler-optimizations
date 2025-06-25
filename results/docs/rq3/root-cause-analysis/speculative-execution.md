# speculative-execution

Speculative execution attempts to hoist side-effect–free instructions out of conditional blocks so that they execute unconditionally, eliminating the branch-dependence and potentially shortening critical paths on out-of-order, superscalar hardware. Because zkVMs execute RISCV32 code strictly in-order inside an arithmetic circuit, they do not benefit from reduced branch mis-prediction latency.

## Conclusions

- Isolated speculative execution can hurt zkVMs. As there is no out-of-order execution the pass only creates extra work. The prover must evaluate and the verifier must verify every speculated instruction.
- Speculative execution for zkVMs back-ends should be disabled unless cost-modeling can prove that the number of executed basic-blocks falls.
