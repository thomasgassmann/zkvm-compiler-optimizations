#!/bin/bash

module load stack/2024-06 openssl/3.1.3-zhfub4o cuda/12.1.1 gperftools/2.13
srun --mem-per-cpu=32GB --time=1-00:00:00 --pty bash -l
