import numpy as np
from zkbench.config import get_measurements, get_zkvms
from zkbench.plot.common import get_point_estimate_median_ms, get_program_selection, get_sample_times_ms, get_title, get_values_by_profile, plot_sorted


def plot_duration(dir: str, programs: list[str], program_group: str, profile: str):
    title = get_title(
        "Duration", [", ".join(programs) if any(programs) else None, program_group, profile]
    )

    programs = get_program_selection(programs, program_group)

    labels = []
    series = []
    for zkvm in get_zkvms():
        for measurement in get_measurements():
            values = get_values_by_profile(
                dir,
                zkvm,
                measurement,
                programs,
                None,
                [profile],
                lambda dir, program, zkvm, profile, measurement: get_point_estimate_median_ms(
                    dir, program, zkvm, profile, measurement
                ),
            )
            series.append(values)
            labels.append(f"{zkvm} {measurement}")
    series.append(
        get_values_by_profile(
            dir,
            "x86",
            "exec",
            programs,
            None,
            [profile],
            lambda dir, program, zkvm, profile, measurement: get_point_estimate_median_ms(
                dir, program, zkvm, profile, measurement
            ),
        )
    )
    labels.append("x86 exec")

    series = np.array(series).squeeze(axis=1).tolist()
    plot_sorted(series, list(programs), title, "Duration (ms)", list(labels), log_scale=True)
