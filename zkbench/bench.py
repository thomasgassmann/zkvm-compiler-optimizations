from zkbench.common import get_run_config


def run_bench(program: str | None, zkvm: str | None, profile: str | None):
    programs, zkvms, profiles = get_run_config(program, zkvm, profile)
