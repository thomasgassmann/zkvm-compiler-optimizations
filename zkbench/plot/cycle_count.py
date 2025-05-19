from zkbench.config import (
    get_default_profiles_ids,
    get_measurements,
    get_zkvms,
)
from zkbench.plot.common import (
    BASELINE,
    get_cycle_count,
    get_title,
    get_values_by_profile,
    plot_grouped_boxplot,
)


def f(dir, program, zkvm, profile):
    baseline = get_cycle_count(dir, program, zkvm, BASELINE)
    compared = get_cycle_count(dir, program, zkvm, profile)
    if compared is None:
        return None

    return (compared - baseline) / baseline


def plot_cycle_count(dir: str, program: str | None, profiles: list[str] | None = None):
    title = get_title("Relative cycle count compared to baseline", [program])
    profiles = get_default_profiles_ids() if profiles is None else profiles
    if BASELINE in profiles:
        profiles.remove(BASELINE)
    values = []
    series = []
    for zkvm in get_zkvms():
        values.append(
            get_values_by_profile(
                dir,
                zkvm,
                [get_measurements()[0]],  # can be arbitrary
                program,
                None,
                profiles,
                lambda dir, program, zkvm, profile, _: f(dir, program, zkvm, profile),
            )
        )
        series.append(zkvm)
    plot_grouped_boxplot(values, profiles, title, "Relative cycle count", series)
