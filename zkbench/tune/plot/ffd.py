import logging

from matplotlib import pyplot as plt
import numpy as np
from zkbench.config import get_default_profiles_ids, get_programs
from zkbench.plot.common import (
    BASELINE,
    get_cycle_count,
    get_program_selection,
    get_title,
    plot_sorted,
    show_or_save_plot,
)
from zkbench.tune.ffd import FFDResult, FFDRun, read_ffd_stats
from zkbench.tune.plot.common import get_metric_sum
import seaborn as sns

from zkbench.tune.plot.exhaustive import get_pass_label

RESULTS_DIR = "./results/bench"


BASELINE_CACHE = {}


def get_response(
    response_type: str,
    run: FFDRun,
    res: FFDResult,
    programs: list[str],
    zkvm: str | None,
) -> float:
    if response_type == "cumulative":
        return float(get_metric_sum(res.eval_result.values, programs, zkvm))
    elif response_type == "relative-avg":
        improvements = []

        zkvms = [zkvm] if zkvm else run.zkvms
        for program in programs:
            for cz in zkvms:
                eval_result = res.eval_result.get_eval_result(cz, program)
                if not eval_result:
                    raise ValueError(
                        f"No eval result for program {program} and zkvm {cz} in row {res.row_index}."
                    )

                if (cz, program) in BASELINE_CACHE:
                    baseline_cycle_count = BASELINE_CACHE[(cz, program)]
                else:
                    baseline_cycle_count = get_cycle_count(
                        RESULTS_DIR, program, cz, BASELINE
                    )
                    BASELINE_CACHE[(cz, program)] = baseline_cycle_count
                relative_improvement = (
                    (baseline_cycle_count - eval_result.metric)
                    / baseline_cycle_count
                    * 100
                )
                improvements.append(relative_improvement)

        return float(np.mean(improvements))


def compute_factor_effect(
    ffd: FFDRun,
    factors: list[str],
    programs: list[str],
    zkvm: str | None,
    response_type: str,
) -> float:
    contrasts: list[int] = []
    responses: list[float] = []

    for res in ffd.results:
        if (
            res.eval_result is None
            or len(res.active_factors) == 0
            or res.eval_result.has_error
        ):
            logging.warning(f"Row {res.row_index} has no eval result or active factors")
            continue

        if res.build_error or res.eval_result is None:
            logging.warning(f"Row {res.row_index} has build errors")

        # coded level: +1 if on, -1 if off
        active_factors = set(res.active_factors)
        signs = [1 if f in active_factors else -1 for f in factors]
        contrast = int(np.prod(signs))

        y = get_response(response_type, ffd, res, programs, zkvm)

        contrasts.append(contrast)
        responses.append(float(y))

    n = len(responses)
    if n == 0:
        raise ValueError(
            f"No successful runs to compute effect for factor {factors!r}."
        )

    # yates
    effect = np.dot(contrasts, responses) / (n / 2)
    return effect


def _get_programs(ffd: FFDRun, program: str | None) -> list[str]:
    ignore = set(get_programs()) - set(ffd.programs)
    programs = get_program_selection(program, None, ignore=ignore)
    return programs


def _get_y_axis_label(response: str) -> str:
    if response == "cumulative":
        return "Change in cycle count"
    elif response == "relative-avg":
        return "Estimated change in cycle count (%)"
    else:
        raise ValueError(f"Unknown response type: {response}")


def plot_ffd1d(
    stats_file: str,
    program: str | None = None,
    zkvm: str | None = None,
    response: str = "cumulative",
    drop_below: float = 0.0,
):
    stats = read_ffd_stats(stats_file)

    effects = [
        compute_factor_effect(
            stats, [factor], _get_programs(stats, program), zkvm, response_type=response
        )
        for factor in stats.factors
    ]

    factor_list = []
    effect_list = []
    for factor, effect in zip(stats.factors, effects):
        factor_label = get_pass_label(factor)
        if factor_label in get_default_profiles_ids():
            factor_list.append(factor_label)
            effect_list.append(-effect)
        else:
            logging.warning(f"Factor {factor} is not a valid pass. Skipping.")

    y_axis = _get_y_axis_label(response)
    plot_sorted(
        [effect_list],
        factor_list,
        get_title(
            f"Estimated change in cumulative cycle count by optimization pass", [program, zkvm]
        ),
        y_axis,
        [None],
        drop_below=drop_below,
    )


def plot_ffd2d(
    stats_file: str,
    program: str | None = None,
    zkvm: str | None = None,
    response: str = "cumulative",
):
    stats = read_ffd_stats(stats_file)
    programs = _get_programs(stats, program)

    matrix = np.zeros((len(stats.factors), len(stats.factors)))
    for idx_a, factor_a in enumerate(stats.factors):
        for idx_b, factor_b in enumerate(stats.factors):
            if idx_a > idx_b:
                continue

            logging.info(
                f"Computing effect for factors {factor_a} and {factor_b} ({idx_a}, {idx_b})"
            )
            if factor_a == factor_b:
                matrix[idx_a, idx_b] = compute_factor_effect(
                    stats, [factor_a], programs, zkvm, response_type=response
                )
            else:
                effect_a = compute_factor_effect(
                    stats, [factor_a, factor_b], programs, zkvm, response_type=response
                )
                matrix[idx_a, idx_b] = effect_a
                matrix[idx_b, idx_a] = effect_a

    plt.figure(figsize=(12, 10))

    sns.heatmap(
        matrix,
        annot=True if len(stats.factors) <= 20 else False,
        fmt=".3f",
        xticklabels=stats.factors,
        yticklabels=stats.factors,
    )
    plt.xticks(rotation=90, ha="center", fontsize=7)
    plt.yticks(rotation=0, fontsize=7)

    title = get_title(f"FFD estimated effects for {stats.metric}", [program, zkvm])
    plt.title(title)
    plt.xlabel("Factor B")
    plt.ylabel("Factor A")
    plt.tight_layout()
    show_or_save_plot()
