import os
from matplotlib import pyplot as plt
import numpy as np

from zkbench.config import get_programs, get_programs_by_group, get_zkvms
from zkbench.plot.common import show_or_save_plot
from zkbench.tune.common import MetricValue
from zkbench.tune.genetic import Genetic
from zkbench.tune.plot.common import read_genetic_stats
from zkbench.tune.plot.genetic import get_metric_sum


def plot_genetic_individual(
    stats_dir: str,
    baseline_profile: str,
    average_programs: bool = False,
    program: str | None = None,
    zkvm: str | None = None,
    program_group: str | None = None,
):
    programs = [] if program is None else [program]
    if program_group is not None:
        programs.extend(get_programs_by_group(program_group))
    if program is None and program_group is None:
        programs = get_programs()

    zkvms = get_zkvms() if zkvm is None else [zkvm]
    for zkvm in zkvms:
        program_values = []
        for program in programs:
            stats = read_genetic_stats(
                os.path.join(stats_dir, f"{program}-{zkvm}-stats.json")
            )
            stats_values = [get_metric_sum(v, [program], zkvm) for v in stats.metrics]
            baseline = get_metric_sum(stats.baselines[baseline_profile], [program], zkvm)
            relative_values = [v / baseline for v in stats_values if v > 0]
            program_values.append(relative_values)

        if average_programs:
            least_number_of_iterations = min([len(values) for values in program_values])
            program_values = [
                values[:least_number_of_iterations] for values in program_values
            ]
            averages = np.mean(np.array(program_values), axis=0)
            plt.plot(averages, label=f"{zkvm} (avg)", marker="o")
        else:
            for i, program in enumerate(programs):
                plt.plot(
                    program_values[i],
                    label=f"{program} ({zkvm})",
                    marker="o",
                )

    plt.legend()
    plt.xlabel("Iteration")
    plt.ylabel(f"Relative metric value")
    plt.suptitle(f"Metric over Iterations", y=0.95, fontsize=18)
    plt.grid()
    show_or_save_plot()
