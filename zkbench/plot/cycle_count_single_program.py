import logging
from zkbench.config import get_zkvms
from zkbench.plot.common import (
    get_cycle_count,
    get_title,
    plot_sorted,
)


def plot_cycle_count_for_single_program(
    dir: str,
    program: str,
    profiles: list[str],
    baseline_profile: str,
    abs: bool = False,
):
    title = get_title(
        f"Cycle count for {program} ({', '.join(profiles)} compared to {baseline_profile})",
        [],
    )

    values = []
    series = get_zkvms()

    all_profiles = profiles if not abs else list(profiles) + [baseline_profile]

    for zkvm in series:
        current = []
        for _, profile in enumerate(all_profiles):
            baseline_cycle_count = get_cycle_count(dir, program, zkvm, baseline_profile)
            try:
                profile_cycle_count = get_cycle_count(dir, program, zkvm, profile)
                if abs:
                    current.append(profile_cycle_count)
                else:
                    current.append((profile_cycle_count - baseline_cycle_count) / baseline_cycle_count)
            except FileNotFoundError:
                logging.warning(
                    f"Cycle count for {program} with profile {profile} on zkvm {zkvm} not found."
                )
                continue
        values.append(current)

    y_axis = "cycle count" if abs else "relative change in cycle count"
    plot_sorted(values, all_profiles, title, y_axis, series)
