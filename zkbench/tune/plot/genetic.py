from matplotlib import pyplot as plt

from zkbench.config import get_programs_by_group
from zkbench.plot.common import show_or_save_plot
from zkbench.tune.common import MetricValue
from zkbench.tune.genetic import Genetic
from zkbench.tune.plot.common import read_genetic_stats


def get_metric_sum(
    l: list[MetricValue], program_list: list[str] | None, zkvm: str | None
) -> float:
    return sum(
        [
            v.metric
            for v in l
            if (program_list is None or v.program in program_list)
            and (v.zkvm == zkvm or zkvm is None)
            and not v.timeout
        ]
    )


def plot_genetic(
    stats: str,
    program: str | None = None,
    zkvm: str | None = None,
    program_group: str | None = None,
):
    stats: Genetic = read_genetic_stats(stats)

    programs = [] if program is None else [program]
    if program_group is not None:
        programs.extend(get_programs_by_group(program_group))
    if program is None and program_group is None:
        programs = None

    stats_values = [get_metric_sum(v, programs, zkvm) for v in stats.metrics]

    plt.plot(stats_values, marker="o", linestyle="-")

    colors = plt.cm.tab10.colors
    for i, baseline in enumerate(stats.baselines):
        value = stats.baselines[baseline]
        metric_sum = get_metric_sum(value, programs, zkvm)
        plt.axhline(y=metric_sum, label=baseline, color=colors[i % len(colors)])

    best_metric = min(stats_values)
    plt.axhline(y=best_metric, label="Best", color="red", linestyle="--")

    best_iteration = stats_values.index(best_metric)
    plt.annotate(
        f"Best value: {best_metric}",
        xy=(best_iteration, best_metric),
        xytext=(best_iteration, best_metric - 0.05 * best_metric),
        arrowprops=dict(facecolor="black", arrowstyle="->"),
        fontsize=10,
    )

    plt.legend()
    plt.xlabel('Iteration')
    plt.ylabel(f"Metric value ({stats.metric})")
    plt.suptitle(f"Metric over Iterations ({stats.mode_name})", y=0.95, fontsize=18)
    plt.title(
        f"Passes of global best: {', '.join(stats.best_profile.passes)}", fontsize=10
    )

    plt.grid()
    show_or_save_plot()
