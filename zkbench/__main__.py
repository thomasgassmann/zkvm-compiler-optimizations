import logging
import click

from zkbench.bench import run_bench
from zkbench.build import run_build
from zkbench.clean import run_clean
from zkbench.run import run_with_plot

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


@click.command(name='build')
@click.option('--program', nargs=1, required=False)
@click.option('--zkvm', nargs=1, required=False)
@click.option('--profile', nargs=1, required=False)
@click.option('--force', required=False, is_flag=True, default=False)
def build_cli(program: str | None, zkvm: str | None, profile: str | None, force: bool):
    run_build(program, zkvm, profile, force)

@click.command(name='clean')
@click.option('--program', nargs=1, required=False)
@click.option('--zkvm', nargs=1, required=False)
def clean_cli(program: str | None, zkvm: str | None):
    run_clean(program, zkvm)

@click.command(name='run')
def run_cli():
    run_with_plot()


@click.command(name="bench")
def bench_cli():
    run_bench()


zkbench_cli.add_command(build_cli)
zkbench_cli.add_command(clean_cli)
zkbench_cli.add_command(run_cli)
zkbench_cli.add_command(bench_cli)

if __name__ == '__main__':
    zkbench_cli()
