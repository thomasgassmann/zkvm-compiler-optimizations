from matplotlib import pyplot as plt

from zkbench.plot.common import get_program_selection, show_or_save_plot
from zkbench.tune.genetic import Genetic
from zkbench.tune.plot.common import get_metric_sum, read_genetic_stats


def plot_genetic(
    stats: str,
    program: str | None = None,
    zkvm: str | None = None,
    program_group: str | None = None,
):
    stats: Genetic = read_genetic_stats(stats)

    programs = get_program_selection(program, program_group)

    stats_values = [get_metric_sum(v, programs, zkvm) for v in stats.metrics]
    stats_values = [v for v in stats_values if v > 0]

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
