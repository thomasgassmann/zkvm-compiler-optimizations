import logging

from matplotlib import pyplot as plt
import numpy as np
from zkbench.plot.common import get_program_selection, get_title, plot_sorted, show_or_save_plot
from zkbench.tune.ffd import FFDRun
from zkbench.tune.plot.common import get_metric_sum, read_ffd_stats
import seaborn as sns

def compute_factor_effect(ffd: FFDRun, factors: list[str], programs: list[str], zkvm: str | None) -> float:
    contrasts: list[int] = []
    responses: list[float] = []

    for res in ffd.results:
        if res.eval_result is None or len(res.active_factors) == 0 or res.eval_result.has_error:
            logging.warning(
                f"Row {res.row_index} has no eval result or active factors"
            )
            continue

        if res.build_error or res.eval_result is None:
            logging.warning(
                f"Row {res.row_index} has build errors"
            )
        
        # coded level: +1 if on, –1 if off
        active_factors = set(res.active_factors)
        signs = [1 if f in active_factors else -1 for f in factors]
        contrast = int(np.prod(signs))
        
        y = get_metric_sum(res.eval_result.values, programs, zkvm)
        
        contrasts.append(contrast)
        responses.append(float(y))
    
    n = len(responses)
    if n == 0:
        raise ValueError(f"No successful runs to compute effect for factor {factors!r}.")
    
    # yates
    effect = np.dot(contrasts, responses) / (n/2)
    return effect



def plot_ffd1d(stats_file: str, program: str | None = None, zkvm: str | None = None):
    programs = get_program_selection(program, None)
    stats = read_ffd_stats(stats_file)

    effects = [
        compute_factor_effect(stats, [factor], programs, zkvm)
        for factor in stats.factors
    ]
    plot_sorted([effects], stats.factors, get_title(f"FFD estimated effects ({stats.metric})", [program, zkvm]), "Change in cycle count", [None])


def plot_ffd2d(stats_file: str, program: str | None = None, zkvm: str | None = None):
    programs = get_program_selection(program, None)
    stats = read_ffd_stats(stats_file)

    matrix = np.zeros((len(stats.factors), len(stats.factors)))
    for idx_a, factor_a in enumerate(stats.factors):
        for idx_b, factor_b in enumerate(stats.factors):
            if factor_a == factor_b:
                matrix[idx_a, idx_b] = compute_factor_effect(stats, [factor_a], programs, zkvm)
            else:
                effect_a = compute_factor_effect(stats, [factor_a, factor_b], programs, zkvm)
                effect_b = compute_factor_effect(stats, [factor_b, factor_a], programs, zkvm)
                matrix[idx_a, idx_b] = (effect_a + effect_b) / 2

    sns.heatmap(
        matrix,
        annot=True if len(stats.factors) <= 20 else False,
        fmt=".3f",
        xticklabels=stats.factors,
        yticklabels=stats.factors,
    )
    plt.xticks(rotation=90, ha="center", fontsize=7)
    plt.yticks(rotation=0, fontsize=7)

    title = get_title(
        f"FFD estimated effects for {stats.metric}", [program, zkvm]
    )
    plt.title(title)
    plt.xlabel("Factor B")
    plt.ylabel("Factor A")
    plt.tight_layout()
    show_or_save_plot()
