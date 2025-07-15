import os
import click

from zkbench.config import (
    get_measurements,
    get_profiles_ids,
    get_program_groups,
    get_programs,
    get_zkvms,
    get_zkvms_with_x86,
)
from zkbench.plot.average_improvement import plot_average_improvement
from zkbench.plot.average_duration import plot_average_duration
from zkbench.plot.average_improvement_compare import plot_average_improvement_compare
from zkbench.plot.average_improvement_difference import (
    plot_average_improvement_difference,
)
from zkbench.plot.average_improvement_zkvm import plot_average_improvement_zkvm
from zkbench.plot.average_khz import plot_khz
from zkbench.plot.binary_size_duration import plot_binsize_duration
from zkbench.plot.cycle_count_single_program import plot_cycle_count_for_single_program
from zkbench.plot.duration import plot_duration
from zkbench.plot.duration_single_program import plot_duration_for_single_program
from zkbench.plot.improvement_by_program_exec import plot_improvement_by_program_exec
from zkbench.plot.improvement_profile import plot_improvement_for_profile
from zkbench.plot.improvement_single_program import plot_improvement_for_single_program
from zkbench.plot.rca_classify import classify_rca
from zkbench.plot.stddev import list_by_stddev
from zkbench.plot.common import has_data_on
from zkbench.plot.cycle_count import plot_cycle_count
from zkbench.plot.cycle_count_abs import plot_cycle_count_abs
from zkbench.plot.cycle_count_by_program import plot_cycle_count_by_program
from zkbench.plot.cycle_count_duration import (
    plot_cycle_count_duration,
    plot_cycle_count_stats,
)
from zkbench.plot.duration_by_program import plot_duration_by_program
from zkbench.plot.export import export_report
from zkbench.plot.improvement_by_program import plot_improvement_by_program
from zkbench.plot.no_effect import plot_no_effect
from zkbench.plot.opt_by_program import plot_opt_by_program
from zkbench.plot.opt_no_effect import plot_opt_no_effect
from zkbench.plot.prove_exec import plot_prove_exec
from zkbench.plot.total_time_by_profile import plot_total_time_by_profile
from zkbench.plot.paging_by_profile import plot_paging_by_profile
from zkbench.plot.x86_exec import plot_x86_exec


@click.command(
    name="average-improvement", help="Plot average improvement compared to baseline"
)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option(
    "--program-group", type=click.Choice(get_program_groups()), required=False
)
@click.option("--speedup", type=bool, is_flag=True, required=False, default=False)
@click.option(
    "--global-average", type=bool, is_flag=True, required=False, default=False
)
@click.option("--show-x86", type=bool, is_flag=True, required=False, default=False)
@click.option(
    "--drop-below",
    type=float,
    required=False,
    default=None,
    help="Drop values below this threshold (in percent)",
)
def average_improvement_cli(
    zkvm: str | None,
    program: str | None,
    program_group: str | None,
    speedup: bool,
    global_average: bool,
    show_x86: bool,
    drop_below: float | None = None,
):
    dir = click.get_current_context().parent.params["dir"]
    plot_average_improvement(
        dir,
        zkvm,
        program,
        program_group,
        speedup,
        global_average,
        show_x86,
        drop_below=drop_below,
    )


@click.command(
    name="average-improvement-zkvm",
    help="Plot average improvement compared to baseline with each zkVM being a series",
)
@click.option("--measurement", type=click.Choice(get_measurements()), required=True)
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option(
    "--program-group", type=click.Choice(get_program_groups()), required=False
)
@click.option("--speedup", type=bool, is_flag=True, required=False, default=False)
@click.option(
    "--global-average", type=bool, is_flag=True, required=False, default=False
)
@click.option(
    "--drop-below",
    type=float,
    required=False,
    default=None,
    help="Drop values below this threshold (in percent)",
)
def average_improvement_zkvm_cli(
    measurement: str,
    program: str | None,
    program_group: str | None,
    speedup: bool,
    global_average: bool,
    drop_below: float | None = None,
):
    dir = click.get_current_context().parent.params["dir"]
    plot_average_improvement_zkvm(
        dir,
        measurement,
        program,
        program_group,
        speedup,
        global_average,
        drop_below=drop_below,
    )


@click.command(name="average-duration", help="Plot raw duration of measurements")
@click.option("--zkvm", type=click.Choice(get_zkvms_with_x86()), required=False)
@click.option("--measurement", type=click.Choice(get_measurements()), required=True)
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option(
    "--profile", type=click.Choice(get_profiles_ids()), required=False, multiple=True
)
@click.option("--single", is_flag=True, default=False, help="Plot single value")
def average_duration_cli(
    zkvm: str | None,
    measurement: str,
    program: str | None,
    profile: list[str] | None,
    single: bool = False,
):
    dir = click.get_current_context().parent.params["dir"]
    plot_average_duration(dir, zkvm, measurement, program, profile, single)


