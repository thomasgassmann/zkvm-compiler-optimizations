import logging
import click

from zkbench.config import get_default_profiles_ids, get_measurements
from zkbench.plot.common import (
    BASELINE,
    get_title,
    get_values_by_profile,
    plot_grouped_boxplot,
    read_program_meta,
)


def f(dir, program, zkvm, profile):
    baseline = read_program_meta(dir, program, zkvm, BASELINE)
    compared = read_program_meta(dir, program, zkvm, profile)
    if compared is None:
        return None

    if "paging_cycles" not in baseline or "paging_cycles" not in compared:
        logging.warning(f"Paging cycles not found in {program}-{zkvm}-{profile}")
        return None

    baseline = baseline["paging_cycles"]
    compared = compared["paging_cycles"]
    return (compared - baseline) / baseline


def plot_paging_by_profile(dir: str, zkvm: str, program: str):
    if zkvm == "sp1":
        raise click.UsageError("Currently only supported for risc0")

    title = get_title(
        "Relative change in paging cycles compared to baseline", [program, zkvm]
    )
    profiles = get_default_profiles_ids()
    profiles.remove(BASELINE)
    values = get_values_by_profile(
        dir,
        zkvm,
        [get_measurements()[0]],  # can be arbitrary
        program,
        None,
        profiles,
        lambda dir, program, zkvm, profile, _: f(dir, program, zkvm, profile),
    )

    plot_grouped_boxplot([values], profiles, title, "Relative change in paging cycles", [zkvm])
