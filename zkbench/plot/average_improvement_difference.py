import logging

import numpy as np
from zkbench.config import get_default_profiles_ids, get_programs, get_zkvms
from zkbench.plot.common import (
    BASELINE,
    get_average_improvement_over_baseline,
    get_title,
    plot_sorted,
)


def plot_average_improvement_difference(
    dir: str,
    speedup: bool,
    zkvm: str | None = None,
    show_x86: bool = True,
):
    def f(dir, program, zkvm, profile, measurement):
        return get_average_improvement_over_baseline(
            dir, zkvm, program, profile, measurement, speedup=speedup
        )

    title = get_title(
        f"Average difference in improvement by profile compared to baseline",
        [],
    )

    profiles = get_default_profiles_ids()
    profiles.remove(BASELINE)

    prove_values = []
    exec_values = []
    exec_x86_values = []
    for profile in profiles:
        current_results_prove = []
        current_results_exec = []
        current_results_exec_x86 = []
        for program in get_programs():
            for current_zkvm in get_zkvms() if zkvm is None else [zkvm]:
                try:
                    p_prove = f(dir, program, current_zkvm, profile, "prove")
                    p_exec = f(dir, program, current_zkvm, profile, "exec")
                    if show_x86:
                        p_exec_x86 = f(dir, program, "x86", profile, "exec")
                        current_results_exec_x86.append(p_exec_x86)
                    current_results_prove.append(p_prove)
                    current_results_exec.append(p_exec)
                except FileNotFoundError:
                    logging.warning(
                        f"Data for {program}-{current_zkvm}-{profile} not found"
                    )

        prove_values.append(np.mean(current_results_prove))
        exec_values.append(np.mean(current_results_exec))
        if show_x86:
            exec_x86_values.append(np.mean(current_results_exec_x86))

    y_axis = "difference in speedup" if speedup else "difference in % faster"
    plot_sorted(
        [prove_values, exec_values, exec_x86_values] if show_x86 else [prove_values, exec_values],
        profiles,
        title,
        y_axis,
        ["prove", "exec", "x86 exec"] if show_x86 else ["prove", "exec"],
    )
