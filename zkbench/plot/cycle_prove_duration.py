import logging
from matplotlib import pyplot as plt
import numpy as np
from zkbench.config import get_profiles_ids, get_programs, get_zkvms
from zkbench.plot.common import BASELINE, get_cycle_count, get_point_estimate_ms, get_title


def plot_cycle_count_duration(dir: str, zk: str | None, measurement: str, p: str | None):
    zkvms = get_zkvms() if zk is None else [zk]
    programs = get_programs() if p is None else [p]
    profiles = get_profiles_ids()
    profiles.remove(BASELINE)
    for zkvm in zkvms:
        x, y = [], []
        for program in programs:
            try:
                baseline_cycle_count = get_cycle_count(dir, program, zkvm, BASELINE)
                baseline_duration = get_point_estimate_ms(dir, program, zkvm, BASELINE, measurement)
                for profile in profiles:
                    cycle_count = get_cycle_count(dir, program, zkvm, profile)
                    duration = get_point_estimate_ms(dir, program, zkvm, profile, measurement)
                    x.append((cycle_count - baseline_cycle_count) / baseline_cycle_count)
                    y.append((duration - baseline_duration) / baseline_duration)
            except FileNotFoundError:
                logging.warning(
                    f"Data for {program}-{zkvm}-{measurement} not found"
                )
        correlation = np.corrcoef(x, y)[0, 1]
        plt.scatter(x, y, label=zkvm)
        plt.plot(np.unique(x), np.poly1d(np.polyfit(x, y, 1))(np.unique(x)), label=f"{zkvm}, r={correlation:.3f}")


    plt.title(get_title("Relative cycle count vs duration compared to baseline", [measurement, p, zk]))
    plt.xlabel("Relative cycle count compared to baseline")
    plt.ylabel("Relative duration compared to baseline")
    plt.grid(linestyle="--", alpha=0.7)
    plt.legend()
    plt.show()
