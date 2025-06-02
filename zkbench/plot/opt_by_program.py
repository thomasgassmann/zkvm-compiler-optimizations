import logging

import numpy as np
from zkbench.config import get_programs, get_zkvms
from zkbench.plot.common import (
    BASELINE,
    get_point_estimate_median_ms,
    get_title,
    plot_grouped_boxplot,
    plot_sorted,
)


# for each program plot the average improvement this profile has over baseline
def plot_opt_by_program(
    dir: str, profile: str, zkvm: str | None, speedup: bool = False
):
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
                exec = get_point_estimate_median_ms(
                    dir, program, current_zkvm, profile, "exec"
                )
                exec_baseline = get_point_estimate_median_ms(
                    dir, program, current_zkvm, BASELINE, "exec"
                )
                if zkvm == "x86":
                    # x86 has no prove, use average of zkVM exec
                    prove = np.mean(
                        [
                            get_point_estimate_median_ms(
                                dir, program, z, profile, "exec"
                            )
                            for z in get_zkvms()
                        ]
                    )
                    prove_baseline = np.mean(
                        [
                            get_point_estimate_median_ms(
                                dir, program, z, BASELINE, "exec"
                            )
                            for z in get_zkvms()
                        ]
                    )
                else:
                    prove_baseline = get_point_estimate_median_ms(
                        dir, program, current_zkvm, BASELINE, "prove"
                    )
                    prove = get_point_estimate_median_ms(
                        dir, program, current_zkvm, profile, "prove"
                    )

                if speedup:
                    exec_values.append(exec_baseline / exec)
                    prove_values.append(prove_baseline / prove)
                else:
                    exec_values.append((exec_baseline - exec) / exec_baseline)
                    prove_values.append((prove_baseline - prove) / prove_baseline)

            relative_improvements_exec.append(exec_values)
            relative_improvements_prove.append(prove_values)
            plotted_programs.append(program)
        except FileNotFoundError:
            logging.warning(f"Data for {program}-{current_zkvm}-{profile} not found")

    y_axis = "speedup" if speedup else "% faster"
    labels = ["prove", "exec"] if zkvm != "x86" else ["exec-x86", "exec-zkvm (avg)"]
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
            y_axis,
            labels,
        )
    else:
        plot_grouped_boxplot(
            [relative_improvements_prove, relative_improvements_exec],
            plotted_programs,
            title,
            y_axis,
            labels,
        )
