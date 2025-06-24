import logging
from matplotlib import pyplot as plt
import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns

from zkbench.config import get_programs_by_group
from zkbench.plot.common import get_title, show_or_save_plot
from zkbench.tune.exhaustive import Exhaustive, ExhaustiveResult
from zkbench.tune.plot.common import read_exhaustive_stats


def plot_exhaustive_depth2(
    stats: str,
    program: str | None,
    zkvm: str | None,
    program_group: str | None = None,
    relative: bool = False,
):
    stats: Exhaustive = read_exhaustive_stats(stats)
    if relative and stats.baseline is None:
        logging.error("Relative plotting requested, but no baseline found in stats.")
        return

    passes = (
        stats.config.loop_passes
        + stats.config.function_passes
        + stats.config.module_passes
    )
    matrix = []
    largest = 0
    smallest = float("inf")
    for pass_a in passes:
        row = []
        for pass_b in passes:
            res: ExhaustiveResult = list(
                filter(
                    lambda x: x.passes[0] == pass_a and x.passes[1] == pass_b,
                    stats.results,
                )
            )
            if len(res) != 1:
                logging.warning(
                    f"Expected 1 result for {pass_a} and {pass_b}, got {len(res)}"
                )
                row.append(-1)
                continue

            res = res[0]

            if res.build_error:
                row.append(np.nan)
            elif res.eval_result.has_error:
                row.append(np.nan)
            else:
                relevant = list(
                    filter(
                        lambda x: (
                            x.program == program
                            or (
                                program_group is not None
                                and x.program in get_programs_by_group(program_group)
                            )
                        )
                        or (program is None and program_group is None)
                        and (x.zkvm == zkvm or zkvm is None),
                        res.eval_result.values,
                    )
                )
                if any(map(lambda x: x.timeout, relevant)):
                    row.append(np.nan)
                else:
                    if relative:
                        all_values = []
                        for s in relevant:
                            baseline_result = (
                                stats.baseline.eval_result.get_eval_result(
                                    s.zkvm, s.program
                                )
                            )
                            if baseline_result is None:
                                raise ValueError(
                                    f"No baseline eval result for program {s.program} and zkvm {s.zkvm}."
                                )
                            relative_value = (
                                s.metric - baseline_result.metric
                            ) / baseline_result.metric * 100
                            all_values.append(relative_value)
                        mean_value = float(np.mean(all_values))
                        largest = max(largest, mean_value)
                        smallest = min(smallest, mean_value)
                        row.append(mean_value)
                    else:
                        s = sum(map(lambda x: x.metric, relevant))
                        largest = max(largest, s)
                        smallest = min(smallest, s)
                        row.append(s)
        matrix.append(row)

    matrix = np.array(matrix)
    if not relative:
        matrix = matrix / largest

    plt.figure(figsize=(12, 10))

    cmap = sns.color_palette("coolwarm", as_cmap=True)

    mask = np.isnan(matrix)

    sns.heatmap(
        matrix,
        annot=True if len(passes) <= 20 else False,
        fmt=".3f",
        xticklabels=passes,
        yticklabels=passes,
        vmin=smallest / largest if not relative else largest,
        vmax=1 if not relative else smallest,
        mask=mask,
        cmap=cmap,
        cbar_kws={
            "label": (
                f"Normalized cumulative {stats.metric}"
                if not relative
                else f"Relative change (% {stats.metric})"
            )
        },
    )

    for i in range(len(passes)):
        for j in range(len(passes)):
            if mask[i, j]:
                plt.gca().add_patch(plt.Rectangle((j, i), 1, 1, color="white", ec=None))

    plt.xticks(rotation=90, ha="center", fontsize=7)
    plt.yticks(rotation=0, fontsize=7)

    title = get_title(
        f"Normalized cumulative {stats.metric}{' (relative)' if relative else ''}", [program, zkvm, program_group]
    )
    plt.title(title)
    plt.xlabel("Pass B")
    plt.ylabel("Pass A")
    plt.tight_layout()
    show_or_save_plot()