@click.command(
    name="duration", help="Plot raw duration of measurements for some program"
)
@click.option(
    "--program", type=click.Choice(get_programs()), required=False, multiple=True
)
@click.option(
    "--profile", type=click.Choice(get_profiles_ids()), required=True, multiple=False
)
@click.option(
    "--program-group", type=click.Choice(get_program_groups()), required=False
)
def duration_cli(program: list[str], profile: str, program_group: str | None):
    dir = click.get_current_context().parent.params["dir"]
    plot_duration(dir, program, program_group, profile)


@click.command(
    name="cycle-count", help="Plot relative cycle count compared to baseline"
)
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option(
    "--program-group", type=click.Choice(get_program_groups()), required=False
)
@click.option(
    "--profile", type=click.Choice(get_profiles_ids()), required=False, multiple=True
)
@click.option(
    "--global-average", type=bool, is_flag=True, required=False, default=False
)
@click.option("--show-x86", type=bool, is_flag=True, required=False, default=False)
@click.option(
    "--drop-below",
    type=float,
    required=False,
    default=None,
    help="Drop values below this threshold (in percent)",
)
def cycle_count_cli(
    program: str | None,
    program_group: str | None,
    profile: list[str] | None,
    global_average: bool,
    show_x86: bool,
    drop_below: float | None = None,
):
    dir = click.get_current_context().parent.params["dir"]
    plot_cycle_count(
        dir,
        program,
        program_group,
        list(profile) if profile else None,
        global_average,
        show_x86,
        drop_below=drop_below,
    )


@click.command(name="cycle-count-abs", help="Plot absolute cycle count")
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
def cycle_count_abs_cli(program: str | None, zkvm: str | None):
    dir = click.get_current_context().parent.params["dir"]
    plot_cycle_count_abs(dir, program, zkvm)


@click.command(
    name="cycle-count-duration",
    help="Plot cycle count vs. duration (for some given measurement)",
)
@click.option("--measurement", type=click.Choice(get_measurements()), required=True)
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option("--relative", is_flag=True, default=False)
def cycle_count_duration_cli(measurement: str, program: str | None, relative: bool):
    dir = click.get_current_context().parent.params["dir"]
    plot_cycle_count_duration(dir, measurement, program, relative)


@click.command(
    name="cycle-count-stats",
    help="Plot cycle count pearson vs. spearman coefficients for given measurement",
)
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


@click.command(
    name="opt-by-program",
    help="For specific profile, plot improvement/degradation by program",
)
@click.option("--profile", type=click.Choice(get_profiles_ids()), required=True)
@click.option("--zkvm", type=click.Choice(get_zkvms_with_x86()), required=False)
@click.option("--speedup", type=bool, is_flag=True, required=False, default=False)
@click.option("--show-x86", type=bool, is_flag=True, required=False, default=False)
def opt_by_program_cli(
    profile: str, zkvm: str | None, speedup: bool, show_x86: bool = False
):
    dir = click.get_current_context().parent.params["dir"]
    plot_opt_by_program(dir, profile, zkvm, speedup, show_x86)


@click.command(name="khz", help="Plot kHz by profile")
@click.option("--program", type=click.Choice(get_programs()))
@click.option("--zkvm", type=click.Choice(get_zkvms()))
def khz_cli(program: str | None, zkvm: str | None):
    dir = click.get_current_context().parent.params["dir"]
    plot_khz(dir, zkvm, program)


@click.command(name="missing", help="List all missing measurements")
@click.option("--measurement", type=click.Choice(get_measurements()), required=False)
@click.option("--zkvm", type=click.Choice(get_zkvms_with_x86()), required=False)
def plot_missing_cli(measurement: str | None, zkvm: str | None):
    measurements = get_measurements() if measurement is None else [measurement]
    zkvms = get_zkvms_with_x86() if zkvm is None else [zkvm]
    programs = get_programs()
    dir = click.get_current_context().parent.params["dir"]
    for m in measurements:
        for z in zkvms:
            for p in programs:
                if z == "x86" and m != "exec":
                    continue

                if not has_data_on(dir, p, z, m):
                    print(f"{p}-{z}-{m}")


@click.command(
    name="opt-no-effect",
    help="Show percentage of optimizations that had no effect (by program)",
)
@click.option("--zkvm", type=click.Choice(get_zkvms_with_x86()), required=False)
def opt_no_effect_cli(zkvm: str | None):
    dir = click.get_current_context().parent.params["dir"]
    plot_opt_no_effect(dir, zkvm)


