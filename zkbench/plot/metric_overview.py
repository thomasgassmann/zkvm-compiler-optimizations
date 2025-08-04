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


def plot_metric_overview(
    dir: str,
    top_n: int | None,
    zkvms: list[str] | None,
    metrics: list[str] | None = None,
    speedup: bool = False,
):
    zkvms = get_zkvms() if len(zkvms) == 0 else zkvms
    metrics = ["prove", "exec", "cycle-count"] if len(metrics) == 0 else metrics
    fig, axes = plt.subplots(
        nrows=len(zkvms), ncols=len(metrics), figsize=(15, 10), sharey=True
    )
    fig.subplots_adjust(hspace=0, wspace=0)

    programs = get_programs()

    def get_values(metric: str, zkvm: str, profile: str):
        if metric == "prove":
            return [
                get_average_improvement_over_baseline(
                    dir, zkvm, program, profile, "prove", speedup=speedup
                )
                for program in programs
            ]
        elif metric == "exec":
            return [
                get_average_improvement_over_baseline(
                    dir, zkvm, program, profile, "exec", speedup=speedup
                )
                for program in programs
            ]
        elif metric == "cycle-count":
            return [
                get_cycle_count_improvement_over_baseline(
                    dir, program, zkvm, profile, speedup=speedup
                )
                for program in programs
            ]

    sort_metric = "prove"
    all_profiles = list(
        sorted(
            [p for p in get_default_profiles_ids() if p != BASELINE],
            key=lambda p: np.average(
                np.array([get_values(sort_metric, zkvm, p) for zkvm in zkvms]).reshape(
                    -1
                )
            ),
            reverse=True,
        )
    )
    data = [
        [
            [get_values(metric, zkvm, profile) for profile in all_profiles]
            for metric in metrics
        ]
        for zkvm in zkvms
    ]

    if top_n:
        # keep only the top_n profiles for each metric
        profiles_idx_per_metric = [[] for _ in metrics]
        profiles_per_metric = [[] for _ in metrics]
        for j, metric in enumerate(metrics):
            profile_averages = []
            for p, profile in enumerate(all_profiles):
                values = []
                for i in range(len(zkvms)):
                    values.extend(data[i][j][p])
                profile_averages.append((np.mean(np.abs(values)), p, profile))

            # Sort by average and take top n
            profile_averages.sort(reverse=True)
            top_profiles = profile_averages[:top_n]

            for profile in all_profiles:
                if any(p[2] == profile for p in top_profiles):
                    cur = next(p for p in top_profiles if p[2] == profile)
                    profiles_idx_per_metric[j].append(cur[1])
                    profiles_per_metric[j].append(cur[2])
    else:
        profiles_idx_per_metric = [list(range(len(all_profiles))) for _ in metrics]
        profiles_per_metric = [all_profiles for _ in metrics]

    for i, zkvm in enumerate(zkvms):
        for j, metric in enumerate(metrics):
            ax = (
                axes[i, j]
                if len(zkvms) > 1 and len(metrics) > 1
                else (
                    axes[j if len(metrics) > 1 else i]
                    if len(zkvms) > 1 or len(metrics) > 1
                    else axes
                )
            )

            profiles_data = data[i][j]
            means = [
                np.mean(profile_data)
                for k, profile_data in enumerate(profiles_data)
                if k in profiles_idx_per_metric[j]
            ]
            stds = [
                np.std(profile_data)
                for k, profile_data in enumerate(profiles_data)
                if k in profiles_idx_per_metric[j]
            ]

            x = np.arange(len(profiles_per_metric[j]))
            ax.fill_between(
                x,
                np.array(means) - np.array(stds),
                np.array(means) + np.array(stds),
                alpha=0.3,
                color="red",
            )
            ax.plot(x, means, "-o", color="red", markersize=6)
            ax.axhline(0, color="black", linewidth=0.5)
            ax.set_xticks(x)
            if i == len(zkvms) - 1:
                ax.set_xticklabels(profiles_per_metric[j], rotation=90, ha="center")
            else:
                ax.xaxis.set_tick_params(labelcolor="none")

            ax.set_title(
                f"{get_zkvm_display_name(zkvm)} ({get_metric_display_name(metric)})"
            )
            if j == 0:
                ax.set_ylabel("speedup" if speedup else "(%) change")
    show_or_save_plot()
