import subprocess
import numpy as np
import os
import json
import matplotlib.pyplot as plt

json.load(open("flags.json", "r"))
OPTIMIZATIONS = json.load(open("flags.json", "r"))

ZKVMS = ["sp1", "risc0"]

PROGRAMS = ["loop-sum", "factorial", "sha256", "rust-tests", "keccak256"]

PLOT_PROPERTY = "execution_duration"


def filename(program: str, zkvm: str, optimization: str) -> str:
    return f"results/{program}-{zkvm}-{optimization}.json"


def build(optimization: str, program: str, zkvm: str):
    subprocess.run(["./build.sh", program, zkvm, optimization, ""])

def run(program: str, zkvm: str, file: str):
    subprocess.run(["./run.sh", program, zkvm, file])

scores = dict()
groups = list()
for optimization in OPTIMIZATIONS.keys():
    scores[optimization] = []
    for zkvm in ZKVMS:
        for program in PROGRAMS:
            fn = filename(program, zkvm, optimization)
            if not os.path.isfile(fn):
                print(f"Running {zkvm}: {program} with ({optimization})")
                build(OPTIMIZATIONS[optimization], program, zkvm)
                run(program, zkvm, fn)

            with open(fn, "r") as f:
                d = json.load(f)

            n = f"{program} ({zkvm})"
            if n not in groups:
                groups.append(n)
            scores[optimization].append(d[PLOT_PROPERTY])

x = np.arange(len(groups))
width = 0.2

fig, ax = plt.subplots()
for i, (label, values) in enumerate(scores.items()):
    ax.bar(x + i * width, values, width, label=label)

ax.set_xlabel("program - zkvm")
ax.set_ylabel("Prove duration (s)")
ax.set_title("Prove duration by optimization level")
ax.set_xticks(x + width * 1.5)
ax.set_xticklabels(groups)
ax.legend()

plt.show()
