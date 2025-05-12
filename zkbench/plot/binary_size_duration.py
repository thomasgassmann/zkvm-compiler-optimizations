import logging
from zkbench.config import get_default_profiles_ids, get_programs
from zkbench.plot.common import (
    get_point_estimate_mean_ms,
    get_title,
    plot_scatter_by_zkvm,
    read_program_meta,
)


def _get_values(dir: str, zkvm: str, programs: list[str], measurement: str):
    x, y = [], []
    for program in programs:
        for profile in get_default_profiles_ids():
            try:
                meta = read_program_meta(dir, program, zkvm, profile)
                if meta is None:
                    continue
                current_x = meta["size"]
                current_y = get_point_estimate_mean_ms(dir, program, zkvm, profile, measurement)
                x.append(current_x)
                y.append(current_y)
            except FileNotFoundError as e:
                logging.warning(f"Data for {program}-{zkvm}-{measurement}-{profile} not found {e}")
            
    return x, y


def plot_binsize_duration(
    dir: str, program: str | None, measurement: str
):
    programs = get_programs() if program is None else [program]
    plot_scatter_by_zkvm(
        get_title("Binary size vs. duration", [program, measurement]),
        lambda zkvm: _get_values(dir, zkvm, programs, measurement),
        "Binary size (bytes)",
        f"Duration (ms, {measurement})",
    )