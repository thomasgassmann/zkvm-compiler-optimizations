from zkbench.config import get_profiles_ids
from zkbench.plot.common import (
    get_mean_ms,
    get_title,
    get_values_by_profile,
    plot_grouped_boxplot,
)


def plot_average_duration(
    dir: str, zkvm: str | None, measurement: str, program: str | None
):
    title = get_title(
        "Duration by profile, sorted by median", [zkvm, measurement, program]
    )
    profiles = get_profiles_ids()
    values = get_values_by_profile(
        dir,
        zkvm,
        measurement,
        program,
        profiles,
        lambda dir, program, zkvm, profile, measurement: get_mean_ms(
            dir, program, zkvm, profile, measurement
        ),
    )
    plot_grouped_boxplot([values], profiles, title, "Duration (ms)", [])
