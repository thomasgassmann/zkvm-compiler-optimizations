from matplotlib import pyplot as plt
import numpy as np
from zkbench.config import (
    get_default_profiles_ids,
    get_metric_display_name,
    get_programs,
    get_zkvm_display_name,
    get_zkvms,
)
from zkbench.plot.common import (
    BASELINE,
    get_average_improvement_over_baseline,
    get_cycle_count_improvement_over_baseline,
    show_or_save_plot,
)

def plot_improvement_programs_overview(
    dir: str,
    severe: float,
    moderate: float,
    top_n: int | None,
    zkvms: list[str] | None,
    metrics: list[str] | None,
):
    zkvms = get_zkvms() if not zkvms else zkvms
    metrics = ["prove", "exec", "cycle-count"] if not metrics else metrics

    programs = get_programs()
    profiles = [p for p in get_default_profiles_ids(remove_ox=True) if p != BASELINE]

    # structure [zkvm][metric][program][category]
    counts = {
        zkvm: {m: {prog: np.zeros(4, dtype=int) for prog in programs} for m in metrics}
        for zkvm in zkvms
    }

    for zkvm in zkvms:
        for metric in metrics:
            for profile in profiles:
                for prog in programs:
                    if metric in ("prove", "exec"):
                        v = get_average_improvement_over_baseline(
                            dir, zkvm, prog, profile, metric, speedup=False
                        )
                    else:
                        v = -get_cycle_count_improvement_over_baseline(
                            dir, prog, zkvm, profile, speedup=False
                        )

                    if v >= severe:
                        counts[zkvm][metric][prog][0] += 1
                    elif v >= moderate:
                        counts[zkvm][metric][prog][1] += 1
                    elif v <= -severe:
                        counts[zkvm][metric][prog][3] += 1
                    elif v <= -moderate:
                        counts[zkvm][metric][prog][2] += 1

    programs_per_metric = {}
    for metric in metrics:
        # order by total severe gains across zkvm
        ordering = list(
            sorted(
                programs,
                key=lambda prog: sum(counts[zkvm][metric][prog][0] for zkvm in zkvms),
                reverse=True,
            )
        )
        if top_n:
            # score by sum severe + severe degradation counts
            scores = []
            for prog in programs:
                total = sum(
                    counts[zkvm][metric][prog][0] + counts[zkvm][metric][prog][3]
                    for zkvm in zkvms
                )
                scores.append((total, prog))
            top = sorted(scores, reverse=True)[:top_n]
            # preserve ordering index
            programs_per_metric[metric] = sorted(
                [p for _, p in top], key=ordering.index
            )
        else:
            programs_per_metric[metric] = ordering

    # plotting
    nrows, ncols = len(zkvms), len(metrics)
    fig, axes = plt.subplots(nrows=nrows, ncols=ncols, figsize=(15, 10), sharey=True)
    fig.subplots_adjust(hspace=0, wspace=0.3)

    cat_labels = [
        f"\u2265 {severe}%",
        f"{moderate}% to {severe}%",
        f"-{severe}% to -{moderate}%",
        f"\u2264 -{severe}%",
    ]

    for i, zkvm in enumerate(zkvms):
        for j, metric in enumerate(metrics):
            ax = (
                axes[i, j]
                if nrows > 1 and ncols > 1
                else (axes[j] if nrows == 1 else axes[i])
            )
            progs = programs_per_metric[metric]
            x = np.arange(len(progs))
            bottom = np.zeros_like(x)

            for idx in range(4):
                vals = [counts[zkvm][metric][prog][idx] for prog in progs]
                ax.bar(x, vals, bottom=bottom, label=cat_labels[idx])
                bottom += vals

            ax.set_xticks(x)
            if i == len(zkvms) - 1:
                ax.set_xticklabels(progs, rotation=90, ha="center")
            else:
                ax.xaxis.set_tick_params(labelcolor="none")

            ax.set_title(
                f"{get_zkvm_display_name(zkvm)} ({get_metric_display_name(metric)})"
            )
            if j == 0:
                ax.set_ylabel("number of optimizations")
            if i == 0 and j == ncols - 1:
                ax.legend(loc="upper right")

    show_or_save_plot()
