import logging

import numpy as np
from zkbench.config import get_measurements, get_zkvms_with_x86
from zkbench.plot.common import (
    get_average_improvement_over_baseline,
    get_title,
    plot_sorted,
)


def plot_improvement_for_single_program(
    dir: str,
    program: str,
    profiles: list[str],
    baseline_profile: str,
    speedup: bool,
    show_x86: bool = True,
):

    def f(dir, program, zkvm, measurement, profile):
        return get_average_improvement_over_baseline(
            dir,
            zkvm,
            program,
            profile,
            measurement,
            speedup=speedup,
            baseline=baseline_profile,
        )

    title = get_title(
        f"Average improvement for {program} ({', '.join(profiles)} compared to {baseline_profile})",
        [],
    )

    improvements = [[] for _ in profiles]
    labels = []
    for zkvm in get_zkvms_with_x86():
        if not show_x86 and zkvm == "x86":
            continue

        for measurement in get_measurements():
            if measurement == "prove" and zkvm == "x86":
                continue

            for i, profile in enumerate(profiles):
                try:
                    improvement = f(dir, program, zkvm, measurement, profile)
                    improvements[i].append(improvement)
                except FileNotFoundError:
                    continue
            else:
                labels.append(f"{zkvm} ({measurement})")

    for i, label in enumerate(labels):
        logging.info(f"Label: {label}")
        logging.info("Improvements: %s", np.array(improvements)[:, i])

    y_axis = "speedup" if speedup else "% faster"
    plot_sorted(improvements, labels, title, y_axis, profiles)
