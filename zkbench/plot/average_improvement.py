import numpy as np
from zkbench.config import get_default_profiles_ids
from zkbench.plot.common import (
    BASELINE,
    get_point_estimate_mean_ms,
    get_title,
    get_values_by_profile,
    plot_grouped_boxplot,
    plot_sorted,
)


def plot_average_improvement(
    dir: str,
    zkvm: str | None,
    program: str | None,
    program_group: str | None,
    speedup: bool,
    global_average: bool,
):
    def f(dir, program, zkvm, profile, measurement):
        baseline = get_point_estimate_mean_ms(dir, program, zkvm, BASELINE, measurement)
        compared = get_point_estimate_mean_ms(dir, program, zkvm, profile, measurement)
        if speedup:
            return baseline / compared
        return (-(compared - baseline) / baseline) * 100

    title = get_title(
        "Average improvement by profile compared to baseline",
        [zkvm, program, program_group],
    )

    profiles = get_default_profiles_ids()
    profiles.remove(BASELINE)
    relative_improvements_prove = get_values_by_profile(
        dir, zkvm, "prove", program, program_group, profiles, f
    )
    relative_improvements_exec = get_values_by_profile(
        dir, zkvm, "exec", program, program_group, profiles, f
    )
    if global_average:
        relative_improvements_exec = np.mean(relative_improvements_exec, axis=1)
        relative_improvements_prove = np.mean(relative_improvements_prove, axis=1)

    y_axis = "speedup" if speedup else "% faster"
    if global_average or len(relative_improvements_exec[0]) == 1:
        # if we only have one value, no need to plot boxplot
        if not global_average:
            prove_values = np.squeeze(relative_improvements_prove, axis=1)
            exec_values = np.squeeze(relative_improvements_exec, axis=1)
        else:
            prove_values = relative_improvements_prove
            exec_values = relative_improvements_exec
        plot_sorted(
            [
                prove_values,
                exec_values,
            ],
            profiles,
            title,
            y_axis,
            ["prove", "exec"],
        )
    else:
        plot_grouped_boxplot(
            [relative_improvements_prove, relative_improvements_exec],
            profiles,
            title,
            y_axis,
            ["prove", "exec"],
        )
