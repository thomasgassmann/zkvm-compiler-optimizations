#!/bin/bash

usage() {
    echo 'Usage: ./build.sh <program> <zkvm> <rustc>'
    echo '<program> - Program name to execute (in programs/)'
    echo '<zkvm>    - sp1/risc0'
    echo '<rustc>   - Flags to rustc'
    echo Example: ./build.sh loop-sum risc0 "-C llvm-args=-unroll-threshold=0"
    exit
}

if [ "$#" -ne 3 ]
then
    usage
fi

program="$1"
if [ "$program" == "keccak256" ]; then
    program="$program-$2"
fi

cd "programs/$program"

if [ "$2" == "sp1" ]; then
    RUSTFLAGS="-C passes=lower-atomic -C link-arg=-Ttext=0x00200800 -C panic=abort $3" \
        RUSTUP_TOOLCHAIN=succinct \
        CARGO_BUILD_TARGET=riscv32im-succinct-zkvm-elf \
        cargo build --release --locked --features sp1
    echo "Built ./programs/$program/target/riscv32im-succinct-zkvm-elf/release/$program"
fi

if [ "$2" == "risc0" ]; then
    CC=gcc CC_riscv32im_risc0_zkvm_elf=~/.risc0/cpp/bin/riscv32-unknown-elf-gcc \
        RUSTFLAGS="-C passes=loweratomic -C link-arg=-Ttext=0x00200800 -C panic=abort $3" \
        RISC0_FEATURE_bigint2=1 \
        cargo +risc0 build --release --locked \
            --target riscv32im-risc0-zkvm-elf --manifest-path Cargo.toml --features risc0
    echo "Built ./programs/$program/target/riscv32im-risc0-zkvm-elf/release/$program"
fi
