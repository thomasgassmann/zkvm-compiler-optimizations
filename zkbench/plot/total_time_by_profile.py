import numpy as np
from zkbench.config import get_default_profiles_ids
from zkbench.plot.common import (
    get_point_estimate_median_ms,
    get_title,
    get_values_by_profile,
    plot_sorted,
)


def plot_total_time_by_profile(dir: str, zkvm: str | None, program: str | None, measurement: str):
    title = get_title("Total exec/prove time by profile", [zkvm, program])

    profiles = get_default_profiles_ids()
    values = get_values_by_profile(
        dir,
        zkvm,
        measurement,
        program,
        None,
        profiles,
        lambda dir, program, zkvm, profile, measurement: get_point_estimate_median_ms(
            dir, program, zkvm, profile, measurement
        ),
        skipped_value=0,
    )

    res = np.sum(values, axis=1) / 1_000
    plot_sorted([res], profiles, title, 'Total prove time (s)', [None])
