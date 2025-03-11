import logging
import click

from zkbench.common import coro
from zkbench.config import get_measurements, get_profiles_ids, get_programs, get_zkvms
from zkbench.plot.plot import (
    average_duration_cli,
    average_improvement_cli,
    better_worse_cli,
)
from zkbench.bench import run_bench
from zkbench.build import run_build
from zkbench.clean import run_clean

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
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
@click.option("--profile", type=click.Choice(get_profiles_ids()), required=False)
@click.option("--force", required=False, is_flag=True, default=False)
@click.option("-j", required=True, type=int)
@coro
async def build_cli(
    program: str | None, zkvm: str | None, profile: str | None, force: bool, j: int
):
    await run_build(program, zkvm, profile, force, j)


@click.command(name="clean")
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
def clean_cli(program: str | None, zkvm: str | None):
    run_clean(program, zkvm)


@click.command(name="bench")
@click.option("--program", type=click.Choice(get_programs()), required=False)
@click.option("--zkvm", type=click.Choice(get_zkvms()), required=False)
@click.option("--measurement", type=click.Choice(get_measurements()), required=False)
def bench_cli(program: str | None, zkvm: str | None, measurement: str | None):
    run_bench(program, zkvm, measurement)


@click.group(name="plot")
@click.option("--dir", nargs=1, required=True, help="Directory with Criterion data")
def plot_cli(dir: str):
    pass


zkbench_cli.add_command(build_cli)
zkbench_cli.add_command(clean_cli)
zkbench_cli.add_command(bench_cli)
zkbench_cli.add_command(plot_cli)

plot_cli.add_command(better_worse_cli)
plot_cli.add_command(average_improvement_cli)
plot_cli.add_command(average_duration_cli)

if __name__ == '__main__':
    zkbench_cli()
