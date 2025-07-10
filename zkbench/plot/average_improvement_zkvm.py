import numpy as np
from zkbench.config import get_default_profiles_ids, get_zkvms
from zkbench.plot.common import (
    BASELINE,
    get_average_improvement_over_baseline,
    get_title,
    get_values_by_profile,
    plot_grouped_boxplot,
    plot_sorted,
)


def plot_average_improvement_zkvm(
    dir: str,
    measurement: str,
    program: str | None,
    program_group: str | None,
    speedup: bool,
    global_average: bool,
    drop_below: float | None = None,
):
    profiles = get_default_profiles_ids()
    if BASELINE in profiles:
        profiles.remove(BASELINE)
    values = []
    series = []
    for zkvm in get_zkvms():
        values.append(
            get_values_by_profile(
                dir,
                zkvm,
                measurement,
                program,
                program_group,
                profiles,
                lambda dir, program, zkvm, profile, m: get_average_improvement_over_baseline(
                    dir, zkvm, program, profile, m, speedup
                ),
            )
        )
        series.append(zkvm)

    title = get_title(
        f"Improvement over baseline ({measurement})",
        [program, program_group],
    )
    y_label = "speedup (%)"
    if global_average:
        for i in range(len(values)):
            values[i] = np.mean(values[i], axis=1)

        plot_sorted(values, profiles, title, y_label, series, drop_below=drop_below)
    else:
        if drop_below:
            raise ValueError("drop_below is not supported for grouped boxplots")

        plot_grouped_boxplot(values, profiles, title, y_label, series)