@click.command(
    name="no-effect",
    help="Show for each optimization number of programs where it had no effect",
)
@click.option("--zkvm", type=click.Choice(get_zkvms_with_x86()), required=False)
@click.option(
    "--program-group", type=click.Choice(get_program_groups()), required=False
)
@click.option("--program", type=click.Choice(get_programs()), required=False)
def no_effect_cli(zkvm: str | None, program_group: str | None, program: str | None):
    dir = click.get_current_context().parent.params["dir"]
    plot_no_effect(dir, zkvm, program_group, program)


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
@click.option("--profile", type=str, required=True)
@click.option("--baseline-profile", type=str, required=True)
@click.option("--speedup", type=bool, is_flag=True, required=False, default=False)
@click.option("--show-x86", type=bool, is_flag=True, required=False, default=False)
@click.option(
    "--measurement", type=click.Choice(get_measurements()), required=False, default=None
)
def improvement_by_program_cli(
    profile: str,
    baseline_profile: str,
    speedup: bool,
    show_x86: bool,
    measurement: str | None,
):
    dir = click.get_current_context().parent.params["dir"]

    plot_improvement_by_program(
        dir, profile, baseline_profile, speedup, show_x86, measurement
    )


@click.command(
    name="improvement-for-profile",
    help="Show (average) improvement for some profile compared to some other baseline profile by program",
)
@click.option("--profile", type=str, required=True)
@click.option("--baseline-profile", type=str, required=True)
@click.option("--speedup", type=bool, is_flag=True, required=False, default=False)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
@click.option("--measurement", type=click.Choice(get_measurements()), required=False)
def improvement_for_profile_cli(
    profile: str,
    baseline_profile: str,
    speedup: bool,
    zkvm: str | None,
    measurement: str | None = None,
):
    dir = click.get_current_context().parent.params["dir"]

    plot_improvement_for_profile(
        dir, profile, baseline_profile, speedup, zkvm, measurement
    )


@click.command(
    name="improvement-single-program",
    help="Show (average) improvement for some profile compared to some other baseline profile for a single program",
)
@click.option("--program", type=str, required=True)
@click.option("--profile", type=str, required=True, multiple=True)
@click.option("--baseline-profile", type=str, required=True)
@click.option("--speedup", type=bool, is_flag=True, required=False, default=False)
@click.option("--show-x86", type=bool, is_flag=True, required=False, default=False)
def improvement_single_program_cli(
    program: str,
    profile: list[str],
    baseline_profile: str,
    speedup: bool,
    show_x86: bool,
):
    dir = click.get_current_context().parent.params["dir"]

    plot_improvement_for_single_program(
        dir, program, profile, baseline_profile, speedup, show_x86
    )


@click.command(
    name="duration-single-program",
    help="Show raw duration for some profile compared to some other baseline profile for a single program",
)
@click.option("--program", type=str, required=True)
@click.option("--profile", type=str, required=True, multiple=True)
@click.option("--show-x86", type=bool, is_flag=True, required=False, default=False)
def duration_single_program_cli(
    program: str,
    profile: list[str],
    show_x86: bool,
):
    dir = click.get_current_context().parent.params["dir"]

    plot_duration_for_single_program(dir, program, profile, show_x86)


@click.command(
    name="cycle-count-single-program",
    help="Show cycle count for some profiles compared to some other baseline profile for a single program",
)
@click.option("--program", type=click.Choice(get_programs()), required=True)
@click.option(
    "--profile", type=click.Choice(get_profiles_ids()), required=True, multiple=True
)
@click.option(
    "--baseline-profile", type=click.Choice(get_profiles_ids()), required=True
)
@click.option("--abs", type=bool, is_flag=True, required=False, default=False)
def cycle_count_single_program_cli(
    program: str,
    profile: list[str],
    baseline_profile: str,
    abs: bool,
):
    dir = click.get_current_context().parent.params["dir"]

    plot_cycle_count_for_single_program(dir, program, profile, baseline_profile, abs)


@click.command(
    name="improvement-by-program-exec",
    help="Show (average) improvement for some profile compared to some other baseline profile by program for exec",
)
@click.option("--profile", type=click.Choice(get_profiles_ids()), required=True)
@click.option(
    "--baseline-profile", type=click.Choice(get_profiles_ids()), required=True
)
@click.option("--speedup", type=bool, is_flag=True, required=False, default=False)
@click.option("--show-x86", type=bool, is_flag=True, required=False, default=False)
def improvement_by_program_exec_cli(
    profile: str, baseline_profile: str, speedup: bool, show_x86: bool
):
    dir = click.get_current_context().parent.params["dir"]

    plot_improvement_by_program_exec(dir, profile, baseline_profile, speedup, show_x86)


