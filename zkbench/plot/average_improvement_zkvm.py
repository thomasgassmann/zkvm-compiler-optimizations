import logging
import numpy as np
from zkbench.config import get_default_profiles_ids, get_zkvms
from zkbench.plot.common import (
    BASELINE,
    get_average_improvement_over_baseline,
    get_title,
    get_values_by_profile,
    plot_grouped_boxplot,
    plot_sorted,
)


def plot_average_improvement_zkvm(
    dir: str,
    measurement: str,
    program: str | None,
    program_group: str | None,
    speedup: bool,
    global_average: bool,
    drop_below: float | None = None,
):
    profiles = get_default_profiles_ids()
    if BASELINE in profiles:
        profiles.remove(BASELINE)
    values = []
    series = []
    for zkvm in get_zkvms():
        values.append(
            get_values_by_profile(
                dir,
                zkvm,
                measurement,
                program,
                program_group,
                profiles,
                lambda dir, program, zkvm, profile, m: get_average_improvement_over_baseline(
                    dir, zkvm, program, profile, m, speedup
                ),
            )
        )
        series.append(zkvm)

    title = get_title(
        f"Improvement over baseline ({measurement})",
        [program, program_group],
    )
    y_label = "speedup (%)"
    if global_average:
        for i in range(len(values)):
            pos = {}
            neg = {}
            for j in range(len(values[i])):
                count_positives = len([v for v in values[i][j] if v > 0])
                count_negatives = len([v for v in values[i][j] if v < 0])
                if drop_below is not None:
                    count_positives = len([v for v in values[i][j] if v >= drop_below])
                    count_negatives = len([v for v in values[i][j] if v <= -drop_below])

                pos.setdefault(profiles[j], 0)
                pos[profiles[j]] += count_positives
                neg.setdefault(profiles[j], 0)
                neg[profiles[j]] += count_negatives
            for j in range(len(profiles)):
                logging.info(
                    f"Number of programs for {series[i]}-{profiles[j]}: "
                    f"{pos[profiles[j]]} positive, {neg[profiles[j]]} negative"
                )

        for i in range(len(values)):
            values[i] = np.mean(values[i], axis=1)
            c = sorted(
                [(j, value) for j, value in enumerate(values[i])], key=lambda x: x[1]
            )
            for j, _ in c:
                logging.info(
                    f"Average speedup change for {series[i]}-{profiles[j]}: {values[i][j]}"
                )

        plot_sorted(values, profiles, title, y_label, series, drop_below=drop_below)
    else:
        plot_grouped_boxplot(
            values,
            profiles,
            title,
            y_label,
            series,
            drop_below=drop_below,
            show_fliers=True,
        )
