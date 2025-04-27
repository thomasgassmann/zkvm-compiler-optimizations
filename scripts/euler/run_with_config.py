#!/usr/bin/env python

import argparse
import os

parser = argparse.ArgumentParser(description="Run prove with predetermined config")
parser.add_argument("--zkvm", type=str, help="zkVM to use", required=True)
parser.add_argument("--program", type=str, help="program to run", required=True)
parser.add_argument("--timed", type=int, help="time", required=True)

args = parser.parse_args()

zkvm = args.zkvm
program = args.program
timed = args.timed
if not zkvm in ["sp1", "risc0"]:
    raise ValueError(f"Unsupported zkVM {zkvm}")
if not program:
    raise ValueError(f"Unsupported program {program}")
if not timed:
    raise ValueError(f"Unsupported time {timed}")

config = {
    "rsp": {
        "sp1": "GPUS=rtx_3090:1 GPUMEM=24g",
        "risc0": "GPUS=quadro_rtx_6000:1 GPUMEM=24g",
    },
    "default_sp1": "GPUS=rtx_3090:1 GPUMEM=24g",
    "default_risc0": "GPUS=quadro_rtx_6000:1 GPUMEM=24g",
}

gpu_config = config.get(f'default_{zkvm}') if program not in config else config[program][zkvm]
if gpu_config is None:
    raise ValueError(f"Unsupported program {program} for zkVM {zkvm}")

command = f"""
          TIMED={timed}
          OUT={f'{program}-{zkvm}'} 
          {gpu_config}
          ./scripts/euler/run.sh bench 
              --program {program} 
              --zkvm {zkvm} 
              --measurement prove
""".strip().replace(
    "\n", " "
)
print(command)
