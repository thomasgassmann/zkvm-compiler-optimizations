#!/usr/bin/env python

import argparse
import os

parser = argparse.ArgumentParser(description="Run prove with predetermined config")
parser.add_argument("zkvm", type=str, help="zkVM to use", required=True)
parser.add_argument("program", type=str, help="program to run", required=True)
parser.add_argument("timed", type=int, help="time", required=True)
parser.add_argument("out", type=str, help="out file", required=True)

args = parser.parse_args()

zkvm = args.zkvm
program = args.program
timed = args.timed
out = args.out

config = {
    'npb-ft': {
        'sp1': 'GPUS=rtx_3090:1',
        'risc0': 'GPUS=rtx_3090:1'
    },
    'default_sp1': 'GPUS=rtx_3090:1',
    'default_risc0': 'GPUS=rtx_3090:1',
}

gpu_config = config.get(f'default_{zkvm}') if program not in config else config[program][zkvm]
if gpu_config is None:
    raise ValueError(f"Unsupported program {program} for zkVM {zkvm}")

os.system(f"""
          TIMED={timed}
          OUT={out} 
          {gpu_config}
          ./scripts/euler/run.sh bench 
              --program {program} 
              --zkvm risc0 
              --measurement prove
""".strip().replace('\n', ' '))
