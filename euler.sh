#!/bin/bash

#SBATCH --gpus=1
#SBATCH --gres=gpumem:24gs

uv run --config-file /dev/null -m zkbench bench --zkvm risc0
