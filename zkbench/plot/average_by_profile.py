import logging
from matplotlib import pyplot as plt
import numpy as np
from zkbench.config import get_measurements, get_profiles_ids, get_programs, get_zkvms
from zkbench.plot.common import BASELINE, get_mean_ms, get_title


def get_average_improvement(dir: str, zkvm: str | None, measurement: str | None, program: str | None, profile: list[str]):
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
                        baseline = get_mean_ms(dir, program, zkvm, BASELINE, measurement)
                        compared = get_mean_ms(dir, program, zkvm, profile, measurement)
                        relative_improvements.append(-(compared - baseline) / baseline)
                    except FileNotFoundError:
                        logging.warning(f"Data for {program}-{zkvm}-{measurement}-{profile} not found")
        res.append(np.average(relative_improvements))
    return res


def plot_average_improvement(dir: str, zkvm: str | None, measurement: str | None, program: str | None):
    title = get_title("Average improvement by profile", [zkvm, measurement, program])

    profiles = get_profiles_ids()
    profiles.remove(BASELINE)
    relative_improvements = get_average_improvement(
        dir, zkvm, measurement, program, profiles
    )

    sorted_indices = np.argsort(relative_improvements)[::-1]
    profiles_sorted = [profiles[i] for i in sorted_indices]
    increase_values_sorted = [relative_improvements[i] for i in sorted_indices]

    fig, ax = plt.subplots(figsize=(10, 6))

    x_pos = np.arange(len(profiles_sorted))

    ax.bar(
        x_pos, increase_values_sorted, width=0.4, color="gray"
    )

    ax.set_xticks(x_pos)
    ax.set_xticklabels(profiles_sorted, rotation=45, ha="right")
    ax.set_ylabel('improvement/degradation compared to baseline')
    ax.set_title(title)
    ax.legend()

    ax.grid(axis='y', linestyle='--', alpha=0.7)

    plt.tight_layout

    plt.show()
