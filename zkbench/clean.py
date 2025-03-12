import logging
import os
from zkbench.common import get_run_config
from zkbench.config import get_program_path, get_programs, get_zkvms, is_zkvm_specific


def run_clean(program: list[str], zkvm: list[str]):
    programs_to_build, zkvms, _ = get_run_config(program, zkvm, [])

    for program in programs_to_build:
        if is_zkvm_specific(program):
            for zkvm in zkvms:
                _clean(get_program_path(program, zkvm))
        else:
            # second argument does not matter
            _clean(get_program_path(program, 'sp1'))


def _clean(program_dir: str):
    if not os.path.isdir(program_dir):
        raise ValueError(f"Error: Program directory {program_dir} does not exist")
    logging.info(f"Cleaning {program_dir}")
    ret = os.system(f"cargo clean --manifest-path {program_dir}/Cargo.toml")
    if ret != 0:
        raise ValueError(f"Error: Clean failed with code {ret}")
