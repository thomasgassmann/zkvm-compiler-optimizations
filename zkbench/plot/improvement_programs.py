from zkbench.config import get_default_profiles_ids, get_programs, get_zkvms
from zkbench.plot.common import (
    get_average_improvement_over_baseline,
    get_title,
    plot_sorted,
)


def plot_improvement_number_of_programs(
    dir: str,
    measurement: str,
    drop_below: float | None = None,
    profiles: list[str] | None = None,
):
    values = []
    labels = list(
        get_default_profiles_ids()
        if profiles is None or len(profiles) == 0
        else profiles
    )
    zkvms = get_zkvms()
    for zkvm in zkvms:
        pos_values = []
        neg_values = []
        for profile in labels:
            pos = 0
            neg = 0
            for program in get_programs():
                speedup_percentage = get_average_improvement_over_baseline(
                    dir, zkvm, program, profile, measurement, speedup=False
                )
                if drop_below is not None and abs(speedup_percentage) <= drop_below:
                    continue
                if speedup_percentage > 0:
                    pos += 1
                elif speedup_percentage < 0:
                    neg -= 1
            pos_values.append(pos)
            neg_values.append(neg)

        values.append(pos_values)
        values.append(neg_values)

    values = [
        values[0],
        values[2],
        values[1],
        values[3]
    ]

    title = get_title(
        f"Number of programs with significant improvement or degradation",
        [measurement],
    )

    series = [
        zkvms[0] + " (positive)",
        zkvms[1] + " (positive)",
        zkvms[0] + " (negative)",
        zkvms[1] + " (negative)"
    ]
    plot_sorted(values, labels, title, "Number of programs", series, num_series_labels=2)
