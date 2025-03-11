import json
import logging
import os
from typing import Callable

from matplotlib import pyplot as plt
import numpy as np

from zkbench.config import get_measurements, get_programs, get_zkvms


BASELINE = 'baseline'


def get_title(base: str, info: list[str | None]):
    title = base
    if any(map(lambda x: x is not None, info)):
        title += "(" + ", ".join([x for x in info if x is not None]) + ")"
    return title


def read_data(dir: str, program: str, zkvm: str, profile: str, measurement: str):
    path = os.path.join(dir, f"{program}-{zkvm}-{measurement}/{zkvm}-{measurement}", profile, "new/estimates.json")
    return json.load(open(path, 'r'))

def get_mean_ms(dir: str, program: str, zkvm: str, profile: str, measurement: str):
    data = read_data(dir, program, zkvm, profile, measurement)
    return data['mean']['point_estimate'] / 1_000_000


def plot_sorted(values, labels, title, y_label):
    sorted_indices = np.argsort(values)[::-1]
    profiles_sorted = [labels[i] for i in sorted_indices]
    increase_values_sorted = [values[i] for i in sorted_indices]

    fig, ax = plt.subplots(figsize=(10, 6))

    x_pos = np.arange(len(profiles_sorted))

    ax.bar(x_pos, increase_values_sorted, width=0.4, color="gray")

    ax.set_xticks(x_pos)
    ax.set_xticklabels(profiles_sorted, rotation=45, ha="right")
    ax.set_ylabel(y_label)
    ax.set_title(title)
    ax.legend()

    ax.grid(axis="y", linestyle="--", alpha=0.7)

    plt.tight_layout

    plt.show()


def get_average_across(
    dir: str,
    zkvm: str | None,
    measurement: str | None,
    program: str | None,
    profile: list[str],
    fn: Callable[[str, str, str, str, str], float],
):
    res = []
    zkvms = get_zkvms() if zkvm is None else [zkvm]
    measurements = get_measurements() if measurement is None else [measurement]
    programs = get_programs() if program is None else [program]
    for profile in profile:
        relative_improvements = []
        for program in programs:
            for zkvm in zkvms:
                for measurement in measurements:
                    try:
                        relative_improvements.append(
                            fn(dir, program, zkvm, profile, measurement)
                        )
                    except FileNotFoundError:
                        logging.warning(
                            f"Data for {program}-{zkvm}-{measurement}-{profile} not found"
                        )
        res.append(np.average(relative_improvements))
    return res
