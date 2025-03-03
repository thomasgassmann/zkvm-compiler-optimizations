import subprocess
import numpy as np
import os
import json
import matplotlib.pyplot as plt

json.load(open("flags.json", "r"))
OPTIMIZATIONS = json.load(open("flags.json", "r"))

ZKVMS = ["sp1", "risc0"]

PROGRAMS = ["loop-sum", "factorial", "sha256", "rust-tests", "keccak256"]
ZKVM_SPECIFIC = ["keccak256"]
FORCE_REBUILD = False

PLOT_PROPERTY = "execution_duration"


def filename(program: str, zkvm: str, optimization: str) -> str:
    return f"results/{program}-{zkvm}-{optimization}.json"


def elf_path(program: str, zkvm: str, profile: str | None) -> str:
    if program in ZKVM_SPECIFIC:
        program = f"{program}-{zkvm}"

    if zkvm == "sp1":
        res = (
            f"./programs/{program}/target/riscv32im-succinct-zkvm-elf/release/{program}"
        )
        if profile:
            res += f"-{profile}"
        return res
    elif zkvm == "risc0":
        res = f"./programs/{program}/target/riscv32im-risc0-zkvm-elf/release/{program}"
        if profile:
            res += f"-{profile}"
        return res
    else:
        raise ValueError(f"Unknown zkvm: {zkvm}")


def build(optimization: str, program: str, zkvm: str, profile: str):
    subprocess.run(["./build.sh", program, zkvm, optimization, profile])


def run(program: str, zkvm: str, file: str, profile: str):
    subprocess.run(["./run.sh", program, zkvm, file, profile])


scores = dict()
groups = list()
for zkvm in ZKVMS:
    for program in PROGRAMS:
        for profile in OPTIMIZATIONS.keys():
            existing = elf_path(program, zkvm, profile)
            if not os.path.isfile(existing) or FORCE_REBUILD:
                build(OPTIMIZATIONS[profile], program, zkvm, profile)

for profile in OPTIMIZATIONS.keys():
    scores[profile] = []
    for zkvm in ZKVMS:
        for program in PROGRAMS:
            fn = filename(program, zkvm, profile)
            if not os.path.isfile(fn):
                print(f"Running {zkvm}: {program} with ({profile})")
                run(program, zkvm, fn, profile)

            with open(fn, "r") as f:
                d = json.load(f)

            n = f"{program} ({zkvm})"
            if n not in groups:
                groups.append(n)
            scores[profile].append(d[PLOT_PROPERTY])

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
