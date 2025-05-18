import logging
from zkbench.config import get_programs, get_zkvms
from zkbench.plot.common import (
    get_sample_times_ms,
    get_title,
    plot_grouped_boxplot,
)


def plot_duration_by_program(
    dir: str,
    profile: str,
    baseline_profile: str,
    measurement: str,
    zkvm: str | None = None,
):
    title = get_title(
        f"Duration for {profile} compared to {baseline_profile}",
        [measurement, zkvm],
    )

    durations_profile = []
    durations_baseline = []
    programs = []
    for program in get_programs():
        err = False
        current_profile = []
        current_baseline = []
        for current_zkvm in get_zkvms() if zkvm is None else [zkvm]:
            try:
                p = get_sample_times_ms(
                    dir, program, current_zkvm, profile, measurement
                )
                p_baseline = get_sample_times_ms(
                    dir, program, current_zkvm, baseline_profile, measurement
                )
                current_profile.extend(p)
                current_baseline.extend(p_baseline)
            except FileNotFoundError:
                logging.warning(f"Skipping {program}")
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
