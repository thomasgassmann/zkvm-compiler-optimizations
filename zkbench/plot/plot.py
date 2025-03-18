import click

from zkbench.config import get_measurements, get_programs, get_zkvms
from zkbench.plot.average_by_profile import plot_average_improvement
from zkbench.plot.average_duration import plot_average_duration
from zkbench.plot.better_worse import plot_better_worse
from zkbench.plot.cycle_count import plot_cycle_count

@click.command(name="better-worse")
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
@click.option("--measurement", type=click.Choice(get_measurements()), required=False)
def better_worse_cli(zkvm: str | None, measurement: str | None):
    dir = click.get_current_context().parent.params['dir']
    plot_better_worse(dir, zkvm, measurement)


@click.command(name="average-improvement")
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
@click.option("--program", type=click.Choice(get_programs()), required=False)
def average_improvement_cli(zkvm: str | None, program: str | None):
    dir = click.get_current_context().parent.params["dir"]
    plot_average_improvement(dir, zkvm, program)


@click.command(name="average-duration")
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
@click.option("--measurement", type=click.Choice(get_measurements()), required=True)
@click.option("--program", type=click.Choice(get_programs()), required=False)
def average_duration_cli(zkvm: str | None, measurement: str, program: str | None):
    dir = click.get_current_context().parent.params["dir"]
    plot_average_duration(dir, zkvm, measurement, program)


@click.command(name="cycle-count")
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
@click.option("--program", type=click.Choice(get_programs()), required=False)
def cycle_count_cli(zkvm: str | None, program: str | None):
    dir = click.get_current_context().parent.params["dir"]
    plot_cycle_count(dir, zkvm, program)
