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

def plot_improvement_number_overview(
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

    # structure [zkvm][metric][profile][category]
    counts = {zkvm: {m: {p: np.zeros(4, dtype=int) for p in profiles} for m in metrics}
              for zkvm in zkvms}

    for zkvm in zkvms:
        for metric in metrics:
            for profile in profiles:
                if metric in ("prove", "exec"):
                    vals = [
                        get_average_improvement_over_baseline(
                            dir, zkvm, prog, profile, metric, speedup=False
                        ) for prog in programs
                    ]
                else:
                    vals = [
                        -get_cycle_count_improvement_over_baseline(
                            dir, prog, zkvm, profile, speedup=False
                        ) for prog in programs
                    ]
                for v in vals:
                    if v >= severe:
                        counts[zkvm][metric][profile][0] += 1
                    elif v >= moderate:
                        counts[zkvm][metric][profile][1] += 1
                    elif v <= -severe:
                        counts[zkvm][metric][profile][3] += 1
                    elif v <= -moderate:
                        counts[zkvm][metric][profile][2] += 1

    profiles_per_metric = {}
    for metric in metrics:
        ordering = list(sorted(profiles, key=lambda p: sum(counts[zkvm][metric][p][0] for zkvm in zkvms), reverse=True))
        if top_n:
            scores = []  # (score, profile)
            for profile in profiles:
                total = sum(
                    # select based on severe counts
                    counts[zkvm][metric][profile][3] + counts[zkvm][metric][profile][0]
                    for zkvm in zkvms
                )
                scores.append((total, profile))
            top = sorted(scores, reverse=True)[:top_n]
            profiles_per_metric[metric] = sorted([p for _, p in top], key=ordering.index)
        else:
            profiles_per_metric[metric] = sorted(profiles, key=ordering.index)

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
            ax = axes[i, j] if nrows > 1 and ncols > 1 else (axes[j] if nrows == 1 else axes[i])
            profs = profiles_per_metric[metric]
            x = np.arange(len(profs))
            bottom = np.zeros_like(x)

            for idx in range(4):
                vals = [counts[zkvm][metric][p][idx] for p in profs]
                ax.bar(x, vals, bottom=bottom, label=cat_labels[idx])
                bottom += vals

            ax.set_xticks(x)
            if i == len(zkvms) - 1:
                ax.set_xticklabels(profs, rotation=90, ha="center")
            else:
                ax.xaxis.set_tick_params(labelcolor="none")
            ax.set_title(f"{get_zkvm_display_name(zkvm)} ({get_metric_display_name(metric)})")
            if j == 0:
                ax.set_ylabel("number of programs")
            if i == 0 and j == ncols - 1:
                ax.legend(loc="upper right")

    show_or_save_plot()
