#!/bin/bash

RUSTFLAGS="-C target-cpu=native -C target-feature=-avx512f" cargo build --release -p runner -F profiling
