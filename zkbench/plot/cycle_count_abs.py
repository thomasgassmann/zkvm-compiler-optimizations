import numpy as np
from zkbench.config import get_measurements, get_profiles_ids, get_programs, get_zkvms
from zkbench.plot.common import BASELINE, get_cycle_count, get_title, get_values_by_profile, plot_grouped_boxplot, plot_sorted

def plot_cycle_count_abs(dir: str, program: str, zkvm: str):
    title = get_title("Cycle count by profile", [zkvm, program])

    profiles = get_profiles_ids()
    cycle_counts = get_values_by_profile(
        dir, zkvm, 'prove', program, profiles, lambda dir, program, zkvm, profile, _: get_cycle_count(dir, program, zkvm, profile)
    )

    cycle_counts = np.squeeze(cycle_counts, axis=1)
    plot_sorted(
        [cycle_counts], profiles, title, "Cycle count by profile", [None]
    )
