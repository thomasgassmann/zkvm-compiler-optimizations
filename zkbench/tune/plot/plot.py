
import os
import click

from zkbench.tune.plot.exhaustive import plot_exhaustive_depth2
from zkbench.tune.plot.genetic import plot_genetic


@click.command(name="genetic")
@click.option("--stats", required=True)
def plot_genetic_cli(stats: str):
    if not os.path.exists(stats):
        raise click.ClickException(f"File {stats} does not exist.")
    plot_genetic(stats)

@click.command(name="exhaustive-depth2")
@click.option("--stats", required=True)
def plot_exhaustive_depth2_cli(stats: str):
    if not os.path.exists(stats):
        raise click.ClickException(f"File {stats} does not exist.")
    plot_exhaustive_depth2(stats)
