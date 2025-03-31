import numpy as np
from zkbench.config import get_profiles_ids
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
):
    def f(dir, program, zkvm, profile, measurement):
        baseline = get_point_estimate_mean_ms(dir, program, zkvm, BASELINE, measurement)
        compared = get_point_estimate_mean_ms(dir, program, zkvm, profile, measurement)
        if speedup:
            return baseline / compared
        return -(compared - baseline) / baseline

    title = get_title(
        "Average improvement by profile compared to baseline",
        [zkvm, program, program_group],
    )

    profiles = get_profiles_ids()
    profiles.remove(BASELINE)
    relative_improvements_prove = get_values_by_profile(
        dir, zkvm, "prove", program, program_group, profiles, f
    )
    relative_improvements_exec = get_values_by_profile(
        dir, zkvm, "exec", program, program_group, profiles, f
    )

    if len(relative_improvements_exec[0]) == 1:
        # if we only have one value, no need to plot boxplot
        prove_values = np.squeeze(relative_improvements_prove, axis=1)
        exec_values = np.squeeze(relative_improvements_exec, axis=1)
        plot_sorted(
            [
                prove_values,
                exec_values,
            ],
            profiles,
            title,
            "relative duration improvement percentage",
            ["prove", "exec"],
        )
    else:
        plot_grouped_boxplot(
            [relative_improvements_prove, relative_improvements_exec],
            profiles,
            title,
            "relative duration improvement percentage",
            ["prove", "exec"],
        )
