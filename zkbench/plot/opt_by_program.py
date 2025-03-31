import logging
from zkbench.config import get_programs, get_zkvms
from zkbench.plot.common import (
    BASELINE,
    get_point_estimate_mean_ms,
    get_title,
    plot_grouped_boxplot,
)


# for each program plot the average improvement this profile has over baseline
def plot_opt_by_program(dir: str, profile: str):
    programs = get_programs()
    title = get_title(f"Average improvement by program for {profile}", [])
    relative_improvements_prove = []
    relative_improvements_exec = []
    plotted_programs = set()
    for program in programs:
        try:
            exec_values = []
            prove_values = []
            for zkvm in get_zkvms():
                prove = get_point_estimate_mean_ms(dir, program, zkvm, profile, "prove")
                exec = get_point_estimate_mean_ms(dir, program, zkvm, profile, "exec")
                prove_baseline = get_point_estimate_mean_ms(
                    dir, program, zkvm, BASELINE, "prove"
                )
                exec_baseline = get_point_estimate_mean_ms(
                    dir, program, zkvm, BASELINE, "exec"
                )

                exec_values.append((exec_baseline - exec) / exec_baseline)
                prove_values.append((prove_baseline - prove) / prove_baseline)

            relative_improvements_exec.append(exec_values)
            relative_improvements_prove.append(prove_values)
            plotted_programs.add(program)
        except FileNotFoundError:
            logging.warning(f"Data for {program}-{zkvm}-{profile} not found")

    plot_grouped_boxplot(
        [relative_improvements_prove, relative_improvements_exec],
        list(plotted_programs),
        title,
        "relative improvement compared to baseline",
        ["prove", "exec"],
    )
