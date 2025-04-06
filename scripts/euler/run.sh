#!/bin/bash

sbatch --output=out.txt --mem-per-cpu=32GB --time=5-00:00:00 --gpus=1 --gres=gpumem:24g --wrap="./zkbench.sh $@"
