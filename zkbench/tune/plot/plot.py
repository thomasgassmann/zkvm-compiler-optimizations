import os
import click

from zkbench.config import get_programs, get_zkvms
from zkbench.tune.plot.exhaustive import plot_exhaustive_depth2
from zkbench.tune.plot.export import export_exhaustive_depth2, export_genetic
from zkbench.tune.plot.genetic import plot_genetic


@click.command(name="genetic")
@click.option("--stats", required=True)
def plot_genetic_cli(stats: str):
    if not os.path.exists(stats):
        raise click.ClickException(f"File {stats} does not exist.")
    plot_genetic(stats)


@click.command(name="exhaustive-depth2")
@click.option("--stats", required=True)
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
def plot_exhaustive_depth2_cli(stats: str, program: str | None, zkvm: str | None):
    if not os.path.exists(stats):
        raise click.ClickException(f"File {stats} does not exist.")
    plot_exhaustive_depth2(stats, program, zkvm)


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
