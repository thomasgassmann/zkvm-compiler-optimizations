import logging
from matplotlib import pyplot as plt
import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns

from zkbench.tune.exhaustive import Exhaustive, ExhaustiveResult
from zkbench.tune.plot.common import read_exhaustive_stats


def plot_exhaustive_depth2(stats: str): 
    stats: Exhaustive = read_exhaustive_stats(stats)
    passes = stats.config.loop_passes + stats.config.function_passes + stats.config.module_passes
    matrix = []
    largest = 0
    smallest = float("inf")
    for pass_a in passes:
        row = []
        for pass_b in passes:
            res: ExhaustiveResult = list(filter(lambda x: x.passes[0] == pass_a and x.passes[1] == pass_b, stats.results))
            if len(res) != 1:
                logging.warning(f"Expected 1 result for {pass_a} and {pass_b}, got {len(res)}")
                row.append(float("inf"))
                continue
            
            res = res[0]
            if res.build_error:
                # TODO:
                row.append(np.nan)
            elif res.eval_result.has_error:
                # TODO:
                row.append(np.nan)
            elif any(map(lambda x: x.timeout, res.eval_result.values)):
                # TODO:
                row.append(np.nan)
            else:
                s = sum(map(lambda x: x.metric, res.eval_result.values))
                largest = max(largest, s)
                smallest = min(smallest, s)
                row.append(s)
        matrix.append(row)

    matrix = np.array(matrix)
    matrix = matrix / largest

    plt.figure(figsize=(10, 8))
    sns.heatmap(matrix, annot=True, fmt=".4f", cmap="viridis", xticklabels=passes, yticklabels=passes, vmin=smallest / largest, vmax=1)
    plt.title("Normalized cumulative cycle count")
    plt.xlabel("Pass B")
    plt.ylabel("Pass A")
    plt.tight_layout()
    plt.show()
