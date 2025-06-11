import logging

import numpy as np
from zkbench.config import get_programs, get_zkvms
from zkbench.plot.common import (
    get_cycle_count,
    get_title,
    plot_grouped_boxplot,
    plot_sorted,
)


def plot_cycle_count_by_program(
    dir: str,
    profile: str,
    baseline_profile: str,
    relative: bool = False,
    plotted_zkvm: str | None = None,
):
    title = get_title(
        f"Cycle Count for {profile} compared to {baseline_profile}",
        [plotted_zkvm],
    )

    cycle_counts_profile = []
    cycle_counts_baseline = []
    programs = []
    for program in get_programs():
        err = False
        current_profile = []
        current_baseline = []
        for zkvm in get_zkvms() if plotted_zkvm is None else [plotted_zkvm]:
            try:
                cycle_count = get_cycle_count(dir, program, zkvm, profile)
                cycle_count_baseline = get_cycle_count(
                    dir, program, zkvm, baseline_profile
                )
                if relative:
                    current_profile.append(cycle_count / cycle_count_baseline)
                else:
                    current_profile.append(cycle_count)
                    current_baseline.append(cycle_count_baseline)
            except FileNotFoundError:
                logging.warning(
                    f"File not found for {program} {zkvm} {profile} {baseline_profile}. Skipping."
                )
                err = True
                break
        if err:
            continue
        programs.append(program)
        cycle_counts_profile.append(current_profile)
        cycle_counts_baseline.append(current_baseline)

    series_labels = (
        [
            profile,
            baseline_profile,
        ]
        if not relative
        else [profile]
    )
    if plotted_zkvm is not None:
        plot_sorted(
            (
                [
                    np.squeeze(cycle_counts_profile, axis=1),
                    np.squeeze(cycle_counts_baseline, axis=1),
                ]
                if not relative
                else [np.squeeze(cycle_counts_profile, axis=1)]
            ),
            programs,
            title,
            "Cycle Count",
            series_labels,
        )
    else:
        plot_grouped_boxplot(
            (
                [
                    cycle_counts_profile,
                    cycle_counts_baseline,
                ]
                if not relative
                else [cycle_counts_profile]
            ),
            programs,
            title,
            "Cycle Count",
            series_labels,
        )
