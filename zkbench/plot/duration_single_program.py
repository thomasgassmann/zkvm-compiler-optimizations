from zkbench.config import get_measurements, get_zkvms_with_x86
from zkbench.plot.common import (
    get_average_improvement_over_baseline,
    get_sample_times_ms,
    get_title,
    plot_grouped_boxplot,
    plot_sorted,
)


def plot_duration_for_single_program(
    dir: str,
    program: str,
    profiles: list[str],
    show_x86: bool = True,
):

    title = get_title(
        f"Duration for {program} ({', '.join(profiles)})",
        [],
    )

    improvements = [[] for _ in profiles]
    labels = []
    for zkvm in get_zkvms_with_x86():
        if not show_x86 and zkvm == "x86":
            continue

        for measurement in get_measurements():
            if measurement == "prove" and zkvm == "x86":
                continue

            for i, profile in enumerate(profiles):
                try:
                    prof = get_sample_times_ms(dir, program, zkvm, profile, measurement)
                    improvements[i].append(prof)
                except FileNotFoundError:
                    continue
            else:
                labels.append(f"{zkvm} ({measurement})")

    y_axis = "Duration (ms)"
    plot_grouped_boxplot(improvements, labels, title, y_axis, profiles)
