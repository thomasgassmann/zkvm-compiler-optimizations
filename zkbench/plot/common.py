import json
import logging
import os
from typing import Callable
import click
from scipy import stats

from matplotlib import pyplot as plt
import numpy as np
from contextlib import contextmanager
import seaborn as sns
import pandas as pd

from zkbench.config import (
    get_measurements,
    get_program_by_name,
    get_programs,
    get_programs_by_group,
    get_zkvms,
)


BASELINE = "baseline"


class SaveContext:
    path = None


@contextmanager
def save_path(val):
    old_value = SaveContext.path
    SaveContext.path = val
    try:
        yield
    finally:
        SaveContext.path = old_value


def get_program_selection(
    program: list[str] | str | None,
    program_group: list[str] | str | None,
    ignore: list[str] | None = None,
) -> list[str]:
    if program is None and program_group is None or not program and not program_group:
        return list([p for p in get_programs() if p not in (ignore or [])])

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

    if ignore is not None:
        programs = [p for p in programs if p not in ignore]

    return programs


def get_title(base: str, info: list[str | None]):
    title = base
    if any(map(lambda x: x is not None, info)):
        title += " (" + ", ".join([x for x in info if x is not None]) + ")"
    return title


def has_data_on(dir: str, program: str, zkvm: str, measurement: str):
    path = os.path.join(dir, f"{program}-{zkvm}-{measurement}")
    return os.path.exists(path)


