import logging

from numpy import mean, prod

from zkbench.config import get_programs, get_zkvms
from zkbench.plot.common import (
    get_average_improvement_over_baseline,
    get_sample_times_ms,
    get_title,
    plot_grouped_boxplot,
    read_estimates_data,
)


def plot_improvement_for_profile(
    dir: str,
    profile: str,
    baseline_profile: str,
    speedup: bool,
    zkvm: str | None = None,
    measurement: str | None = None,
):

    def f(dir, program, zkvm, measurement):
        return get_average_improvement_over_baseline(
            dir,
            zkvm,
            program,
            profile,
            measurement,
            speedup=speedup,
            baseline=baseline_profile,
        )

    title = get_title(
        f"Improvement compared to {baseline_profile}",
        [zkvm],
    )

    programs = []
    zkvms = get_zkvms() if not zkvm else [zkvm]
    measurements = [measurement] if measurement else ["prove", "exec"]
    values = []
    for measurement in measurements:
        values.append([])

    for program in get_programs():
        err = False
        current_measurements = [[] for _ in measurements]
        for zkvm in zkvms:
            try:
                for i, measurement in enumerate(measurements):
                    current_measurements[i].append(f(dir, program, zkvm, measurement))
            except FileNotFoundError:
                logging.warning(
                    f"File not found for {program} {zkvm} {profile} {baseline_profile}. Skipping."
                )
                err = True
                break
        if err:
            continue
        programs.append(program)
        for i, _ in enumerate(measurements):
            values[i].append(current_measurements[i])

    logging.info(
        f"Average: {mean(values, axis=1)}"
    )

    y_axis = "speedup" if speedup else "speedup (%)"
    plot_grouped_boxplot(
        values,
        programs,
        title,
        y_axis,
        measurements,
        bar_width=0.35,
    )