@click.command(
    name="duration-by-program",
    help="Show duration for some profiles by program",
)
@click.option("--profile", type=click.Choice(get_profiles_ids()), required=True)
@click.option(
    "--baseline-profile", type=click.Choice(get_profiles_ids()), required=True
)
@click.option("--measurement", type=click.Choice(get_measurements()), required=True)
@click.option("--zkvm", type=click.Choice(get_zkvms_with_x86()), required=False)
def duration_by_program_cli(
    profile: str, baseline_profile: str, measurement: str, zkvm: str | None
):
    dir = click.get_current_context().parent.params["dir"]

    plot_duration_by_program(dir, profile, baseline_profile, measurement, zkvm)


@click.command(
    name="cycle-count-by-program",
    help="Show cycle count for some profiles by program",
)
@click.option("--profile", type=str, required=True, multiple=True)
@click.option("--baseline-profile", type=str, required=True)
@click.option("--relative", is_flag=True, default=False)
@click.option("--zkvm", type=click.Choice(get_zkvms_with_x86()), required=False)
def cycle_count_by_program_cli(
    profile: list[str], baseline_profile: str, relative: bool, zkvm: str | None
):
    dir = click.get_current_context().parent.params["dir"]

    plot_cycle_count_by_program(dir, list(profile), baseline_profile, relative, zkvm)


@click.command(
    name="stddev",
    help="Show cases where standard deviation is too high",
)
@click.option("--threshold", type=int, required=True)
@click.option("--measurement", type=click.Choice(get_measurements()), required=False)
def stddev_cli(threshold: int, measurement: str | None):
    dir = click.get_current_context().parent.params["dir"]

    list_by_stddev(dir, threshold, measurement)


@click.command(
    name="rca-classify",
    help="Find cases for root cause analysis",
)
@click.option("--threshold", type=float, required=True)
@click.option(
    "--strategy",
    type=click.Choice(["improve", "degrade", "exec_prove"]),
    required=True,
)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
@click.option("--measurement", type=click.Choice(get_measurements()), required=False)
@click.option("--avg-programs", type=bool, required=False, default=False)
def rca_classify_cli(
    threshold: float,
    avg_programs: bool,
    strategy: str,
    measurement: str | None,
    zkvm: str | None,
):
    dir = click.get_current_context().parent.params["dir"]

    if strategy in ["improve", "degrade"] and not measurement:
        raise click.UsageError(
            "Measurement is required for 'improve' and 'degrade' strategies."
        )
    if strategy == "exec_prove" and measurement:
        raise click.UsageError("Measurement is not allowed for 'exec_prove' strategy.")

    classify_rca(dir, threshold, avg_programs, strategy, measurement, zkvm)


@click.command(
    name="x86-exec",
    help="Compare x86 execution time with zkVM execution",
)
@click.option("--measurement", type=click.Choice(get_measurements()), required=True)
def x86_exec_cli(
    measurement: str,
):
    dir = click.get_current_context().parent.params["dir"]

    plot_x86_exec(dir, measurement)


@click.command(
    name="average-improvement-compare",
    help="Compare average improvement between two zkVMs",
)
@click.option("--zkvm-a", type=click.Choice(get_zkvms_with_x86()), required=True)
@click.option("--zkvm-b", type=click.Choice(get_zkvms_with_x86()), required=True)
@click.option("--measurement-a", type=click.Choice(get_measurements()), required=True)
@click.option("--measurement-b", type=click.Choice(get_measurements()), required=True)
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option(
    "--program-group", type=click.Choice(get_program_groups()), required=False
)
@click.option("--speedup", type=bool, is_flag=True, required=False, default=False)
def average_improvement_compare_cli(
    zkvm_a: str,
    zkvm_b: str,
    measurement_a: str,
    measurement_b: str,
    program: str | None,
    program_group: str | None,
    speedup: bool,
):
    dir = click.get_current_context().parent.params["dir"]

    plot_average_improvement_compare(
        dir,
        zkvm_a,
        zkvm_b,
        measurement_a,
        measurement_b,
        program,
        program_group,
        speedup,
    )


@click.command(
    name="average-improvement-difference",
    help="Measure average improvement between zkVM and x86",
)
@click.option("--zkvm", type=click.Choice(get_zkvms_with_x86()), required=False)
@click.option("--speedup", type=bool, is_flag=True, required=False, default=False)
def average_improvement_difference_cli(
    zkvm: str | None,
    speedup: bool,
):
    dir = click.get_current_context().parent.params["dir"]

    plot_average_improvement_difference(
        dir,
        speedup,
        zkvm,
    )
