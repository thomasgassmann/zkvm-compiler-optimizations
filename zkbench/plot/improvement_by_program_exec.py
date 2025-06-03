import logging
from zkbench.config import get_programs
from zkbench.plot.common import (
    get_point_estimate_median_ms,
    get_title,
    plot_grouped_boxplot,
)


def plot_improvement_by_program_exec(
    dir: str, profile: str, baseline_profile: str, speedup: bool, show_x86: bool = False
):

    def f(dir, program, zkvm, measurement):
        baseline = get_point_estimate_median_ms(
            dir, program, zkvm, baseline_profile, measurement
        )
        compared = get_point_estimate_median_ms(
            dir, program, zkvm, profile, measurement
        )
        if speedup:
            return baseline / compared
        return (-(compared - baseline) / baseline) * 100

    title = get_title(
        f"Average improvement for {profile} compared to {baseline_profile}",
        [],
    )

    relative_improvements_exec_risc0 = []
    relative_improvements_exec_sp1 = []
    relative_improvements_exec_x86 = []
    programs = []
    for program in get_programs():
        current_improvements_exec_risc0 = []
        current_improvements_exec_sp1 = []
        current_improvements_exec_x86 = []
        try:
            sp1 = f(dir, program, "sp1", "exec")
            risc0 = f(dir, program, "risc0", "exec")
            if show_x86:
                x86 = f(dir, program, "x86", "exec")
                current_improvements_exec_x86.append(x86)
            current_improvements_exec_risc0.append(risc0)
            current_improvements_exec_sp1.append(sp1)
        except FileNotFoundError:
            logging.warning(
                f"File not found for {program} {profile} {baseline_profile}. Skipping."
            )
            continue
        programs.append(program)
        relative_improvements_exec_risc0.append(current_improvements_exec_risc0)
        relative_improvements_exec_sp1.append(current_improvements_exec_sp1)
        relative_improvements_exec_x86.append(current_improvements_exec_x86)

    y_axis = "speedup" if speedup else "% faster"
    plot_grouped_boxplot(
        (
            [relative_improvements_exec_risc0, relative_improvements_exec_sp1]
            if not show_x86
            else [
                relative_improvements_exec_x86,
                relative_improvements_exec_risc0,
                relative_improvements_exec_sp1,
            ]
        ),
        programs,
        title,
        y_axis,
        ["risc0", "sp1"] if not show_x86 else ["x86", "risc0", "sp1"],
        bar_width=0.35 if not show_x86 else 0.2,
    )
