import logging
from zkbench.config import get_profiles_ids, get_programs
from zkbench.plot.common import (
    BASELINE,
    get_cycle_count,
    get_point_estimate_ms,
    get_title,
    plot_scatter_by_zkvm,
)


def _get_values(
    dir: str, zkvm: str, programs: list[str], profiles: list[str], measurement: str
):
    x, y = [], []
    for program in programs:
        try:
            baseline_cycle_count = get_cycle_count(dir, program, zkvm, BASELINE)
            baseline_duration = get_point_estimate_ms(
                dir, program, zkvm, BASELINE, measurement
            )
            for profile in profiles:
                cycle_count = get_cycle_count(dir, program, zkvm, profile)
                duration = get_point_estimate_ms(
                    dir, program, zkvm, profile, measurement
                )
                x.append((cycle_count - baseline_cycle_count) / baseline_cycle_count)
                y.append((duration - baseline_duration) / baseline_duration)
        except FileNotFoundError:
            logging.warning(f"Data for {program}-{zkvm}-{measurement} not found")
    return x, y


def plot_cycle_count_duration(dir: str, measurement: str, p: str | None):
    profiles = get_profiles_ids()
    profiles.remove(BASELINE)
    programs = get_programs() if p is None else [p]
    plot_scatter_by_zkvm(
        get_title(
            "Relative cycle count vs duration compared to baseline", [measurement, p]
        ),
        lambda zkvm: _get_values(dir, zkvm, programs, profiles, measurement),
        "Relative cycle count compared to baseline",
        "Relative duration compared to baseline",
    )
