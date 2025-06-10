import numpy as np
from zkbench.config import get_default_profiles_ids
from zkbench.plot.common import (
    BASELINE,
    get_average_improvement_over_baseline,
    get_title,
    get_values_by_profile,
    plot_grouped_boxplot,
    plot_sorted,
)


def plot_average_improvement(
    dir: str,
    zkvm: str | None,
    program: str | None,
    program_group: str | None,
    speedup: bool,
    global_average: bool,
    show_x86: bool = False,
):
    def f(dir, program, zkvm, profile, measurement):
        return get_average_improvement_over_baseline(
            dir, zkvm, program, profile, measurement, speedup=speedup
        )

    title = get_title(
        "Average improvement by profile compared to baseline",
        [zkvm, program, program_group],
    )

    profiles = get_default_profiles_ids()
    profiles.remove(BASELINE)
    relative_improvements_prove = get_values_by_profile(
        dir, zkvm, "prove", program, program_group, profiles, f
    )
    relative_improvements_exec = get_values_by_profile(
        dir, zkvm, "exec", program, program_group, profiles, f
    )
    relative_improvements_exec_x86 = []
    if show_x86:
        relative_improvements_exec_x86 = get_values_by_profile(
            dir, "x86", "exec", program, program_group, profiles, f
        )
    if global_average:
        relative_improvements_exec = np.mean(relative_improvements_exec, axis=1)
        relative_improvements_prove = np.mean(relative_improvements_prove, axis=1)
        if show_x86:
            relative_improvements_exec_x86 = np.mean(
                relative_improvements_exec_x86, axis=1
            )

    y_axis = "speedup" if speedup else "% faster"
    if global_average or len(relative_improvements_exec[0]) == 1:
        # if we only have one value, no need to plot boxplot
        if not global_average:
            prove_values = np.squeeze(relative_improvements_prove, axis=1)
            exec_values = np.squeeze(relative_improvements_exec, axis=1)
            if show_x86:
                exec_values_x86 = np.squeeze(relative_improvements_exec_x86, axis=1)
        else:
            prove_values = relative_improvements_prove
            exec_values = relative_improvements_exec
            if show_x86:
                exec_values_x86 = relative_improvements_exec_x86
        plot_sorted(
            (
                [
                    prove_values,
                    exec_values,
                ]
                if not show_x86
                else [
                    prove_values,
                    exec_values,
                    exec_values_x86,
                ]
            ),
            profiles,
            title,
            y_axis,
            ["prove", "exec"] if not show_x86 else ["prove", "exec", "exec x86"],
        )
    else:
        plot_grouped_boxplot(
            (
                [relative_improvements_prove, relative_improvements_exec]
                if not show_x86
                else [
                    relative_improvements_prove,
                    relative_improvements_exec,
                    relative_improvements_exec_x86,
                ]
            ),
            profiles,
            title,
            y_axis,
            ["prove", "exec"] if not show_x86 else ["prove", "exec", "exec x86"],
        )
