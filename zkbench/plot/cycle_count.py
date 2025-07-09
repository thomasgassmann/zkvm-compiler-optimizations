import numpy as np
from zkbench.config import (
    get_default_profiles_ids,
    get_measurements,
    get_zkvms,
)
from zkbench.plot.common import (
    BASELINE,
    get_average_improvement_over_baseline,
    get_cycle_count,
    get_title,
    get_values_by_profile,
    plot_grouped_boxplot,
    plot_sorted,
)


def f(dir, program, zkvm, profile):
    baseline = get_cycle_count(dir, program, zkvm, BASELINE)
    compared = get_cycle_count(dir, program, zkvm, profile)
    if compared is None:
        return None

    return (compared - baseline) / baseline * 100


def plot_cycle_count(
    dir: str,
    program: str | None,
    program_group: str | None = None,
    profiles: list[str] | None = None,
    global_average: bool = False,
    show_x86: bool = False,
):
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
                program_group,
                profiles,
                lambda dir, program, zkvm, profile, _: f(dir, program, zkvm, profile),
            )
        )
        series.append(zkvm)

    if show_x86:
        series.append("x86")
        values.append(
            get_values_by_profile(
                dir,
                "x86",
                "exec",
                program,
                program_group,
                profiles,
                lambda dir, program, _zk, profile, _: get_average_improvement_over_baseline(
                    dir, "x86", program, profile, "exec", False
                ),
            )
        )

    title = get_title(
        (
            "Relative cycle count compared to baseline"
            if not show_x86
            else "Relative cycle count/x86 exec time compared to baseline"
        ),
        [program, program_group],
    )
    y_label = (
        "Relative change in cycle count (%)"
        if not show_x86
        else "Relative change in cycle count/x86 exec time (%)"
    )
    if global_average:
        for i in range(len(values)):
            values[i] = np.mean(values[i], axis=1)
        plot_sorted(values, profiles, title, y_label, series)
    else:
        plot_grouped_boxplot(values, profiles, title, y_label, series)
