import json
import os
import click

from zkbench.config import get_profiles_ids
from zkbench.plot.common import get_program_selection
from zkbench.tune.common import TuneConfig
from zkbench.tune.exhaustive import run_tune_exhaustive
from zkbench.tune.genetic import run_tune_genetic

TUNE_METRICS = ["cycle-count", "prove-time", "gas", "exec-time", "paging-cycle-count"]


@click.command(name="exhaustive")
@click.option(
    "--depth",
    multiple=False,
    type=int,
    help="Depth to test",
)
def tune_exhaustive_cli(depth: int):
    (selected_programs, zkvms, metric, config, out) = get_config()
    os.makedirs(out, exist_ok=True)
    run_tune_exhaustive(selected_programs, zkvms, metric, config, out, depth)


@click.command(name="genetic")
@click.option(
    "--mode",
    type=click.Choice(["default", "depth"]),
    required=True,
    multiple=False,
)
@click.option("--depth", multiple=False, type=int, required=False)
@click.option(
    "--baseline",
    multiple=True,
    type=click.Choice(get_profiles_ids()),
    required=False,
)
@click.option("--individual", type=bool, default=False, is_flag=True)
def tune_genetic_cli(
    mode: str, depth: int | None, baseline: list[str] | None, individual: bool
):
    if mode == "depth" and depth is None:
        raise click.UsageError("Depth must be provided when mode is 'depth'.")

    (selected_programs, zkvms, metric, config, out) = get_config()
    os.makedirs(out, exist_ok=True)
    run_tune_genetic(
        selected_programs, zkvms, metric, config, mode, out, depth, baseline, individual
    )


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
    output_dir: str = click.get_current_context().parent.params["out"]

    selected_programs = get_program_selection(programs, program_groups)
    return (selected_programs, zkvms, metric, config, output_dir)
