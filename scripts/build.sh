#!/bin/bash

RUSTFLAGS="-C target-cpu=native" cargo build --release -p runner
