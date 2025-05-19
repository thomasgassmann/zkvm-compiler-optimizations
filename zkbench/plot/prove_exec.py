import logging
from zkbench.config import get_default_profiles_ids
from zkbench.plot.common import (
    get_point_estimate_mean_ms,
    get_program_selection,
    get_title,
    plot_scatter_by_zkvm,
)


def _get_values(dir: str, zkvm: str, programs: list[str]):
    profiles = get_default_profiles_ids()
    x, y = [], []
    for profile in profiles:
        for program in programs:
            try:
                exec = get_point_estimate_mean_ms(dir, program, zkvm, profile, "exec")
                prove = get_point_estimate_mean_ms(dir, program, zkvm, profile, "prove")
                x.append(exec)
                y.append(prove)
            except FileNotFoundError:
                logging.warning(f"Data for {program}-{zkvm}-{profile} not found")
    return x, y


def plot_prove_exec(dir: str, program: str | None, program_group: str | None):
    programs = get_program_selection(program, program_group)
    plot_scatter_by_zkvm(
        get_title("Prove vs. exec time", [program, program_group]),
        lambda zkvm: _get_values(dir, zkvm, programs),
        "Exec time (ms)",
        "Prove time (ms)",
    )
