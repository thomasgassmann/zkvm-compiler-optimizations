import logging
import os
import subprocess
import sys
import tempfile
from zkbench.build import get_build_command
from zkbench.config import get_profile_by_name, get_program_path


def show_asm(
    program: str,
    zkvm: str,
    profile_name: str,
    features: list[str],
    llvm: bool,
    rust: bool,
    rest: str,
    do_open: bool,
    subtractive: bool = False,
):
    profile = get_profile_by_name(profile_name)
    cmd, env = get_build_command(
        zkvm, profile, False, False, None, features, "asm", subtractive=subtractive
    )
    if llvm:
        cmd += " --llvm"
    if rust:
        cmd += " --rust"

    if zkvm == "x86":
        pass # should have already been added
    elif zkvm == "risc0" or zkvm == "sp1":
        cmd += f" --bin {program}"
    else:
        raise ValueError(f"Unsupported zkvm: {zkvm}")

    cmd += f" --context 3 {rest}"

    logging.debug(f"Running command: {cmd}")

    program_dir = get_program_path(program, zkvm)
    if not do_open:
        res = subprocess.run(cmd, env=env, shell=True, check=False, cwd=program_dir)
        if res.returncode != 0:
            sys.exit(res.returncode)
    else:
        res = subprocess.run(
            cmd, env=env, shell=True, check=False, cwd=program_dir, stdout=subprocess.PIPE, stderr=subprocess.PIPE
        )
        if res.returncode != 0:
            print(f"Error running command: {cmd}", file=sys.stderr)
            sys.exit(res.returncode)
        if res.stdout:
            temp_file = tempfile.mktemp(suffix=".asm")
            with open(temp_file, "wb") as f:
                f.write(res.stdout)
            os.system(f"code {temp_file}")
        if res.stderr:
            print(res.stderr.decode("utf-8"), file=sys.stderr)
