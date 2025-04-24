import json
import logging
import os
from typing import Callable
from scipy import stats

from matplotlib import pyplot as plt
import numpy as np

from zkbench.config import (
    get_measurements,
    get_program_by_name,
    get_programs,
    get_programs_by_group,
    get_zkvms,
)


BASELINE = "baseline"


def get_program_selection(
    program: list[str] | str | None, program_group: list[str] | str | None
) -> list[str]:
    if program is None and program_group is None or not program and not program_group:
        return get_programs()

    programs = []
    if program is not None:
        if isinstance(program, str):
            programs.append(program)
        else:
            programs.extend(program)

    if program_group is not None:
        if isinstance(program_group, str):
            program_groups = [program_group]
        else:
            program_groups = program_group
        for group in program_groups:
            programs.extend(get_programs_by_group(group))

    return programs


def get_title(base: str, info: list[str | None]):
    title = base
    if any(map(lambda x: x is not None, info)):
        title += " (" + ", ".join([x for x in info if x is not None]) + ")"
    return title


def has_data_on(dir: str, program: str, zkvm: str, measurement: str):
    path = os.path.join(dir, f"{program}-{zkvm}-{measurement}")
    return os.path.exists(path)


def read_estimates_data(
    dir: str, program: str, zkvm: str, profile: str, measurement: str
):
    opt_path = os.path.join(dir, f"{program}-{zkvm}-{measurement}", profile)
    if not os.path.exists(opt_path):
        baseline_meta = read_program_meta(dir, program, zkvm, BASELINE)

        program_config = get_program_by_name(program)
        if profile in program_config.skip:
            logging.warning(
                f"{profile} is skipped for {program}, but using {BASELINE} data!"
            )
        else:
            meta = read_program_meta(dir, program, zkvm, profile)
            # in case this directory does not exist, the optimization was not applied
            # hence we did not run it and the two binaries must be the same
            if meta["hash"] != baseline_meta["hash"]:
                raise FileNotFoundError(
                    f"Expected {profile} for {program}-{zkvm}-{measurement} to be the same as {BASELINE}, but is not"
                )

        # as the binaries are the same, we use the baseline estimates
        opt_path = os.path.join(dir, f"{program}-{zkvm}-{measurement}", BASELINE)

    json_file = os.path.join(opt_path, "new/estimates.json")
    return json.load(open(json_file, "r"))


def read_program_meta(dir: str, program: str, zkvm: str, profile: str):
    program_config = get_program_by_name(program)
    if profile in program_config.skip:
        logging.warning(f"{profile} is skipped for {program}, returning None")
        return None

    path = os.path.join(dir, f"meta/{program}/{zkvm}/{profile}.json")
    return json.load(open(path, "r"))


def get_cycle_count(dir: str, program: str, zkvm: str, profile: str):
    program_config = get_program_by_name(program)
    if profile in program_config.skip:
        logging.warning(f"{profile} is skipped for {program}, returning None")
        return None

    return read_program_meta(dir, program, zkvm, profile)["cycle_count"]


def get_point_estimate_mean_ms(
    dir: str, program: str, zkvm: str, profile: str, measurement: str
):
    data = read_estimates_data(dir, program, zkvm, profile, measurement)
    return data["mean"]["point_estimate"] / 1_000_000


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


def get_spearman(x, y):
    return stats.spearmanr(x, y).statistic


def get_pearson(x, y):
    return np.corrcoef(x, y)[0, 1]


def plot_scatter_by_zkvm(
    title: str,
    get_by_zkvm: Callable[[str], tuple[np.ndarray, np.ndarray]],
    x_label: str,
    y_label: str,
):
    for zkvm in get_zkvms():
        x, y = get_by_zkvm(zkvm)
        pearson = get_pearson(x, y)
        spearman = get_spearman(x, y)
        plt.scatter(
            x, y, label=f"{zkvm}, Pearson={pearson:.3f}, Spearman={spearman:.3f}"
        )
        plt.plot(
            np.unique(x),
            np.poly1d(np.polyfit(x, y, 1))(np.unique(x)),
        )

    plt.title(title)
    plt.xlabel(x_label)
    plt.ylabel(y_label)
    plt.grid(linestyle="--", alpha=0.7)
    plt.legend()
    plt.show()


def plot_sorted(values, labels, title, y_label, series_labels):
    sorted_indices = np.argsort(values[0])[::-1]
    profiles_sorted = [labels[i] for i in sorted_indices]
    increase_values_sorted = [
        [values[j][i] for i in sorted_indices] for j in range(len(values))
    ]

    fig, ax = plt.subplots(figsize=(10, 6))
    x_pos = np.arange(len(profiles_sorted))

    bar_width = 0.8 / len(values)

    for i in range(len(values)):
        ax.bar(
            x_pos + i * bar_width - (0.8 - bar_width) / 2,
            increase_values_sorted[i],
            width=bar_width,
            label=series_labels[i],
        )

    for x in x_pos:
        ax.axvline(
            x + bar_width / 2 - (0.8 - bar_width) / 2,
            color="gray",
            linestyle="--",
            alpha=0.2,
        )

    ax.set_xticks(x_pos)
    ax.set_xticklabels(profiles_sorted, rotation=45, ha="right")
    ax.set_ylabel(y_label)
    ax.set_title(title)
    if any(map(lambda x: x is not None, series_labels)):
        ax.legend()

    ax.grid(axis="y", linestyle="--", alpha=0.7)

    plt.tight_layout()
    plt.show()


def get_values_by_profile(
    dir: str,
    zkvm: str | None,
    measurement: str | None,
    program: str | None,
    program_group: str | None,
    profiles: list[str],
    fn: Callable[[str, str, str, str, str], float],
):
    res = []
    zkvms = get_zkvms() if zkvm is None else [zkvm]
    measurements = get_measurements() if measurement is None else [measurement]
    programs = get_program_selection(program, program_group)
    for profile in profiles:
        values_list = []
        for prog in programs:
            for zk in zkvms:
                for meas in measurements:
                    try:
                        r = fn(dir, prog, zk, profile, meas)
                        if r is not None:
                            values_list.append(r)
                    except FileNotFoundError:
                        logging.warning(
                            f"Data for {prog}-{zk}-{meas}-{profile} not found"
                        )
        res.append(values_list)
    return res
