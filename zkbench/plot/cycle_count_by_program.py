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
    profile: list[str],
    baseline_profile: str,
    relative: bool = False,
    plotted_zkvm: str | None = None,
):
    title = get_title(
        f"Cycle count compared to {baseline_profile}",
        [plotted_zkvm],
    )

    cycle_counts_profiles = {p: [] for p in profile}
    cycle_counts_baseline = []
    programs = []
    for program in get_programs():
        err = False
        current_profiles = {p: [] for p in profile}
        current_baseline = []
        for zkvm in get_zkvms() if plotted_zkvm is None else [plotted_zkvm]:
            try:
                cycle_count_baseline = get_cycle_count(
                    dir, program, zkvm, baseline_profile
                )
                current_baseline.append(cycle_count_baseline)
                
                for prof in profile:
                    cycle_count = get_cycle_count(dir, program, zkvm, prof)
                    if relative:
                        change = (cycle_count - cycle_count_baseline) / cycle_count_baseline * 100
                        current_profiles[prof].append(change)
                    else:
                        current_profiles[prof].append(cycle_count)
            except FileNotFoundError:
                logging.warning(
                    f"File not found for {program} {zkvm} {', '.join(profile)} {baseline_profile}. Skipping."
                )
                err = True
                break
        if err:
            continue
        programs.append(program)
        for prof in profile:
            cycle_counts_profiles[prof].append(current_profiles[prof])
        cycle_counts_baseline.append(current_baseline)

    series_labels = (
        profile + [baseline_profile]
        if not relative
        else profile
    )
    y_axis = "Change in Cycle Count (relative to baseline)" if relative else "Cycle Count"
    
    if relative:
        for prof in profile:
            avg = np.mean(cycle_counts_profiles[prof], axis=0)
            logging.info(
                f"Average cycle count change for {prof} across zkVMs: {avg}"
            )

    plot_data = (
        list(cycle_counts_profiles.values()) + [cycle_counts_baseline]
        if not relative
        else list(cycle_counts_profiles.values())
    )
    
    if plotted_zkvm is not None:
        plot_sorted(
            [np.squeeze(data, axis=1) for data in plot_data],
            programs,
            title,
            y_axis,
            series_labels,
        )
    else:
        plot_grouped_boxplot(
            plot_data,
            programs,
            title,
            y_axis,
            series_labels,
        )
