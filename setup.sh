#!/bin/bash

export LD_LIBRARY_PATH=/home/thomas/git/thesis/rust-risc0/llvm-build/lib:$LD_LIBRARY_PATH
export ZK_CLANG_PATH="LD_LIBRARY_PATH='' /usr/lib/llvm19/bin/clang"
export ZK_OPT_PATH=/home/thomas/git/thesis/rust-risc0/llvm-build/bin/opt
export ZK_LLC_PATH=/home/thomas/git/thesis/rust-risc0/llvm-build/bin/llc
