import logging

import numpy as np
from zkbench.config import get_programs, get_zkvm_display_name, get_zkvms
from zkbench.plot.common import (
    get_average_improvement_over_baseline,
    get_title,
    plot_grouped_boxplot,
    plot_sorted,
)


def plot_improvement_by_program_zkvm(
    dir: str,
    profile: str,
    baseline_profile: str,
    speedup: bool,
    measurement: str,
    drop_below: float | None = None,
):

    def f(dir, program, zkvm, measurement):
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
        f"Improvement compared to original toolchain",
        [],
    )

    zkvms = get_zkvms()
    values = [[] for _ in zkvms]
    programs = []
    for program in get_programs():
        err = False
        for zkvm_idx, zkvm in enumerate(zkvms):
            try:
                p = f(dir, program, zkvm, measurement)
                values[zkvm_idx].append(p)
            except FileNotFoundError:
                logging.warning(
                    f"File not found for {program} {zkvm} {profile} {baseline_profile}. Skipping."
                )
                err = True
                break
        if err:
            continue
        programs.append(program)


    for i, zkvm in enumerate(zkvms):
        logging.info("Average improvement for zkVM %s: %.2f%%", zkvm, np.mean(values[i]))


    for zkvm in zkvms:
        num_neg = len([v for v in values[zkvms.index(zkvm)] if v < -drop_below ])
        num_pos = len([v for v in values[zkvms.index(zkvm)] if v > drop_below ])
        logging.info(
            f"Number of programs for {zkvm}: {num_pos} positive, {num_neg} negative"
        )

    for program in programs:
        values_for_program = [values[i][programs.index(program)] for i in range(len(zkvms))]
        logging.info(
            "Average improvement for program %s (%s, %s): (%.2f%%, %.2f%%)",
            program,
            zkvms[0],
            zkvms[1],
            values_for_program[0],
            values_for_program[1],
        )

    y_axis = "speedup" if speedup else "speedup (%)"
    plot_sorted(
        values,
        programs,
        title,
        y_axis,
        [get_zkvm_display_name(zkvm) for zkvm in zkvms],
        drop_below=drop_below,
    )
