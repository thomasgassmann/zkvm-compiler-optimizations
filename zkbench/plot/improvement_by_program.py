import logging
from zkbench.config import get_programs, get_zkvms
from zkbench.plot.common import get_point_estimate_mean_ms, get_title, plot_grouped_boxplot


def plot_improvement_by_program(dir: str, profile: str, baseline_profile: str, speedup: bool):
    
    def f(dir, program, zkvm, measurement):
        baseline = get_point_estimate_mean_ms(dir, program, zkvm, baseline_profile, measurement)
        compared = get_point_estimate_mean_ms(dir, program, zkvm, profile, measurement)
        if speedup:
            return baseline / compared
        return (-(compared - baseline) / baseline) * 100

    title = get_title(
        f"Average improvement for {profile} compared to {baseline_profile}",
        [],
    )

    relative_improvements_prove = []
    relative_improvements_exec = []
    programs = []
    for program in get_programs():
        err = False
        current_improvements_prove = []
        current_improvements_exec = []
        for zkvm in get_zkvms():
            try:
                p = f(dir, program, zkvm, "prove")
                e = f(dir, program, zkvm, "exec")
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


    y_axis = "speedup" if speedup else "% faster"
    plot_grouped_boxplot(
        [relative_improvements_prove, relative_improvements_exec],
        programs,
        title,
        y_axis,
        ["prove", "exec"],
    )

