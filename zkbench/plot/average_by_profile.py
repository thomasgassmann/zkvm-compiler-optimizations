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


def f(dir, program, zkvm, profile, measurement):
    baseline = get_point_estimate_mean_ms(dir, program, zkvm, BASELINE, measurement)
    compared = get_point_estimate_mean_ms(dir, program, zkvm, profile, measurement)
    return -(compared - baseline) / baseline


def plot_average_improvement(dir: str, zkvm: str | None, program: str | None):
    title = get_title(
        "Average improvement by profile compared to baseline", [zkvm, program]
    )

    profiles = get_profiles_ids()
    profiles.remove(BASELINE)
    relative_improvements_prove = get_values_by_profile(
        dir, zkvm, "prove", program, profiles, f
    )
    relative_improvements_exec = get_values_by_profile(
        dir, zkvm, "exec", program, profiles, f
    )

    if not zkvm or not program:
        plot_grouped_boxplot(
            [relative_improvements_prove, relative_improvements_exec],
            profiles,
            title,
            "relative duration improvement percentage",
            ["prove", "exec"],
        )
    else:
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
