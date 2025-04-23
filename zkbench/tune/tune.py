import json
import os
import uuid
import click

from zkbench.plot.common import get_program_selection
from zkbench.tune.common import OUT_EXHAUSTIVE, OUT_GENETIC, TuneConfig
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
def tune_exhaustive_cli(depth: int):
    (selected_programs, zkvms, metric, config) = get_config()
    out_stats = os.path.join(
        OUT_EXHAUSTIVE,
        f"stats-{metric}-{str(uuid.uuid4())[:5]}.json",
    )
    os.makedirs(OUT_EXHAUSTIVE, exist_ok=True)
    run_tune_exhaustive(selected_programs, zkvms, metric, config, out_stats, depth)


@click.command(name="genetic")
@click.option(
    "--mode",
    type=click.Choice(["default", "depth"]),
    required=True,
    multiple=False,
)
@click.option("--depth", multiple=False, type=int, required=False)
def tune_genetic_cli(mode: str, depth: int | None):
    if mode == "depth" and depth is None:
        raise click.UsageError("Depth must be provided when mode is 'depth'.")

    (selected_programs, zkvms, metric, config) = get_config()
    out_stats = os.path.join(
        OUT_GENETIC,
        f"stats-{metric}-{str(uuid.uuid4())[:5]}.json",
    )
    os.makedirs(OUT_GENETIC, exist_ok=True)
    run_tune_genetic(selected_programs, zkvms, metric, config, mode, out_stats, depth)


def get_config():
    zkvms: list[str] = click.get_current_context().parent.params["zkvm"]
    programs: list[str] = click.get_current_context().parent.params["program"]
    program_groups: list[str] = click.get_current_context().parent.params[
        "program_group"
    ]
    metric: str = click.get_current_context().parent.params["metric"]
    config: TuneConfig = TuneConfig(
        **json.load(click.get_current_context().parent.params["config"])
    )

    selected_programs = get_program_selection(programs, program_groups)
    return (selected_programs, zkvms, metric, config)
