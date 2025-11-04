import logging
import click
from matplotlib import pyplot as plt

from zkbench.asm import show_asm
from zkbench.common import coro, setup_logger
from zkbench.config import (
    get_measurements,
    get_profiles_ids,
    get_program_groups,
    get_programs,
    get_zkvms,
    get_zkvms_with_x86,
)
from zkbench.plot.plot import (
    average_duration_cli,
    average_improvement_cli,
    average_improvement_compare_cli,
    average_improvement_difference_cli,
    average_improvement_zkvm_cli,
    binsize_duration_cli,
    cycle_count_abs_cli,
    cycle_count_by_program_cli,
    cycle_count_by_program_zkvm_cli,
    cycle_count_cli,
    cycle_count_single_program_cli,
    cycle_count_stats_cli,
    cycle_count_duration_cli,
    duration_by_program_cli,
    duration_cli,
    duration_single_program_cli,
    export_report_cli,
    improvement_by_program_cli,
    improvement_by_program_exec_cli,
    improvement_by_program_zkvm_cli,
    improvement_for_profile_cli,
    improvement_number_of_programs_cli,
    improvement_profiles_overview_cli,
    improvement_programs_overview_cli,
    improvement_single_program_cli,
    khz_cli,
    metric_overview_cli,
    no_effect_cli,
    opt_by_program_cli,
    opt_no_effect_cli,
    paging_by_profile_cli,
    plot_missing_cli,
    prove_exec_cli,
    rca_classify_cli,
    stddev_cli,
    total_time_by_profile_cli,
    x86_exec_cli,
)
from zkbench.bench import run_bench
from zkbench.build import run_build
from zkbench.clean import run_clean
from zkbench.run import run_single
from zkbench.tune.plot.plot import (
    export_exhaustive_depth2_cli,
    export_genetic_cli,
    export_genetic_individual_cli,
    extract_genetic_individual_cli,
    plot_exhaustive_depth2_cli,
    plot_ffd1d_cli,
    plot_ffd2d_cli,
    plot_genetic_cli,
    plot_genetic_individual_cli,
)
from zkbench.tune.tune import (
    TUNE_METRICS,
    tune_exhaustive_cli,
    tune_ffd_cli,
    tune_genetic_cli,
)


@click.group()
@click.option("--log-level", nargs=1, required=False)
@click.option("--log-file", nargs=1, required=False)
def zkbench_cli(log_level: str, log_file: str):
    setup_logger(log_level, log_file)


@click.command(name="build")
@click.option(
    "--program",
    type=click.Choice(get_programs() + ["example"]),
    required=False,
    multiple=True,
)
@click.option(
    "--ignore-program", type=click.Choice(get_programs()), required=False, multiple=True
)
@click.option(
    "--program-group",
    type=click.Choice(get_program_groups()),
    required=False,
    multiple=True,
)
@click.option(
    "--zkvm", type=click.Choice(get_zkvms_with_x86()), required=False, multiple=True
)
@click.option(
    "--profile", type=click.Choice(get_profiles_ids()), required=False, multiple=True
)
@click.option("--force", required=False, is_flag=True, default=False)
@click.option("-j", required=False, type=int)
@click.option("--llvm", required=False, is_flag=True, default=False)
@click.option("--build-by-program", required=False, is_flag=True, default=False)
@click.option("--feature", required=False, type=str, multiple=True)
@click.option("--name", required=False, type=str, default=None)
@click.option("--clean-after-build", required=False, is_flag=True, default=False)
@click.option("--subtractive", required=False, is_flag=True, default=False)
@coro
async def build_cli(
    program: list[str],
    ignore_program: list[str],
    program_group: list[str],
    zkvm: list[str],
    profile: list[str],
    force: bool,
    j: int | None,
    llvm: bool,
    feature: list[str] | None,
    name: str | None,
    build_by_program: bool = False,
    clean_after_build: bool = False,
    subtractive: bool = False,
):
    await run_build(
        list(program),
        list(ignore_program),
        list(program_group),
        zkvm,
        profile,
        force,
        j or 1,
        llvm,
        features=feature,
        name=name,
        build_by_program=build_by_program,
        clean_after_build=clean_after_build,
        subtractive=subtractive,
    )


