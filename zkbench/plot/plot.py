import os
import click

from zkbench.config import (
    get_measurements,
    get_profiles_ids,
    get_program_groups,
    get_programs,
    get_zkvms,
)
from zkbench.plot.average_improvement import plot_average_improvement
from zkbench.plot.average_duration import plot_average_duration
from zkbench.plot.common import has_data_on
from zkbench.plot.cycle_count import plot_cycle_count
from zkbench.plot.cycle_count_abs import plot_cycle_count_abs
from zkbench.plot.cycle_prove_duration import plot_cycle_count_duration
from zkbench.plot.genetic import plot_genetic
from zkbench.plot.opt_by_program import plot_opt_by_program
from zkbench.plot.prove_exec import plot_prove_exec

@click.command(name="average-improvement")
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option(
    "--program-group", type=click.Choice(get_program_groups()), required=False
)
@click.option("--speedup", type=bool, is_flag=True, required=False, default=False)
def average_improvement_cli(
    zkvm: str | None, program: str | None, program_group: str | None, speedup: bool
):
    dir = click.get_current_context().parent.params["dir"]
    plot_average_improvement(dir, zkvm, program, program_group, speedup)


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
@click.option("--relative", is_flag=True, default=False)
def cycle_count_duration_cli(measurement: str, program: str | None, relative: bool):
    dir = click.get_current_context().parent.params["dir"]
    plot_cycle_count_duration(dir, measurement, program, relative)


@click.command(name="prove-exec")
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option(
    "--program-group", type=click.Choice(get_program_groups()), required=False
)
def prove_exec_cli(program: str | None, program_group: str | None):
    dir = click.get_current_context().parent.params["dir"]
    plot_prove_exec(dir, program, program_group)


@click.command(name="opt-by-program")
@click.option("--profile", type=click.Choice(get_profiles_ids()), required=True)
def opt_by_program_cli(profile: str):
    dir = click.get_current_context().parent.params["dir"]
    plot_opt_by_program(dir, profile)


@click.command(name="tune-genetic")
@click.option("--stats", required=True)
def plot_genetic_cli(stats: str):
    if not os.path.exists(stats):
        raise click.ClickException(f"File {stats} does not exist.")
    plot_genetic(stats)


@click.command(name="missing")
@click.option("--measurement", type=click.Choice(get_measurements()), required=False)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
def plot_missing_cli(measurement: str | None, zkvm: str | None):
    measurements = get_measurements() if measurement is None else [measurement]
    zkvms = get_zkvms() if zkvm is None else [zkvm]
    programs = get_programs()
    dir = click.get_current_context().parent.params["dir"]
    for m in measurements:
        for z in zkvms:
            for p in programs:
                if not has_data_on(dir, p, z, m):
                    print(f"{p}-{z}-{m}")
