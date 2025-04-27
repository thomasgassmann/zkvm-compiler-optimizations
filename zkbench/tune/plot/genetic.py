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

    plt.legend()
    plt.xlabel('Iteration')
    plt.ylabel(f"Metric value ({stats.metric})")
    plt.title(f"Metric over Iterations ({stats.mode_name})")

    plt.grid()
    plt.show()
