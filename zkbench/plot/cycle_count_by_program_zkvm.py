import logging

import numpy as np
from zkbench.config import get_programs, get_zkvms
from zkbench.plot.common import (
    get_cycle_count,
    get_title,
    plot_sorted,
)


def plot_cycle_count_by_program_zkvm(
    dir: str,
    profile: str,
    baseline_profile: str,
    drop_below: float | None = None,
):
    title = get_title(f"Change in cycle count", [])

    programs = []
    zkvms = get_zkvms()
    values = [[] for _ in zkvms]
    for program in get_programs():
        err = False
        for zkvm_idx, zkvm in enumerate(zkvms):
            try:
                cycle_count_baseline = get_cycle_count(
                    dir, program, zkvm, baseline_profile
                )
                cycle_count = get_cycle_count(dir, program, zkvm, profile)
                change = (
                    (cycle_count - cycle_count_baseline)
                    / cycle_count_baseline
                    * 100
                )
                values[zkvm_idx].append(change)
            except FileNotFoundError:
                logging.warning(
                    f"File not found for {program} {zkvm}. Skipping."
                )
                err = True
                break
        if err:
            continue
        programs.append(program)

    for i in range(len(values)):
        logging.info("Average cycle count change for zkVM %s: %.2f%%", zkvms[i], np.mean(values[i]))

    for i in range(len(programs)):
        logging.info(
            "Average cycle count change for program %s (%s, %s): (%.2f%%, %.2f%%)",
            programs[i],
            zkvms[0],
            zkvms[1],
            values[0][i],
            values[1][i],
        )

    plot_sorted(
        values,
        programs,
        title,
        "Change in Cycle Count (%)",
        zkvms,
        drop_below=drop_below,
    )