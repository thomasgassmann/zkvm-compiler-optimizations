import logging
import click

from zkbench.common import coro
from zkbench.config import get_measurements, get_profiles_ids, get_programs, get_zkvms
from zkbench.plot.plot import (
    average_duration_cli,
    average_improvement_cli,
    cycle_count_abs_cli,
    cycle_count_cli,
    cycle_count_duration_cli,
    prove_exec_cli,
)
from zkbench.bench import run_bench
from zkbench.build import run_build
from zkbench.clean import run_clean
from zkbench.run import run_single

def get_log_level(level_str: str) -> int:
    try:
        try:
            import pydevd # type: ignore
            return logging.DEBUG
        except ImportError:
            return getattr(logging, level_str) if level_str else logging.INFO
    except AttributeError:
        raise click.ClickException(f'Log level {level_str} not found.')


def setup_logger(level_str: str):
    log_formatter = logging.Formatter(
        "%(asctime)s [%(threadName)-12.12s] [%(levelname)-5.5s]  %(message)s")
    root_logger = logging.getLogger()
    root_logger.propagate = True
    level = get_log_level(level_str)
    root_logger.setLevel(level)

    console_handler = logging.StreamHandler()
    console_handler.setFormatter(log_formatter)
    root_logger.addHandler(console_handler)


@click.group()
@click.option('--log-level', nargs=1, required=False)
def zkbench_cli(log_level: str):
    setup_logger(log_level)


@click.command(name="build")
@click.option(
    "--program", type=click.Choice(get_programs()), required=False, multiple=True
)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False, multiple=True)
@click.option(
    "--profile", type=click.Choice(get_profiles_ids()), required=False, multiple=True
)
@click.option("--force", required=False, is_flag=True, default=False)
@click.option("-j", required=True, type=int)
@click.option("--llvm", required=False, is_flag=True, default=False)
@coro
async def build_cli(
    program: list[str],
    zkvm: list[str],
    profile: list[str],
    force: bool,
    j: int,
    llvm: bool,
):
    await run_build(program, zkvm, profile, force, j, llvm)


@click.command(name="clean")
@click.option(
    "--program", type=click.Choice(get_programs()), required=False, multiple=True
)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False, multiple=True)
def clean_cli(program: list[str], zkvm: list[str]):
    run_clean(program, zkvm)


@click.command(name="bench")
@click.option(
    "--program", type=click.Choice(get_programs()), required=False, multiple=True
)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False, multiple=True)
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
def bench_cli(
    program: list[str],
    zkvm: list[str],
    measurement: list[str],
    profile: list[str],
    profile_time: int,
):
    run_bench(program, zkvm, measurement, profile, profile_time)


@click.command(name="run")
@click.option("--program", type=click.Choice(get_programs()), required=True)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=True)
@click.option(
    "--elf",
    type=click.Path(exists=True, file_okay=True, dir_okay=False),
    required=True,
)
def run_single_cli(program: str, zkvm: str, elf: str):
    run_single(program, zkvm, elf)


@click.group(name="plot")
@click.option("--dir", nargs=1, required=True, help="Directory with Criterion data")
def plot_cli(dir: str):
    pass


zkbench_cli.add_command(build_cli)
zkbench_cli.add_command(clean_cli)
zkbench_cli.add_command(bench_cli)
zkbench_cli.add_command(run_single_cli)
zkbench_cli.add_command(plot_cli)

plot_cli.add_command(average_improvement_cli)
plot_cli.add_command(average_duration_cli)
plot_cli.add_command(cycle_count_cli)
plot_cli.add_command(cycle_count_duration_cli)
plot_cli.add_command(prove_exec_cli)
plot_cli.add_command(cycle_count_abs_cli)

if __name__ == '__main__':
    zkbench_cli()
