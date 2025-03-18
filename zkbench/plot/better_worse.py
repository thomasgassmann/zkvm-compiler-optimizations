import logging
from matplotlib import pyplot as plt
import numpy as np
from zkbench.config import get_measurements, get_profiles_ids, get_programs, get_zkvms
from zkbench.plot.common import BASELINE, get_point_estimate_ms, get_title


def get_number_of_programs_improving_degrading(dir: str, zkvm: str, profile: str, measurement: str):
    better = 0
    worse = 0
    for program in get_programs():
        try:
            baseline_mean = get_point_estimate_ms(
                dir, program, zkvm, BASELINE, measurement
            )
            current_mean = get_point_estimate_ms(
                dir, program, zkvm, profile, measurement
            )
            if current_mean > baseline_mean:
                better += 1
            elif current_mean < baseline_mean:
                worse += 1
        except FileNotFoundError:
            logging.warning(f"Data for {program}-{zkvm}-{measurement}/{profile} not found")
    return (worse, better)


def get_increase_decrease_values(
    dir: str, zkvm: str | None, measurement: str | None, profiles: list[str]
):
    increase_values, decrease_values = [], []
    zkvms = get_zkvms() if zkvm is None else [zkvm]
    measurements = get_measurements() if measurement is None else [measurement]
    for profile in profiles:
        total_worse = 0
        total_better = 0
        for zkvm in zkvms:
            for measurement in measurements:
                w, b = get_number_of_programs_improving_degrading(
                    dir, zkvm, profile, measurement
                )
                total_worse += w
                total_better += b
        increase_values.append(total_worse)
        decrease_values.append(total_better)
    return increase_values, decrease_values


def plot_better_worse(dir: str, zkvm: str | None, measurement: str | None):
    title = get_title(
        "Improvement/degradation compared to baseline", [zkvm, measurement]
    )

    profiles = get_profiles_ids()
    profiles.remove(BASELINE)
    increase_values, decrease_values = get_increase_decrease_values(
        dir, zkvm, measurement, profiles
    )

    sorted_indices = np.argsort(increase_values)[::-1]
    profiles_sorted = [profiles[i] for i in sorted_indices]
    increase_values_sorted = [increase_values[i] for i in sorted_indices]
    decrease_values_sorted = [-decrease_values[i] for i in sorted_indices]

    fig, ax = plt.subplots(figsize=(10, 6))

    x_pos = np.arange(len(profiles_sorted))

    ax.bar(
        x_pos - 0.2, increase_values_sorted, width=0.4, color="gray", label="Increase"
    )
    ax.bar(
        x_pos + 0.2, decrease_values_sorted, width=0.4, color="red", label="Decrease"
    )

    ax.set_xticks(x_pos)
    ax.set_xticklabels(profiles_sorted, rotation=45, ha="right")
    ax.set_ylabel('improvement/degradation compared to baseline')
    ax.set_title(title)
    ax.legend()

    ax.grid(axis='y', linestyle='--', alpha=0.7)

    plt.tight_layout

    plt.show()
