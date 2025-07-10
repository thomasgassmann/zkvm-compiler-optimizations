import logging

from matplotlib import pyplot as plt
from zkbench.config import (
    get_default_profiles_ids,
    get_programs,
    get_zkvms,
)
from zkbench.plot.common import (
    BASELINE,
    get_cycle_count,
    get_pearson,
    get_point_estimate_median_ms,
    get_spearman,
    get_title,
    plot_scatter_by_zkvm,
    show_or_save_plot,
)


def _get_values(
    dir: str,
    zkvm: str,
    programs: list[str],
    profiles: list[str],
    measurement: str,
    relative: bool,
):
    x, y = [], []
    for program in programs:
        try:
            baseline_cycle_count = get_cycle_count(dir, program, zkvm, BASELINE)
            baseline_duration = get_point_estimate_median_ms(
                dir, program, zkvm, BASELINE, measurement
            )
            for profile in profiles:
                cycle_count = get_cycle_count(dir, program, zkvm, profile)
                if cycle_count is None:
                    logging.warning(
                        f"Cycle count for {program}-{zkvm}-{profile} not found"
                    )
                    continue
                duration = get_point_estimate_median_ms(
                    dir, program, zkvm, profile, measurement
                )
                if relative:
                    x.append(
                        (cycle_count - baseline_cycle_count) / baseline_cycle_count
                    )
                    y.append((duration - baseline_duration) / baseline_duration)
                else:
                    x.append(cycle_count)
                    y.append(duration)
        except FileNotFoundError as e:
            logging.warning(f"Data for {program}-{zkvm}-{measurement} not found {e}")
    return x, y


def plot_cycle_count_stats(
    dir: str, measurement: str, relative: bool
):
    profiles = get_default_profiles_ids()
    if relative:
        profiles.remove(BASELINE)

    for zkvm in get_zkvms():
        xs, ys = [], []
        for program in get_programs():
            x, y = _get_values(dir, zkvm, [program], profiles, measurement, relative)
            if len(x) == 0:
                continue

            spearman = get_spearman(x, y)
            pearson = get_pearson(x, y)
            xs.append(spearman)
            ys.append(pearson)

        plt.scatter(
            xs, ys, label=zkvm
        )

    plt.title(get_title("Cycle count/duration coefficients", [measurement]))
    plt.xlabel('Pearson coefficient')
    plt.ylabel('Spearman coefficient')
    plt.grid(linestyle="--", alpha=0.7)
    plt.legend()
    show_or_save_plot()


def plot_cycle_count_duration(
    dir: str, measurement: str, program: str | None, relative: bool
):
    profiles = get_default_profiles_ids()
    if relative:
        profiles.remove(BASELINE)
    programs = get_programs() if program is None else [program]
    plot_scatter_by_zkvm(
        get_title("Cycle count vs. duration", [measurement, program]),
        lambda zkvm: _get_values(dir, zkvm, programs, profiles, measurement, relative),
        "Cycle count",
        "Duration (ms)",
    )
