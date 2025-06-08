# Polybench

Run is documented [here](./report/all-polybench-cycle-count/README.md).

## Improvement by program

### percentage

![polybench-improvement](./imgs/polybench-improvement.png)

### speedup

![polybench-speedup](./imgs/polybench-speedup.png)

## Duration by program

### prove

![prove-duration](./imgs/polybench-prove.png)

#### prove duration sp1

![prove-duration-sp1](./imgs/polybench-prove-sp1.png)

#### prove duration risc0

![prove-duration-risc0](./imgs/polybench-prove-risc0.png)

### exec

![exec-duration](./imgs/polybench-exec.png)

#### exec duration sp1

![exec-duration-sp1](./imgs/polybench-exec-sp1.png)

#### exec duration risc0

![exec-duration-risc0](./imgs/polybench-exec-risc0.png)

## Cycle counts

![cycle-counts](./imgs/polybench-cycle-count.png)

## Cause

Incorrect blackbox implementation in polybench-rs, see [comment in source](https://docs.rs/bencher/0.1.5/src/bencher/lib.rs.html#590-596).
