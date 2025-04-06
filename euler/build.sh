#!/bin/bash

CUDA_LIBRARY_PATH=/cluster/home/tgassmann/nvidia-toolkit RUST_BACKTRACE=1 RUSTFLAGS="-C target-cpu=native" cargo build -r -F cuda
