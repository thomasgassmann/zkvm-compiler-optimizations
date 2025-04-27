import logging

import numpy as np
from zkbench.config import get_programs, get_zkvms
from zkbench.plot.common import (
    BASELINE,
    get_point_estimate_mean_ms,
    get_title,
    plot_grouped_boxplot,
    plot_sorted,
)


# for each program plot the average improvement this profile has over baseline
def plot_opt_by_program(dir: str, profile: str, zkvm: str | None):
    programs = get_programs()
    title = get_title(f"Average improvement by program for {profile}", [])
    relative_improvements_prove = []
    relative_improvements_exec = []
    plotted_programs = []
    for program in programs:
        try:
            exec_values = []
            prove_values = []
            for current_zkvm in get_zkvms() if not zkvm else [zkvm]:
                prove = get_point_estimate_mean_ms(
                    dir, program, current_zkvm, profile, "prove"
                )
                exec = get_point_estimate_mean_ms(
                    dir, program, current_zkvm, profile, "exec"
                )
                prove_baseline = get_point_estimate_mean_ms(
                    dir, program, current_zkvm, BASELINE, "prove"
                )
                exec_baseline = get_point_estimate_mean_ms(
                    dir, program, current_zkvm, BASELINE, "exec"
                )

                exec_values.append((exec_baseline - exec) / exec_baseline)
                prove_values.append((prove_baseline - prove) / prove_baseline)

            relative_improvements_exec.append(exec_values)
            relative_improvements_prove.append(prove_values)
            plotted_programs.append(program)
        except FileNotFoundError:
            logging.warning(f"Data for {program}-{current_zkvm}-{profile} not found")

    if len(relative_improvements_exec[0]) == 1:
        # if we only have one value, no need to plot boxplot
        prove_values = np.squeeze(relative_improvements_prove, axis=1)
        exec_values = np.squeeze(relative_improvements_exec, axis=1)
        plot_sorted(
            [
                prove_values,
                exec_values,
            ],
            plotted_programs,
            title,
            "relative duration improvement percentage",
            ["prove", "exec"],
        )
    else:
        plot_grouped_boxplot(
            [relative_improvements_prove, relative_improvements_exec],
            plotted_programs,
            title,
            "relative improvement compared to baseline",
            ["prove", "exec"],
        )
