import os
import re
from typing import Counter
from matplotlib import pyplot as plt
import numpy as np

from zkbench.config import Profile, get_programs, get_programs_by_group, get_zkvms
from zkbench.plot.common import show_or_save_plot
from zkbench.tune.common import ProfileConfig
from zkbench.tune.genetic import Genetic
from zkbench.tune.plot.common import read_genetic_stats
from zkbench.tune.plot.genetic import get_metric_sum


def extract_common_passes(stats_dir: str, best: bool):
    stats = [
        read_genetic_stats(os.path.join(stats_dir, f"{program}-{zkvm}-stats.json"))
        for program in get_programs()
        for zkvm in get_zkvms()
    ]

    def get_k_best(
        k: int, stats: list[Genetic], best=True
    ) -> list[Profile | ProfileConfig]:
        best_profiles = []
        for stat in stats:
            profile_value_pairs = list(zip(stat.profile_configs, stat.values))

            profile_value_pairs.sort(key=lambda x: x[1], reverse=not best)

            k_selected_profiles = [p for p, v in profile_value_pairs[:k]]

            best_profiles.extend(k_selected_profiles)
        return best_profiles

    def parse_single_pass_string(passes: str) -> list[str]:
        _token_re = re.compile(r"[A-Za-z0-9_-]+" r"(?=\s*(?:,|\)|$))")

        _WRAPPERS = {"module", "function", "loop"}

        return [tok for tok in _token_re.findall(passes) if tok not in _WRAPPERS]

    def parse_passes(passes: list[str]) -> list[str]:
        res = []
        for p in passes:
            res.extend(parse_single_pass_string(p))
        return res

    best_profiles = [
        parse_passes(profile.passes) for profile in get_k_best(5, stats, best=best)
    ]

    def extract_top_of_len(
        k: int, n: int, best_profiles: list[list[str]]
    ) -> list[list[str]]:
        counter: Counter[tuple[str, ...]] = Counter()
        for profile in best_profiles:
            n = len(profile)
            if n < k:
                continue
            for i in range(n - k + 1):
                counter[tuple(profile[i : i + k])] += 1
        return [(list(seq), k) for seq, k in counter.most_common(n)]

    for k in range(1, 6):
        print(f"Top {k}-length sequences:")
        for s, c in extract_top_of_len(k, 10, best_profiles):
            print(f"Sequence: {s}, Count: {c}")


def plot_genetic_individual(
    stats_dir: str,
    baseline_profile: str | None,
    average_programs: bool = False,
    program: str | None = None,
    zkvm: str | None = None,
    program_group: str | None = None,
):
    programs = [] if program is None else [program]
    if program_group is not None:
        programs.extend(get_programs_by_group(program_group))
    if program is None and program_group is None:
        programs = get_programs()

    if len(programs) == 0:
        raise ValueError(
            "No programs selected. Please specify a program or program group."
        )

    zkvms = get_zkvms() if zkvm is None else [zkvm]
    for zkvm in zkvms:
        program_values = []
        for program in programs:
            stats = read_genetic_stats(
                os.path.join(stats_dir, f"{program}-{zkvm}-stats.json")
            )
            stats_values = [get_metric_sum(v, [program], zkvm) for v in stats.metrics]
            if baseline_profile is not None:
                baseline = get_metric_sum(
                    stats.baselines[baseline_profile], [program], zkvm
                )
            else:
                baseline = max(stats_values)
            relative_values = [v / baseline for v in stats_values if v > 0]
            program_values.append(relative_values)

        if average_programs:
            least_number_of_iterations = min([len(values) for values in program_values])
            program_values = [
                values[:least_number_of_iterations] for values in program_values
            ]
            averages = np.mean(np.array(program_values), axis=0)
            plt.plot(averages, label=f"{zkvm} (avg)", marker="o")
        else:
            for i, program in enumerate(programs):
                plt.plot(
                    program_values[i],
                    label=f"{program} ({zkvm})",
                    marker="o",
                )

    plt.legend()
    plt.xlabel("Iteration")
    plt.ylabel(f"Relative metric value")
    plt.suptitle(f"Metric over Iterations", y=0.95, fontsize=18)
    plt.grid()
    show_or_save_plot()
