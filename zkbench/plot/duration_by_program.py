import logging
from zkbench.config import get_programs, get_zkvms
from zkbench.plot.common import (
    get_point_estimate_mean_ms,
    get_title,
    plot_grouped_boxplot,
)


def plot_duration_by_program(dir: str, profile: str, baseline_profile: str, measurement: str):
    title = get_title(
        f"Duration for {profile} compared to {baseline_profile}",
        [],
    )

    durations_profile = []
    durations_baseline = []
    programs = []
    for program in get_programs():
        err = False
        current_profile = []
        current_baseline = []
        for zkvm in get_zkvms():
            try:
                p = get_point_estimate_mean_ms(dir, program, zkvm, profile, measurement)
                p_baseline = get_point_estimate_mean_ms(
                    dir, program, zkvm, baseline_profile, measurement
                )
                current_profile.append(p)
                current_baseline.append(p_baseline)
            except FileNotFoundError:
                logging.warning(
                    f"File not found for {program} {zkvm} {profile} {baseline_profile}. Skipping."
                )
                err = True
                break
        if err:
            continue
        programs.append(program)
        durations_profile.append(current_profile)
        durations_baseline.append(current_baseline)

    plot_grouped_boxplot(
        [
            durations_profile,
            durations_baseline,
        ],
        programs,
        title,
        "Duration (ms)",
        [
            f"{measurement}-{profile}",
            f"{measurement}-{baseline_profile}",
        ],
    )
