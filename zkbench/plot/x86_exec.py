import logging
from zkbench.config import get_default_profiles_ids, get_programs
from zkbench.plot.common import get_point_estimate_median_ms, get_title, plot_scatter_by_zkvm


def _get_values(dir: str, zkvm: str, measurement: str):
    profiles = get_default_profiles_ids()
    x, y = [], []
    for profile in profiles:
        for program in get_programs():
            try:
                x86 = get_point_estimate_median_ms(dir, program, "x86", profile, "exec")
                z = get_point_estimate_median_ms(
                    dir, program, zkvm, profile, measurement
                )
                x.append(x86)
                y.append(z)
            except FileNotFoundError:
                logging.warning(f"Data for {program}-{zkvm}-{profile} not found")
    return x, y

def plot_x86_exec(dir: str, measurement: str):
    plot_scatter_by_zkvm(
        get_title(f"Exec (x86) vs. {measurement} (zkVM)", []),
        lambda zkvm: _get_values(dir, zkvm, measurement),
        "Exec time (x86, ms)",
        f"{measurement} time (zkVM, ms)",
    )
