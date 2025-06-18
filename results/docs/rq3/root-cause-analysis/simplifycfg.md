# simplifycfg

## Motivating example

```rust
#[inline(never)]
fn abs_i32_branchy(x: i32) -> i32 {
    if x < 0 {
        x.wrapping_neg()
    } else {
        x
    }
}
```

Compile with `-C no-prepopulate-passes -C passes=sroa,mem2reg,simplifycfg -C opt-level=3 -C llvm-args=-disable-cgp-branch-opts` (`simplifycfg-opt`):

```asm
srai a1, a0, 31
xor a0, a0, a1
sub a0, a0, a1
ret
```

Compile with `-C no-prepopulate-passes -C passes=sroa,mem2reg -C opt-level=3 -C llvm-args=-disable-cgp-branch-opts` (`simplifycfg-baseline`):

```asm
bltz a0, .LBB52_2
ret
.LBB52_2:
neg a0, a0
ret
```

`simplifycfg-opt` performs better on x86 (branch predictor mispredictions in the other case).

`simplifycfg-baseline` performs better on zkVMs (especially risc0) as it yields a lower cycle count (`xor` e.g. takes two cycles).
