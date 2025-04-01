import click

from zkbench.tune.genetic import run_tune_genetic


@click.command(name="genetic")
def tune_genetic_cli():
    zkvms: list[str] = click.get_current_context().parent.params["zkvm"]
    programs: list[str] = click.get_current_context().parent.params["program"]
    run_tune_genetic(programs, zkvms)