@click.command(name="asm")
@click.option(
    "--program",
    type=click.Choice(get_programs() + ["example"]),
    required=True,
    multiple=False,
)
@click.option(
    "--zkvm", type=click.Choice(get_zkvms_with_x86()), required=True, multiple=False
)
@click.option(
    "--profile", type=click.Choice(get_profiles_ids()), required=True, multiple=False
)
@click.option("--llvm", required=False, is_flag=True, default=False)
@click.option("--rust", required=False, is_flag=True, default=False)
@click.option("--open", required=False, is_flag=True, default=False)
@click.option("--feature", required=False, type=str, multiple=True)
@click.argument("rest", required=False, default="")
@click.option("--subtractive", required=False, is_flag=True, default=False)
@coro
async def asm_cli(
    program: str,
    zkvm: str,
    profile: str,
    llvm: bool,
    rust: bool,
    feature: list[str],
    rest: str,
    open: bool,
    subtractive: bool = False,
):
    show_asm(
        program, zkvm, profile, feature, llvm, rust, rest, open, subtractive=subtractive
    )


@click.command(name="clean")
@click.option(
    "--program", type=click.Choice(get_programs()), required=False, multiple=True
)
@click.option(
    "--zkvm", type=click.Choice(get_zkvms_with_x86()), required=False, multiple=True
)
@coro
async def clean_cli(program: list[str], zkvm: list[str]):
    await run_clean(program, zkvm)


@click.command(name="bench")
@click.option(
    "--program",
    type=click.Choice(get_programs() + ["example"]),
    required=False,
    multiple=True,
)
@click.option(
    "--ignore-program", type=click.Choice(get_programs()), required=False, multiple=True
)
@click.option(
    "--program-group",
    type=click.Choice(get_program_groups()),
    required=False,
    multiple=True,
)
@click.option(
    "--zkvm", type=click.Choice(get_zkvms_with_x86()), required=False, multiple=True
)
@click.option(
    "--measurement",
    type=click.Choice(get_measurements()),
    required=False,
    multiple=True,
)
@click.option("--profile", type=str, required=False, multiple=True)
@click.option("--profile-time", type=int, required=False)
@click.option("--force", required=False, is_flag=True, default=False)
@click.option("--meta-only", required=False, is_flag=True, default=False)
@click.option("--input-override", required=False, type=str)
@click.option("--sample-size", required=False, type=int)
@click.option("--sampling-mode", type=click.Choice(["flat", "linear"]), required=False)
@click.option("--euler-d", type=int, required=False)
@click.option("--euler-h", type=int, required=False)
@click.option("--euler-log-out", type=str, required=False)
@click.option("--euler-criterion-home", type=str, required=False)
@click.option("--euler-dry", required=False, is_flag=True, default=False)
@click.option(
    "--runner-path",
    type=click.Path(exists=True, file_okay=True, dir_okay=False),
    required=False,
    default="./target/release/runner",
)
@click.option("--measurement-time", type=int, required=False)
@click.option("--warmup-time", type=int, required=False)
@click.option("--nresamples", type=int, required=False)
def bench_cli(
    program: list[str],
    ignore_program: list[str],
    program_group: list[str],
    zkvm: list[str],
    measurement: list[str],
    profile: list[str],
    profile_time: int,
    force: bool,
    meta_only: bool,
    input_override: str | None,
    sample_size: int | None,
    sampling_mode: str | None,
    runner_path: str | None,
    euler_d: int | None,
    euler_h: int | None,
    euler_log_out: str | None,
    euler_criterion_home: str | None,
    euler_dry: bool = False,
    measurement_time: int | None = None,
    warmup_time: int | None = None,
    nresamples: int | None = None,
):
    run_bench(
        program,
        ignore_program,
        program_group,
        zkvm,
        measurement,
        profile,
        profile_time,
        force,
        meta_only,
        input_override,
        sample_size,
        sampling_mode,
        runner_path,
        euler_d,
        euler_h,
        euler_log_out,
        euler_criterion_home,
        euler_dry,
        measurement_time,
        warmup_time,
        nresamples,
    )


@click.command(name="run")
@click.option(
    "--program", type=click.Choice(get_programs() + ["example"]), required=True
)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=True)
@click.option(
    "--elf",
    type=click.Path(exists=True, file_okay=True, dir_okay=False),
    required=True,
)
@click.option("--force", required=False, is_flag=True, default=False)
def run_single_cli(program: str, zkvm: str, elf: str, force: bool):
    run_single(program, zkvm, elf, force)


@click.group(name="plot")
@click.option("--dir", nargs=1, required=True, help="Directory with Criterion data")
@click.option("--remove-ox", required=False, is_flag=True, default=False)
@click.option("--only-ox", required=False, is_flag=True, default=False)
@click.option("--violin", required=False, is_flag=True, default=False)
@click.option("--vertical", required=False, is_flag=True, default=False)
@click.option("--font-size", type=int, default=None, help="Font size for plots")
def plot_cli(dir: str, remove_ox: bool, only_ox: bool, violin: bool, vertical: bool, font_size: int | None):
    if font_size is not None:
        plt.rcParams.update({'font.size': font_size})


