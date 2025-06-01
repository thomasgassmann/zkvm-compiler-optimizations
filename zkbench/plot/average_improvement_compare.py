import numpy as np
from zkbench.config import get_default_profiles_ids
from zkbench.plot.common import (
    BASELINE,
    get_point_estimate_median_ms,
    get_title,
    get_values_by_profile,
    plot_grouped_boxplot,
    plot_sorted,
)


def plot_average_improvement_compare(
    dir: str,
    zkvm_a: str,
    zkvm_b: str,
    measurement_a: str,
    measurement_b: str,
    program: str | None,
    program_group: str | None,
    speedup: bool,
):
    def f(dir, program, zkvm, profile, measurement):
        baseline = get_point_estimate_median_ms(
            dir, program, zkvm, BASELINE, measurement
        )
        compared = get_point_estimate_median_ms(
            dir, program, zkvm, profile, measurement
        )
        if speedup:
            return baseline / compared
        return (-(compared - baseline) / baseline) * 100

    y_axis = "speedup" if speedup else "% faster"
    title = get_title(
        f"Average {y_axis} by profile compared to baseline ({zkvm_a} vs {zkvm_b})",
        [zkvm_a, zkvm_b, program, program_group],
    )

    profiles = get_default_profiles_ids()
    profiles.remove(BASELINE)
    relative_improvements_a = get_values_by_profile(
        dir, zkvm_a, measurement_a, program, program_group, profiles, f
    )
    relative_improvements_b = get_values_by_profile(
        dir, zkvm_b, measurement_b, program, program_group, profiles, f
    )

    if len(relative_improvements_a[0]) == 1:
        relative_improvements_a = np.squeeze(relative_improvements_a, axis=1)
        relative_improvements_b = np.squeeze(relative_improvements_b, axis=1)
        plot_sorted(
            [
                relative_improvements_a,
                relative_improvements_b,
            ],
            profiles,
            title,
            y_axis,
            [zkvm_a, zkvm_b],
        )
    else:
        plot_grouped_boxplot(
            [relative_improvements_a, relative_improvements_b],
            profiles,
            title,
            y_axis,
            [zkvm_a, zkvm_b],
        )
