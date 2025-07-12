import logging

import numpy as np
from zkbench.config import get_programs, get_zkvms
from zkbench.plot.common import (
    get_average_improvement_over_baseline,
    get_title,
    plot_grouped_boxplot,
    plot_sorted,
)


# for each program plot the average improvement this profile has over baseline
def plot_opt_by_program(
    dir: str,
    profile: str,
    zkvm: str | None,
    speedup: bool = False,
    show_x86: bool = False,
):
    programs = get_programs()
    title = get_title(f"Average improvement by program for {profile}", [])
    relative_improvements_prove = []
    relative_improvements_exec = []
    relative_improvements_x86 = []
    plotted_programs = []
    all_zkvms = get_zkvms() if not zkvm else [zkvm]
    for program in programs:
        try:
            exec_values = []
            prove_values = []
            x86_values = []
            for current_zkvm in all_zkvms:
                exec_improvement = get_average_improvement_over_baseline(
                    dir, current_zkvm, program, profile, "exec", speedup=speedup
                )
                x86_improvement = get_average_improvement_over_baseline(
                    dir, "x86", program, profile, "exec", speedup=speedup
                )
                if zkvm == "x86":
                    # x86 has no prove, use average of zkVM exec
                    other_zkvm_improvement_average = np.mean(
                        [
                            get_average_improvement_over_baseline(
                                dir, z, program, profile, "exec", speedup=speedup
                            )
                            for z in get_zkvms()
                        ]
                    )
                    exec_values.append(exec_improvement)
                    prove_values.append(other_zkvm_improvement_average)
                else:
                    prove_improvement = get_average_improvement_over_baseline(
                        dir, current_zkvm, program, profile, "prove", speedup=speedup
                    )
                    exec_values.append(exec_improvement)
                    prove_values.append(prove_improvement)
                x86_values.append(x86_improvement)

            relative_improvements_exec.append(exec_values)
            relative_improvements_prove.append(prove_values)
            relative_improvements_x86.append(x86_values)
            plotted_programs.append(program)
        except FileNotFoundError:
            logging.warning(f"Data for {program}-{current_zkvm}-{profile} not found")

    logging.info("zkVMs: %s", all_zkvms)
    logging.info("Average improvements (exec): %s", np.mean(relative_improvements_exec, axis=0))
    logging.info("Average improvements (prove): %s", np.mean(relative_improvements_prove, axis=0))
    logging.info("Average improvements (x86): %s", np.mean(relative_improvements_x86, axis=0))

    y_axis = "speedup" if speedup else "% faster"
    labels = ["prove", "exec"] if zkvm != "x86" else ["exec-x86", "exec-zkvm (avg)"]
    if show_x86:
        labels.append("exec-x86")
    if len(relative_improvements_exec[0]) == 1:
        # if we only have one value, no need to plot boxplot
        prove_values = np.squeeze(relative_improvements_prove, axis=1)
        exec_values = np.squeeze(relative_improvements_exec, axis=1)
        x86_values = np.squeeze(relative_improvements_x86, axis=1)
        plot_sorted(
            (
                [
                    prove_values,
                    exec_values,
                ]
                if not show_x86
                else [
                    prove_values,
                    exec_values,
                    x86_values,
                ]
            ),
            plotted_programs,
            title,
            y_axis,
            labels,
        )
    else:
        plot_grouped_boxplot(
            (
                [relative_improvements_prove, relative_improvements_exec]
                if not show_x86
                else [
                    relative_improvements_prove,
                    relative_improvements_exec,
                    relative_improvements_x86,
                ]
            ),
            plotted_programs,
            title,
            y_axis,
            labels,
        )
