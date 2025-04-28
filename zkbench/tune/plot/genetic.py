from matplotlib import pyplot as plt

from zkbench.tune.genetic import Genetic
from zkbench.tune.plot.common import read_genetic_stats


def plot_genetic(stats: str): 
    stats: Genetic = read_genetic_stats(stats)

    plt.plot(stats.values, marker="o", linestyle="-")

    colors = plt.cm.tab10.colors
    for i, baseline in enumerate(stats.baselines):
        value = stats.baselines[baseline]
        metric_sum = sum([v.metric for v in value])
        plt.axhline(y=metric_sum, label=baseline, color=colors[i % len(colors)])

    plt.axhline(y=stats.best_metric, label="Best", color="red", linestyle="--")

    best_iteration = stats.values.index(stats.best_metric)
    plt.annotate(
        f"Best value: {stats.best_metric}",
        xy=(best_iteration, stats.best_metric),
        xytext=(best_iteration, stats.best_metric - 0.05 * stats.best_metric),
        arrowprops=dict(facecolor="black", arrowstyle="->"),
        fontsize=10,
    )

    plt.legend()
    plt.xlabel('Iteration')
    plt.ylabel(f"Metric value ({stats.metric})")
    plt.suptitle(f"Metric over Iterations ({stats.mode_name})", y=0.95, fontsize=18)
    plt.title(f"Passes of best: {', '.join(stats.best_profile.passes)}", fontsize=10)

    plt.grid()
    plt.show()
