import numpy as np
from zkbench.config import get_profiles_ids
from zkbench.plot.common import (
    get_cycle_count,
    get_point_estimate_mean_ms,
    get_title,
    get_values_by_profile,
    plot_grouped_boxplot,
    plot_sorted,
)


def plot_khz(
    dir: str, zkvm: str | None, program: str | None
):
    title = get_title(
        "Prove kHz by profile", [zkvm, program]
    )

    def get_khz(program: str, zkvm: str, profile: str):
        prove_time = get_point_estimate_mean_ms(dir, program, zkvm, profile, 'prove')
        cycle_count = get_cycle_count(dir, program, zkvm, profile)
        if cycle_count is None:
            return 0

        return cycle_count / prove_time

    profiles = get_profiles_ids()
    values = get_values_by_profile(
        dir,
        zkvm,
        'prove',
        program,
        None,
        profiles,
        lambda dir, program, zkvm, profile, measurement: get_khz(
            program, zkvm, profile
        ),
    )
    if not program or not zkvm:
        plot_grouped_boxplot([values], profiles, title, "kHz", [])
    else:
        values = np.squeeze(values, axis=1)
        plot_sorted([values], profiles, title, "kHz", [None])
