import logging
import os
from zkbench.common import get_run_config, run_command
from zkbench.config import get_program_path, is_zkvm_specific


async def run_clean(program: list[str], zkvm: list[str], get_path=None):
    programs_to_build, zkvms, _ = get_run_config(program, zkvm, [])

    for program in programs_to_build:
        if is_zkvm_specific(program) or get_path is not None:
            for zkvm in zkvms:
                await _clean(
                    get_program_path(program, zkvm),
                    (
                        get_path(program, zkvm)
                        if get_path
                        else get_program_path(program, zkvm)
                    ),
                )
        else:
            # second argument does not matter
            await _clean(get_program_path(program, "sp1"))


async def _clean(program_dir: str, target_dir: str | None = None):
    if not os.path.isdir(program_dir):
        raise ValueError(f"Error: Program directory {program_dir} does not exist")
    logging.info(f"Cleaning {program_dir}")
    env = {
        **os.environ,
    }
    if target_dir is not None:
        logging.info(f"Setting CARGO_TARGET_DIR to {target_dir}")
        env["CARGO_TARGET_DIR"] = target_dir

    ret = await run_command(
        "cargo clean",
        program_dir,
        env,
        f"clean-{program_dir}",
        timeout=60 * 20,
    )
    if ret != 0:
        raise ValueError(f"Error: Clean failed with code {ret}")
