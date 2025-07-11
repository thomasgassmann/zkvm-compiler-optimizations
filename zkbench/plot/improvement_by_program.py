import logging

import numpy as np
from zkbench.config import get_programs, get_zkvms
from zkbench.plot.common import (
    get_average_improvement_over_baseline,
    get_title,
    plot_grouped_boxplot,
)


def plot_improvement_by_program(
    dir: str, profile: str, baseline_profile: str, speedup: bool, show_x86: bool = False
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
        f"Improvement compared to {baseline_profile}",
        [],
    )

    relative_improvements_prove = []
    relative_improvements_exec = []
    relative_improvements_exec_x86 = []
    programs = []
    zkvms = get_zkvms()
    for program in get_programs():
        err = False
        current_improvements_prove = []
        current_improvements_exec = []
        current_improvements_exec_x86 = []
        for zkvm in zkvms:
            try:
                p = f(dir, program, zkvm, "prove")
                e = f(dir, program, zkvm, "exec")
                if show_x86:
                    x86 = f(dir, program, "x86", "exec")
                    current_improvements_exec_x86.append(x86)
                current_improvements_prove.append(p)
                current_improvements_exec.append(e)
            except FileNotFoundError:
                logging.warning(
                    f"File not found for {program} {zkvm} {profile} {baseline_profile}. Skipping."
                )
                err = True
                break
        if err:
            continue
        programs.append(program)
        relative_improvements_prove.append(current_improvements_prove)
        relative_improvements_exec.append(current_improvements_exec)
        relative_improvements_exec_x86.append(current_improvements_exec_x86)

    logging.info("All values (prove, %s, %s): %s", zkvms, programs, relative_improvements_prove)
    logging.info("All values (exec, %s, %s): %s", zkvms, programs, relative_improvements_exec)
    logging.info("Prove speedups (%s): %s", zkvms, np.mean(relative_improvements_prove, axis=0))
    logging.info("Exec speedups (%s): %s", zkvms, np.mean(relative_improvements_exec, axis=0))

    y_axis = "speedup" if speedup else "speedup (%)"
    plot_grouped_boxplot(
        (
            [relative_improvements_prove, relative_improvements_exec]
            if not show_x86
            else [
                relative_improvements_prove,
                relative_improvements_exec,
                relative_improvements_exec_x86,
            ]
        ),
        programs,
        title,
        y_axis,
        ["prove", "exec"] if not show_x86 else ["prove", "exec", "x86"],
        bar_width=0.35 if not show_x86 else 0.2,
    )
