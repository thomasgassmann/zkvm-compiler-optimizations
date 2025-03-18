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
        title += " (" + ", ".join([x for x in info if x is not None]) + ")"
    return title


def read_data(dir: str, program: str, zkvm: str, profile: str, measurement: str):
    path = os.path.join(
        dir, f"{program}-{zkvm}/{zkvm}-{measurement}", profile, "new/estimates.json"
    )
    return json.load(open(path, 'r'))


def read_program_meta(dir: str, program: str, zkvm: str, profile: str):
    path = os.path.join(dir, f"{program}-{zkvm}/{profile}.json")
    return json.load(open(path, "r"))


def get_cycle_count(dir: str, program: str, zkvm: str, profile: str):
    return read_program_meta(dir, program, zkvm, profile)["cycle_count"]


def get_mean_ms(dir: str, program: str, zkvm: str, profile: str, measurement: str):
    data = read_data(dir, program, zkvm, profile, measurement)
    return data['mean']['point_estimate'] / 1_000_000


def plot_grouped_boxplot(values, labels, title, y_label, series_labels, bar_width=0.35):
    num_profiles = len(labels)
    num_series = len(values)

    sorted_indices = sorted(
        range(num_profiles),
        key=lambda i: np.median(values[0][i]) if values[0][i] else float("-inf"),
        reverse=True,
    )
    sorted_labels = [labels[i] for i in sorted_indices]
    sorted_values = [[series[i] for i in sorted_indices] for series in values]

    group_width = bar_width + 0.05
    offsets = (
        np.linspace(-group_width / 2, group_width / 2, num_series)
        if num_series > 1
        else [0]
    )

    fig, ax = plt.subplots(figsize=(10, 6))
    box_artists = []
    for series_idx in range(num_series):
        positions = np.arange(num_profiles) + offsets[series_idx]
        bp = ax.boxplot(
            sorted_values[series_idx],
            positions=positions,
            widths=bar_width,
            patch_artist=True,
            manage_ticks=False,
        )
        color = plt.cm.tab10(series_idx)
        for box in bp["boxes"]:
            box.set(facecolor=color)
        box_artists.append(bp["boxes"][0])

    ax.set_xticks(np.arange(num_profiles))
    ax.set_xticklabels(sorted_labels, rotation=45, ha="right")
    ax.set_title(title)
    ax.set_ylabel(y_label)
    ax.legend(box_artists, series_labels)
    ax.grid(axis="y", linestyle="--", alpha=0.7)
    ax.grid(axis="x", linestyle="--", alpha=0.5)
    plt.tight_layout()
    plt.show()


def get_values_by_profile(
    dir: str,
    zkvm: str | None,
    measurement: str | None,
    program: str | None,
    profiles: list[str],
    fn: Callable[[str, str, str, str, str], float],
):
    res = []
    zkvms = get_zkvms() if zkvm is None else [zkvm]
    measurements = get_measurements() if measurement is None else [measurement]
    programs = get_programs() if program is None else [program]
    for profile in profiles:
        values_list = []
        for prog in programs:
            for zk in zkvms:
                for meas in measurements:
                    try:
                        values_list.append(fn(dir, prog, zk, profile, meas))
                    except FileNotFoundError:
                        logging.warning(
                            f"Data for {prog}-{zk}-{meas}-{profile} not found"
                        )
        res.append(values_list)
    return res
