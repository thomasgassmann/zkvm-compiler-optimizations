import click

from zkbench.plot.common import get_program_selection
from zkbench.tune.exhaustive import run_tune_exhaustive
from zkbench.tune.genetic import run_tune_genetic

TUNE_METRICS = ["cycle-count", "prove-time"]


@click.command(name="exhaustive")
@click.option(
    "--depth",
    multiple=False,
    type=int,
    help="Depth to test",
)
def tune_exhaustive_cli():
    (selected_programs, zkvms, metric) = get_config()
    run_tune_exhaustive(selected_programs, zkvms, metric)


@click.command(name="genetic")
def tune_genetic_cli():
    (selected_programs, zkvms, metric) = get_config()
    run_tune_genetic(selected_programs, zkvms, metric)


def get_config():
    zkvms: list[str] = click.get_current_context().parent.params["zkvm"]
    programs: list[str] = click.get_current_context().parent.params["program"]
    program_groups: list[str] = click.get_current_context().parent.params[
        "program_group"
    ]
    metric: str = click.get_current_context().parent.params["metric"]

    selected_programs = get_program_selection(programs, program_groups)
    return (selected_programs, zkvms, metric)
