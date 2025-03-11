from zkbench.config import get_profiles_ids
from zkbench.plot.common import (
    BASELINE,
    get_average_across,
    get_mean_ms,
    get_title,
    plot_sorted,
)


def f(dir, program, zkvm, profile, measurement):
    baseline = get_mean_ms(dir, program, zkvm, BASELINE, measurement)
    compared = get_mean_ms(dir, program, zkvm, profile, measurement)
    return -(compared - baseline) / baseline


def plot_average_improvement(dir: str, zkvm: str | None, measurement: str | None, program: str | None):
    title = get_title("Average improvement by profile", [zkvm, measurement, program])

    profiles = get_profiles_ids()
    profiles.remove(BASELINE)
    relative_improvements = get_average_across(
        dir, zkvm, measurement, program, profiles, f
    )

    plot_sorted(
        relative_improvements,
        profiles,
        title,
        "improvement/degradation compared to baseline",
    )
