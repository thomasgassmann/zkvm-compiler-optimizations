import logging
import numpy as np
import os
import json
import matplotlib.pyplot as plt

from zkbench.common import get_run_config
from zkbench.config import get_profiles_ids, get_programs, get_zkvms

PLOT_PROPERTY = "execution_duration"

def filename(program: str, zkvm: str, optimization: str) -> str:
    return f"results/{program}-{zkvm}-{optimization}.json"


def run(program: str, zkvm: str, file: str, profile: str):
    res = os.system(f"""
        cargo run --release -p runner -- --prover {zkvm} --program {program} --filename {file} --profile {profile}
    """.strip())
    if res != 0:
        raise ValueError(f"Error: Run failed with code {res}")


def run_with_plot(program: str | None, zkvm: str | None, profile: str | None):
    scores = dict()
    groups = list()
    programs, zkvms, profiles = get_run_config(program, zkvm, profile)
    for profile in profiles:
        scores[profile] = []
        for zkvm in zkvms:
            for program in programs:
                if program == 'zkvm-mnist':
                    continue
                fn = filename(program, zkvm, profile)
                if not os.path.isfile(fn):
                    logging.info(f"Running {zkvm}: {program} with ({profile})")
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
