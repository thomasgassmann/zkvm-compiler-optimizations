import numpy as np
from zkbench.config import get_default_profiles_ids
from zkbench.plot.common import (
    get_point_estimate_median_ms,
    get_sample_times_ms,
    get_title,
    get_values_by_profile,
    plot_grouped_boxplot,
    plot_sorted,
)


def plot_average_duration(
    dir: str,
    zkvm: str | None,
    measurement: str,
    program: str | None,
    profiles: list[str] | None = None,
    single: bool = False,
):
    title = get_title(
        "Duration by profile, sorted by median", [zkvm, measurement, program]
    )
    profiles = (
        get_default_profiles_ids()
        if profiles is None or len(profiles) == 0
        else profiles
    )
    values = get_values_by_profile(
        dir,
        zkvm,
        measurement,
        program,
        None,
        profiles,
        lambda dir, program, zkvm, profile, measurement: get_sample_times_ms(
            dir, program, zkvm, profile, measurement
        ) if not single else get_point_estimate_median_ms(dir, program, zkvm, profile, measurement),
    )

    if single:
        values = np.array(values).squeeze(axis=1)
        plot_sorted([values], profiles, title, "Duration (ms)", [None])
    else:
        plot_grouped_boxplot([values], profiles, title, "Duration (ms)", [])
