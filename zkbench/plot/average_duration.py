import numpy as np
from zkbench.config import get_profiles_ids
from zkbench.plot.common import (
    get_point_estimate_mean_ms,
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
):
    title = get_title(
        "Duration by profile, sorted by median", [zkvm, measurement, program]
    )
    profiles = get_profiles_ids() if profiles is None else profiles
    values = get_values_by_profile(
        dir,
        zkvm,
        measurement,
        program,
        None,
        profiles,
        lambda dir, program, zkvm, profile, measurement: get_point_estimate_mean_ms(
            dir, program, zkvm, profile, measurement
        ),
    )
    # TODO: consider all values that criterion recorded
    if not program or not zkvm:
        plot_grouped_boxplot([values], profiles, title, "Duration (ms)", [])
    else:
        values = np.squeeze(values, axis=1)
        plot_sorted([values], profiles, title, "Duration (ms)", [None])
