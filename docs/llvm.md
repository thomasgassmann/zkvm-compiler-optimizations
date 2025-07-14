# Build LLVM risc0

The following is still in progress.

## in rust-risc0

```bash
mkdir llvm-build && cd llvm-build
cmake -G Ninja ../src/llvm-project/llvm \
  -DCMAKE_BUILD_TYPE=Release \
  -DLLVM_BUILD_LLVM_DYLIB=ON  \
  -DLLVM_LINK_LLVM_DYLIB=ON   \
  -DLLVM_ENABLE_ASSERTIONS=ON \
  -DLLVM_PARALLEL_LINK_JOBS=2
ninja
```

## in risc0/rzup or sp1/crates/cli

Use following toml:

```toml
[build]
target = ["riscv32im-risc0-zkvm-elf"]
extended = true
tools = ["cargo", "cargo-clippy", "clippy", "rustfmt"]
configure-args = []
cargo-native-static = true

[rust]
lld = true
llvm-tools = true
omit-git-hash = false

[llvm]
download-ci-llvm = false
link-shared = true

[target.riscv32im-risc0-zkvm-elf]
llvm-config = "/home/thomas/git/thesis/rust-risc0/llvm-build/bin/llvm-config"

[target.x86_64-unknown-linux-gnu]
llvm-config = "/home/thomas/git/thesis/rust-risc0/llvm-build/bin/llvm-config"
```

Build rust (only once):

```bash
export LD_LIBRARY_PATH=/home/thomas/git/thesis/rust-risc0/llvm-build/lib:$LD_LIBRARY_PATH
cargo run -- build --path ~/git/thesis/rust-risc0 rust
```

## Rebuild LLVM

```bash
ninja
```

## Build in this repo

```bash
export LD_LIBRARY_PATH=/home/thomas/git/thesis/rust-risc0/llvm-build/lib:$LD_LIBRARY_PATH
./zkbench.sh build ...
```
