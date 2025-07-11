import os
import click

from zkbench.config import get_profiles_ids, get_program_groups, get_programs, get_zkvms
from zkbench.tune.plot.exhaustive import plot_exhaustive_depth2
from zkbench.tune.plot.export import (
    export_exhaustive_depth2,
    export_genetic,
    export_genetic_individual,
)
from zkbench.tune.plot.ffd import plot_ffd1d, plot_ffd2d
from zkbench.tune.plot.genetic import plot_genetic
from zkbench.tune.plot.genetic_individual import (
    extract_common_passes,
    plot_genetic_individual,
)


@click.command(name="genetic")
@click.option("--stats", required=True)
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
def plot_genetic_cli(stats: str, program: str | None, zkvm: str | None):
    if not os.path.exists(stats):
        raise click.ClickException(f"File {stats} does not exist.")
    plot_genetic(stats, program, zkvm)


@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
@click.option(
    "--baseline-profile", type=click.Choice(get_profiles_ids()), required=False
)
@click.command(name="genetic-individual")
@click.option("--stats-dir", required=True)
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option(
    "--program-group", type=click.Choice(get_program_groups()), required=False
)
@click.option("--average-programs", is_flag=True, default=False)
def plot_genetic_individual_cli(
    stats_dir: str,
    baseline_profile: str | None,
    program: str | None,
    zkvm: str | None,
    program_group: str | None = None,
    average_programs: bool = False,
):
    if not os.path.exists(stats_dir):
        raise click.ClickException(f"{stats_dir} does not exist.")
    plot_genetic_individual(
        stats_dir, baseline_profile, average_programs, program, zkvm, program_group
    )


@click.command(name="extract-genetic-individual")
@click.option("--stats-dir", required=True)
@click.option("--worst", is_flag=True, default=False, help="Extract worst passes")
def extract_genetic_individual_cli(stats_dir: str, worst: bool = False):
    if not os.path.exists(stats_dir):
        raise click.ClickException(f"{stats_dir} does not exist.")
    extract_common_passes(stats_dir, not worst)


@click.command(name="exhaustive-depth2")
@click.option("--stats", required=True)
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
@click.option("--relative", is_flag=True, default=False, help="Plot relative values")
def plot_exhaustive_depth2_cli(stats: str, program: str | None, zkvm: str | None, relative: bool = False):
    if not os.path.exists(stats):
        raise click.ClickException(f"File {stats} does not exist.")
    plot_exhaustive_depth2(stats, program, zkvm, program_group=None, relative=relative)


@click.command(name="ffd-1d")
@click.option("--stats", required=True)
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
@click.option("--response", type=click.Choice(["cumulative", "relative-avg"]), default="cumulative")
@click.option("--drop-below", type=float, default=0.0, help="Drop effects below this value")
def plot_ffd1d_cli(stats: str, program: str | None, zkvm: str | None, response: str = "cumulative", drop_below: float = 0.0):
    if not os.path.exists(stats):
        raise click.ClickException(f"File {stats} does not exist.")
    plot_ffd1d(stats, program, zkvm, response, drop_below=drop_below)


@click.command(name="ffd-2d")
@click.option("--stats", required=True)
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
@click.option("--response", type=click.Choice(["cumulative", "relative-avg"]), default="cumulative")
def plot_ffd2d_cli(stats: str, program: str | None, zkvm: str | None, response: str = "cumulative"):
    if not os.path.exists(stats):
        raise click.ClickException(f"File {stats} does not exist.")
    plot_ffd2d(stats, program, zkvm, response)


@click.command(name="export-exhaustive-depth2")
@click.option("--stats", required=True)
@click.option("--out", nargs=1, required=True, help="Output directory")
def export_exhaustive_depth2_cli(stats: str, out: str):
    if not os.path.exists(stats):
        raise click.ClickException(f"File {stats} does not exist.")
    export_exhaustive_depth2(stats, out)


@click.command(name="export-genetic")
@click.option("--stats", required=True)
@click.option("--out", nargs=1, required=True, help="Output directory")
def export_genetic_cli(stats: str, out: str):
    if not os.path.exists(stats):
        raise click.ClickException(f"File {stats} does not exist.")
    export_genetic(stats, out)


@click.command(name="export-genetic-individual")
@click.option("--stats-dir", required=True)
@click.option("--out", nargs=1, required=True, help="Output directory")
@click.option(
    "--baseline-profile", type=click.Choice(get_profiles_ids()), required=True
)
def export_genetic_individual_cli(stats_dir: str, out: str, baseline_profile: str):
    if not os.path.exists(stats_dir):
        raise click.ClickException(f"Directory {stats_dir} does not exist.")
    export_genetic_individual(stats_dir, out, baseline_profile)
