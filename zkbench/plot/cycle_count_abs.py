import numpy as np
from zkbench.config import get_default_profiles_ids
from zkbench.plot.common import (
    get_cycle_count,
    get_title,
    get_values_by_profile,
    plot_sorted,
)


def plot_cycle_count_abs(dir: str, program: str | None, zkvm: str | None):
    title = get_title("Cycle count by profile", [zkvm, program])

    profiles = get_default_profiles_ids()
    cycle_counts = get_values_by_profile(
        dir,
        zkvm,
        "prove",
        program,
        None,
        profiles,
        lambda dir, program, zkvm, profile, _: get_cycle_count(
            dir, program, zkvm, profile
        ),
        skipped_value=0,
    )

    cycle_counts = np.sum(cycle_counts, axis=1)
    plot_sorted([cycle_counts], profiles, title, "Cycle count by profile", [None])