@click.group(name="plot-tune")
@click.option("--font-size", type=int, default=None, help="Font size for plots")
def plot_tune_cli(font_size: int | None):
    if font_size is not None:
        plt.rcParams.update({'font.size': font_size})


@click.group(name="tune")
@click.option(
    "--program",
    type=click.Choice(get_programs()),
    required=False,
    multiple=True,
    default=[],
)
@click.option(
    "--zkvm",
    type=click.Choice(get_zkvms()),
    required=True,
    multiple=True,
    default=get_zkvms(),
)
@click.option(
    "--program-group",
    type=click.Choice(get_program_groups()),
    required=False,
    multiple=True,
    default=[],
)
@click.option(
    "--metric",
    type=click.Choice(TUNE_METRICS),
    required=True,
    multiple=False,
)
@click.option(
    "--config",
    multiple=False,
    required=True,
    type=click.File("r"),
)
@click.option("--out", nargs=1, required=True, help="Output directory")
def tune_cli(
    program: list[str],
    zkvm: list[str],
    program_group: list[str],
    metric: str,
    config,
    out: str,
):
    pass


zkbench_cli.add_command(build_cli)
zkbench_cli.add_command(clean_cli)
zkbench_cli.add_command(bench_cli)
zkbench_cli.add_command(run_single_cli)
zkbench_cli.add_command(plot_cli)
zkbench_cli.add_command(tune_cli)
zkbench_cli.add_command(plot_tune_cli)
zkbench_cli.add_command(asm_cli)

plot_cli.add_command(average_improvement_cli)
plot_cli.add_command(average_duration_cli)
plot_cli.add_command(cycle_count_cli)
plot_cli.add_command(cycle_count_duration_cli)
plot_cli.add_command(cycle_count_stats_cli)
plot_cli.add_command(prove_exec_cli)
plot_cli.add_command(cycle_count_abs_cli)
plot_cli.add_command(opt_by_program_cli)
plot_cli.add_command(plot_missing_cli)
plot_cli.add_command(opt_no_effect_cli)
plot_cli.add_command(no_effect_cli)
plot_cli.add_command(khz_cli)
plot_cli.add_command(total_time_by_profile_cli)
plot_cli.add_command(export_report_cli)
plot_cli.add_command(paging_by_profile_cli)
plot_cli.add_command(binsize_duration_cli)
plot_cli.add_command(improvement_by_program_cli)
plot_cli.add_command(duration_by_program_cli)
plot_cli.add_command(cycle_count_by_program_cli)
plot_cli.add_command(stddev_cli)
plot_cli.add_command(rca_classify_cli)
plot_cli.add_command(x86_exec_cli)
plot_cli.add_command(average_improvement_compare_cli)
plot_cli.add_command(average_improvement_difference_cli)
plot_cli.add_command(improvement_by_program_exec_cli)
plot_cli.add_command(improvement_single_program_cli)
plot_cli.add_command(cycle_count_single_program_cli)
plot_cli.add_command(duration_cli)
plot_cli.add_command(duration_single_program_cli)
plot_cli.add_command(average_improvement_zkvm_cli)
plot_cli.add_command(improvement_for_profile_cli)
plot_cli.add_command(improvement_number_of_programs_cli)
plot_cli.add_command(cycle_count_by_program_zkvm_cli)
plot_cli.add_command(improvement_by_program_zkvm_cli)
plot_cli.add_command(metric_overview_cli)
plot_cli.add_command(improvement_profiles_overview_cli)
plot_cli.add_command(improvement_programs_overview_cli)

plot_tune_cli.add_command(plot_genetic_cli)
plot_tune_cli.add_command(plot_exhaustive_depth2_cli)
plot_tune_cli.add_command(export_exhaustive_depth2_cli)
plot_tune_cli.add_command(export_genetic_cli)
plot_tune_cli.add_command(export_genetic_individual_cli)
plot_tune_cli.add_command(plot_genetic_individual_cli)
plot_tune_cli.add_command(extract_genetic_individual_cli)
plot_tune_cli.add_command(plot_ffd1d_cli)
plot_tune_cli.add_command(plot_ffd2d_cli)

tune_cli.add_command(tune_genetic_cli)
tune_cli.add_command(tune_exhaustive_cli)
tune_cli.add_command(tune_ffd_cli)

if __name__ == "__main__":
    zkbench_cli()
