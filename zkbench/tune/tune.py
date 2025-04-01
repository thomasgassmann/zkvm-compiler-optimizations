import click

from zkbench.plot.common import get_program_selection
from zkbench.tune.genetic import run_tune_genetic


@click.command(name="genetic")
def tune_genetic_cli():
    zkvms: list[str] = click.get_current_context().parent.params["zkvm"]
    programs: list[str] = click.get_current_context().parent.params["program"]
    program_groups: list[str] = click.get_current_context().parent.params[
        "program_group"
    ]

    selected_programs = get_program_selection(programs, program_groups)
    run_tune_genetic(selected_programs, zkvms)
