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
from zkbench.plot.average_khz import plot_khz
from zkbench.plot.binary_size_duration import plot_binsize_duration
from zkbench.plot.common import has_data_on
from zkbench.plot.cycle_count import plot_cycle_count
from zkbench.plot.cycle_count_abs import plot_cycle_count_abs
from zkbench.plot.cycle_count_duration import (
    plot_cycle_count_duration,
    plot_cycle_count_stats,
)
from zkbench.plot.export import export_report
from zkbench.plot.improvement_by_program import plot_improvement_by_program
from zkbench.plot.no_effect import plot_no_effect
from zkbench.plot.opt_by_program import plot_opt_by_program
from zkbench.plot.opt_no_effect import plot_opt_no_effect
from zkbench.plot.prove_exec import plot_prove_exec
from zkbench.plot.total_time_by_profile import plot_total_time_by_profile
from zkbench.plot.paging_by_profile import plot_paging_by_profile


@click.command(name="average-improvement")
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option(
    "--program-group", type=click.Choice(get_program_groups()), required=False
)
@click.option("--speedup", type=bool, is_flag=True, required=False, default=False)
@click.option(
    "--global-average", type=bool, is_flag=True, required=False, default=False
)
def average_improvement_cli(
    zkvm: str | None,
    program: str | None,
    program_group: str | None,
    speedup: bool,
    global_average: bool,
):
    dir = click.get_current_context().parent.params["dir"]
    plot_average_improvement(dir, zkvm, program, program_group, speedup, global_average)


@click.command(name="average-duration")
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
@click.option("--measurement", type=click.Choice(get_measurements()), required=True)
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option(
    "--profile", type=click.Choice(get_profiles_ids()), required=False, multiple=True
)
def average_duration_cli(
    zkvm: str | None, measurement: str, program: str | None, profile: list[str] | None
):
    dir = click.get_current_context().parent.params["dir"]
    plot_average_duration(dir, zkvm, measurement, program, profile)


@click.command(name="cycle-count")
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option(
    "--profile", type=click.Choice(get_profiles_ids()), required=False, multiple=True
)
def cycle_count_cli(program: str | None, profile: list[str] | None):
    dir = click.get_current_context().parent.params["dir"]
    plot_cycle_count(dir, program, list(profile) if profile else None)


@click.command(name="cycle-count-abs")
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
def cycle_count_abs_cli(program: str | None, zkvm: str | None):
    dir = click.get_current_context().parent.params["dir"]
    plot_cycle_count_abs(dir, program, zkvm)


@click.command(name="cycle-count-duration")
@click.option("--measurement", type=click.Choice(get_measurements()), required=True)
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option("--relative", is_flag=True, default=False)
def cycle_count_duration_cli(measurement: str, program: str | None, relative: bool):
    dir = click.get_current_context().parent.params["dir"]
    plot_cycle_count_duration(dir, measurement, program, relative)


@click.command(name="cycle-count-stats")
@click.option("--measurement", type=click.Choice(get_measurements()), required=True)
@click.option("--relative", is_flag=True, default=False)
def cycle_count_stats_cli(measurement: str, relative: bool):
    dir = click.get_current_context().parent.params["dir"]
    plot_cycle_count_stats(dir, measurement, relative)


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
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
def opt_by_program_cli(profile: str, zkvm: str | None):
    dir = click.get_current_context().parent.params["dir"]
    plot_opt_by_program(dir, profile, zkvm)


@click.command(name="khz")
@click.option("--program", type=click.Choice(get_programs()))
@click.option("--zkvm", type=click.Choice(get_zkvms()))
def khz_cli(program: str | None, zkvm: str | None):
    dir = click.get_current_context().parent.params["dir"]
    plot_khz(dir, zkvm, program)


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


@click.command(
    name="opt-no-effect",
    help="Show percentage of optimizations that had no effect (by program)",
)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
def opt_no_effect_cli(zkvm: str | None):
    dir = click.get_current_context().parent.params["dir"]
    plot_opt_no_effect(dir, zkvm)


@click.command(
    name="no-effect",
    help="Show for each optimization number of programs where it had no effect",
)
def no_effect_cli():
    dir = click.get_current_context().parent.params["dir"]
    plot_no_effect(dir)


@click.command(
    name="total-time-by-profile",
    help="Total exec/prove time by profile",
)
@click.option("--program", type=click.Choice(get_programs()))
@click.option("--zkvm", type=click.Choice(get_zkvms()))
@click.option("--measurement", type=click.Choice(get_measurements()), required=True)
def total_time_by_profile_cli(program: str | None, zkvm: str | None, measurement: str):
    dir = click.get_current_context().parent.params["dir"]
    plot_total_time_by_profile(dir, zkvm, program, measurement)


@click.command(
    name="export",
    help="Export plots to report",
)
@click.option("--out", nargs=1, required=True, help="Output directory")
def export_report_cli(out: str):
    dir = click.get_current_context().parent.params["dir"]
    export_report(dir, out)


@click.command(
    name="paging-by-profile",
    help="Plot paging by profile (currently only risc0)",
)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=True)
@click.option("--program", type=click.Choice(get_programs()), required=False)
def paging_by_profile_cli(zkvm: str, program: str | None):
    dir = click.get_current_context().parent.params["dir"]

    plot_paging_by_profile(dir, zkvm, program)


@click.command(
    name="bin-size-duration",
    help="Plot duration as function of binary size",
)
@click.option("--measurement", type=click.Choice(get_measurements()), required=True)
@click.option("--program", type=click.Choice(get_programs()), required=False)
def binsize_duration_cli(measurement: str, program: str | None):
    dir = click.get_current_context().parent.params["dir"]

    plot_binsize_duration(dir, program, measurement)


@click.command(
    name="improvement-by-program",
    help="Show (average) improvement for some profile compared to some other baseline profile by program",
)
@click.option("--profile", type=click.Choice(get_profiles_ids()), required=True)
@click.option(
    "--baseline-profile", type=click.Choice(get_profiles_ids()), required=True
)
@click.option("--speedup", type=bool, is_flag=True, required=False, default=False)
def improvement_by_program_cli(profile: str, baseline_profile: str, speedup: bool):
    dir = click.get_current_context().parent.params["dir"]

    plot_improvement_by_program(dir, profile, baseline_profile, speedup)
