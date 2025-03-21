import click

from zkbench.config import get_measurements, get_programs, get_zkvms
from zkbench.plot.average_by_profile import plot_average_improvement
from zkbench.plot.average_duration import plot_average_duration
from zkbench.plot.cycle_count import plot_cycle_count
from zkbench.plot.cycle_count_abs import plot_cycle_count_abs
from zkbench.plot.cycle_prove_duration import plot_cycle_count_duration
from zkbench.plot.prove_exec import plot_prove_exec


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
@click.option("--program", type=click.Choice(get_programs()), required=False)
def cycle_count_cli(program: str | None):
    dir = click.get_current_context().parent.params["dir"]
    plot_cycle_count(dir, program)


@click.command(name="cycle-count-abs")
@click.option("--program", type=click.Choice(get_programs()), required=True)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=True)
def cycle_count_abs_cli(program: str, zkvm: str):
    dir = click.get_current_context().parent.params["dir"]
    plot_cycle_count_abs(dir, program, zkvm)


@click.command(name="cycle-count-duration")
@click.option("--measurement", type=click.Choice(get_measurements()), required=True)
@click.option("--program", type=click.Choice(get_programs()), required=False)
def cycle_count_duration_cli(measurement: str, program: str | None):
    dir = click.get_current_context().parent.params["dir"]
    plot_cycle_count_duration(dir, measurement, program)


@click.command(name="prove-exec")
@click.option("--program", type=click.Choice(get_programs()), required=False)
def prove_exec_cli(program: str | None):
    dir = click.get_current_context().parent.params["dir"]
    plot_prove_exec(dir, program)