def read_data_file(
    dir: str, program: str, zkvm: str, profile: str, measurement: str, name: str
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
    elif profile != BASELINE:
        meta = read_program_meta(dir, program, zkvm, profile)
        try:
            baseline_meta = read_program_meta(dir, program, zkvm, BASELINE)
            assert (
                meta["hash"] != baseline_meta["hash"]
            ), f"{program}-{zkvm}-{profile} should not have been benchmarked"
        except FileNotFoundError:
            logging.warning(f"No baseline data for {program}-{zkvm}-{measurement}")

    json_file = os.path.join(opt_path, f"new/{name}.json")
    return json.load(open(json_file, "r"))


def read_estimates_data(
    dir: str, program: str, zkvm: str, profile: str, measurement: str
):
    return read_data_file(dir, program, zkvm, profile, measurement, "estimates")


def read_sample_data(dir: str, program: str, zkvm: str, profile: str, measurement: str):
    return read_data_file(dir, program, zkvm, profile, measurement, "sample")


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


def get_point_estimate_median_ms(
    dir: str, program: str, zkvm: str, profile: str, measurement: str
):
    data = read_estimates_data(dir, program, zkvm, profile, measurement)
    return data["median"]["point_estimate"] / 1_000_000


def get_sample_times_ms(
    dir: str, program: str, zkvm: str, profile: str, measurement: str
):
    data = read_sample_data(dir, program, zkvm, profile, measurement)
    return [item / 1_000_000 for item in data["times"]]


def get_average_improvement_over_baseline(
    dir: str,
    zkvm: str,
    program: str,
    profile: str,
    measurement: str,
    speedup: bool = False,
    baseline: str = BASELINE,
):
    baseline = get_point_estimate_median_ms(dir, program, zkvm, baseline, measurement)
    compared = get_point_estimate_median_ms(dir, program, zkvm, profile, measurement)
    if speedup:
        return baseline / compared
    return (-(compared - baseline) / baseline) * 100


def _get_config():
    parent_context = click.get_current_context().parent.params
    if "violin" in parent_context:
        violin = parent_context["violin"]
    else:
        violin = False
    if "vertical" in parent_context:
        vertical = parent_context["vertical"]
    else:
        vertical = False
    return vertical, violin


def plot_grouped_boxplot(
    values, labels, title, y_label, series_labels, bar_width=0.35, log_scale=False, show_fliers=False, drop_below=None
):
    vertical, violin = _get_config()
    num_profiles = len(labels)
    num_series = len(values)

    sorted_indices = sorted(
        range(num_profiles),
        key=lambda i: np.median(values[0][i]) if any(values[0][i]) else 0,
        reverse=True,
    )
    if drop_below is not None:
        sorted_indices = [
            i for i in sorted_indices
            if any((np.abs(values[series_idx][i]) >= drop_below).any() for series_idx in range(num_series))
        ]
        num_profiles = len(sorted_indices)

    sorted_labels = [labels[i] for i in sorted_indices]
    sorted_values = [[series[i] for i in sorted_indices] for series in values]

    if vertical:
        _, ax = plt.subplots(figsize=(6, 10))
    else:
        _, ax = plt.subplots(figsize=(10, 6))
    
    if violin:
        data_list = []
        for series_idx in range(num_series):
            for profile_idx, profile_values in enumerate(sorted_values[series_idx]):
                if profile_values is not None and len(profile_values) > 0:
                    for value in profile_values:
                        data_list.append({
                            'profile': sorted_labels[profile_idx],
                            'series': series_labels[series_idx],
                            'value': value
                        })
        
        df = pd.DataFrame(data_list)
        if not df.empty:
            if vertical:
                sns.violinplot(data=df, y='profile', x='value', hue='series', ax=ax, orient='h')
                ax.set_yticklabels(ax.get_yticklabels(), rotation=0)
            else:
                sns.violinplot(data=df, x='profile', y='value', hue='series', ax=ax)
                ax.set_xticklabels(ax.get_xticklabels(), rotation=45, ha="right")
    else:
        box_artists = []
        if num_series > 1:
            offsets = np.linspace(
                -((num_series - 1) * bar_width) / 2,
                ((num_series - 1) * bar_width) / 2,
                num_series,
            )
        else:
            offsets = [0]
        for series_idx in range(num_series):
            positions = np.arange(num_profiles) + offsets[series_idx]
            bp = ax.boxplot(
                sorted_values[series_idx],
                positions=positions,
                widths=bar_width,
                patch_artist=True,
                manage_ticks=False,
                vert=not vertical,
                showfliers=show_fliers,
            )
            color = plt.cm.tab10(series_idx)
            for box in bp["boxes"]:
                box.set(facecolor=color)
            box_artists.append(bp["boxes"][0])

            for i, arr in enumerate(sorted_values[series_idx]):
                if arr is None or len(arr) == 0:
                    continue
                # if the data array has exactly one element, the box collapses to a line
                # add a small scatter marker to make it more visible
                if len(set(arr)) == 1:
                    pos = positions[i]
                    val = arr[0]
                    if vertical:
                        x, y = val, pos
                    else:
                        x, y = pos, val
                    ax.scatter(
                        [x],
                        [y],
                        color=color,
                        marker="o",
                        s=50,  # size of the dot
                        zorder=3,
                    )

        if vertical:
            ax.set_yticks(np.arange(num_profiles))
            ax.set_yticklabels(sorted_labels)
        else:
            ax.set_xticks(np.arange(num_profiles))
            ax.set_xticklabels(sorted_labels, rotation=45, ha="right")
        ax.legend(box_artists, series_labels)

    ax.set_title(title)
    if vertical:
        ax.set_xlabel(y_label)
        if log_scale:
            ax.set_xscale("log")
        ax.grid(axis="x", linestyle="--", alpha=0.7)
        ax.grid(axis="y", linestyle="--", alpha=0.5)
    else:
        ax.set_ylabel(y_label)
        if log_scale:
            ax.set_yscale("log")
        ax.grid(axis="y", linestyle="--", alpha=0.7)
        ax.grid(axis="x", linestyle="--", alpha=0.5)
    plt.tight_layout()
    show_or_save_plot()


def get_spearman(x, y):
    return stats.spearmanr(x, y).statistic


def get_pearson(x, y):
    return np.corrcoef(x, y)[0, 1]


def show_or_save_plot():
    if SaveContext.path is not None:
        plt.gcf().set_size_inches(18, 10)
        plt.savefig(SaveContext.path, dpi=200)
        plt.close()
    else:
        plt.show()


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
    show_or_save_plot()


def plot_sorted(values, labels, title, y_label, series_labels, log_scale=False, drop_below=None, num_series_labels=None):
    if drop_below is not None:
        # if the value is below the threshold in all series, drop this value
        new_labels = []
        new_values = []
        for i, label in enumerate(labels):
            if np.any([np.abs(values[series_idx][i]) >= drop_below for series_idx in range(len(values))]):
                new_labels.append(label)
                for j in range(len(values)):
                    if len(new_values) <= j:
                        new_values.append([])

                    new_values[j].append(values[j][i])
        labels = new_labels
        values = new_values


    vertical, _ = _get_config()
    sorted_indices = np.argsort(values[0])[::-1]
    profiles_sorted = [labels[i] for i in sorted_indices]
    increase_values_sorted = [
        [values[j][i] for i in sorted_indices] for j in range(len(values))
    ]

    if vertical:
        fig, ax = plt.subplots(figsize=(6, 10))
    else:
        fig, ax = plt.subplots(figsize=(10, 6))
    
    pos = np.arange(len(profiles_sorted))
    num_series_labels = len(series_labels) if num_series_labels is None else num_series_labels
    bar_width = 0.8 / num_series_labels

    for i in range(len(values)):
        series_index = i % num_series_labels
        if vertical:
            ax.barh(
                pos + series_index * bar_width - (0.8 - bar_width) / 2,
                increase_values_sorted[i],
                height=bar_width,
                label=series_labels[i],
            )
        else:
            ax.bar(
                pos + series_index * bar_width - (0.8 - bar_width) / 2,
                increase_values_sorted[i],
                width=bar_width,
                label=series_labels[i],
            )

    for x in pos:
        if vertical:
            ax.axhline(
                x + bar_width / 2 - (0.8 - bar_width) / 2,
                color="gray",
                linestyle="--",
                alpha=0.2,
            )
        else:
            ax.axvline(
                x,
                color="gray",
                linestyle="--",
                alpha=0.2,
            )

    if vertical:
        ax.set_yticks(pos)
        ax.set_yticklabels(profiles_sorted)
        ax.set_xlabel(y_label)
        if log_scale:
            ax.set_xscale("log")
        ax.grid(axis="x", linestyle="--", alpha=0.7)
    else:
        ax.set_xticks(pos)
        ax.set_xticklabels(profiles_sorted, rotation=45, ha="right")
        ax.set_ylabel(y_label)
        if log_scale:
            ax.set_yscale("log")
        ax.grid(axis="y", linestyle="--", alpha=0.7)
    
    ax.set_title(title)

    if any(map(lambda x: x is not None, series_labels)):
        ax.legend()

    plt.tight_layout()
    show_or_save_plot()


def get_values_by_profile(
    dir: str,
    zkvm: str | None,
    measurement: str | None,
    program: str | list[str] | None,
    program_group: str | None,
    profiles: list[str],
    fn: Callable[[str, str, str, str, str], float],
    skipped_value=None,
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
                            if isinstance(r, list):
                                values_list.extend(r)
                            else:
                                values_list.append(r)
                        elif skipped_value is not None:
                            values_list.append(skipped_value)
                    except FileNotFoundError:
                        logging.warning(
                            f"Data for {prog}-{zk}-{meas}-{profile} not found"
                        )
                    except Exception as e:
                        logging.error(f"Error for {prog}-{zk}-{meas}-{profile}: {e}")
                        raise e
        res.append(values_list)
    return res
