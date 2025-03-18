import logging
from zkbench.config import get_profiles_ids, get_programs
from zkbench.plot.common import get_point_estimate_ms, get_title, plot_scatter_by_zkvm


def _get_values(dir: str, zkvm: str, programs: list[str]):
    profiles = get_profiles_ids()
    x, y = [], []
    for profile in profiles:
        for program in programs:
            try:
                x.append(get_point_estimate_ms(
                    dir, program, zkvm, profile, "exec"
                ))
                y.append(get_point_estimate_ms(
                    dir, program, zkvm, profile, "prove"
                ))
            except FileNotFoundError:
                logging.warning(f"Data for {program}-{zkvm}-{profile} not found")
    return x, y


def plot_prove_exec(dir: str, program: str | None):
    programs = get_programs() if program is None else [program]
    plot_scatter_by_zkvm(
        get_title(
            "Prove vs. exec time", [program]
        ),
        lambda zkvm: _get_values(dir, zkvm, programs),
        "Exec time (ms)",
        "Prove time (ms)",
    )
