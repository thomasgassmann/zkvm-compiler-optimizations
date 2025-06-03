import logging
import click

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
    binsize_duration_cli,
    cycle_count_abs_cli,
    cycle_count_by_program_cli,
    cycle_count_cli,
    cycle_count_stats_cli,
    cycle_count_duration_cli,
    duration_by_program_cli,
    export_report_cli,
    improvement_by_program_cli,
    improvement_by_program_exec_cli,
    khz_cli,
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
    plot_exhaustive_depth2_cli,
    plot_genetic_cli,
    plot_genetic_individual_cli,
)
from zkbench.tune.tune import TUNE_METRICS, tune_exhaustive_cli, tune_genetic_cli


@click.group()
@click.option("--log-level", nargs=1, required=False)
@click.option("--log-file", nargs=1, required=False)
def zkbench_cli(log_level: str, log_file: str):
    setup_logger(log_level, log_file)


@click.command(name="build")
@click.option(
    "--program", type=click.Choice(get_programs()), required=False, multiple=True
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
@coro
async def build_cli(
    program: list[str],
    program_group: list[str],
    zkvm: list[str],
    profile: list[str],
    force: bool,
    j: int | None,
    llvm: bool,
):
    await run_build(
        list(program), list(program_group), zkvm, profile, force, j or 1, llvm
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
    "--program", type=click.Choice(get_programs()), required=False, multiple=True
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
@click.option(
    "--profile", type=click.Choice(get_profiles_ids()), required=False, multiple=True
)
@click.option("--profile-time", type=int, required=False)
@click.option("--force", required=False, is_flag=True, default=False)
@click.option("--meta-only", required=False, is_flag=True, default=False)
@click.option("--input-override", required=False, type=str)
@click.option("--sample-size", required=False, type=int)
def bench_cli(
    program: list[str],
    program_group: list[str],
    zkvm: list[str],
    measurement: list[str],
    profile: list[str],
    profile_time: int,
    force: bool,
    meta_only: bool,
    input_override: str | None,
    sample_size: int | None,
):
    run_bench(
        program,
        program_group,
        zkvm,
        measurement,
        profile,
        profile_time,
        force,
        meta_only,
        input_override,
        sample_size,
    )


@click.command(name="run")
@click.option("--program", type=click.Choice(get_programs()), required=True)
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
def plot_cli(dir: str):
    pass


@click.group(name="plot-tune")
def plot_tune_cli():
    pass


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

plot_tune_cli.add_command(plot_genetic_cli)
plot_tune_cli.add_command(plot_exhaustive_depth2_cli)
plot_tune_cli.add_command(export_exhaustive_depth2_cli)
plot_tune_cli.add_command(export_genetic_cli)
plot_tune_cli.add_command(plot_genetic_individual_cli)

tune_cli.add_command(tune_genetic_cli)
tune_cli.add_command(tune_exhaustive_cli)

if __name__ == "__main__":
    zkbench_cli()
