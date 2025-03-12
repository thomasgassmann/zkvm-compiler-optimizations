import asyncio
import logging
import os
import shutil

from zkbench.common import get_run_config
from zkbench.config import (
    get_binary_path,
    get_profile_by_name,
    get_program_path,
)


async def run_build(
    programs: list[str],
    zkvms: list[str],
    profile_names: list[str],
    force: bool,
    j: int,
):
    programs_to_build, zkvms, profiles_to_build = get_run_config(
        programs, zkvms, profile_names
    )

    logging.info(f"Programs to build: {', '.join(programs_to_build)}")
    logging.info(f"zkVMs to build on: {', '.join(zkvms)}")
    logging.info(f"Profiles to build: {', '.join(profiles_to_build)}")

    build_jobs: dict[str, list[str]] = dict()
    for program in programs_to_build:
        build_jobs[program] = []
        for zkvm in zkvms:
            for profile_name in profiles_to_build:
                program_dir = get_program_path(program, zkvm)
                if not os.path.isdir(program_dir):
                    raise ValueError(f"Error: Program directory {program_dir} does not exist")

                target = get_binary_path(program, zkvm, profile_name)
                if os.path.isfile(target) and not force:
                    logging.info(f"Skipping build as target already exists")
                    continue

                build_jobs[program].append((profile_name, zkvm))

    while any([len(build_jobs[program]) > 0 for program in build_jobs.keys()]):
        number_of_jobs = min(
            j, len([job for job in build_jobs.keys() if len(build_jobs[job]) > 0])
        )
        jobs_to_run = []
        for program in build_jobs.keys():
            if len(build_jobs[program]) > 0:
                profile_name, zkvm = build_jobs[program].pop(0)
                jobs_to_run.append((program, profile_name, zkvm))
                if len(jobs_to_run) == number_of_jobs:
                    break

        logging.info(
            "Running build jobs: {}".format(
                ", ".join([f"{p}-{profile}-{zk}" for p, profile, zk in jobs_to_run])
            )
        )
        await asyncio.gather(
            *[
                _build(program, profile_name, zkvm)
                for program, profile_name, zkvm in jobs_to_run
            ]
        )


async def _build(program: str, profile_name: str, zkvm: str):
    source = get_binary_path(program, zkvm, None)
    target = get_binary_path(program, zkvm, profile_name)
    name = "{}-{}-{}".format(program, zkvm, profile_name)

    profile = get_profile_by_name(profile_name)
    logging.info(f"Building {program} on {zkvm} with profile {profile_name}")

    passes = ",".join(profile.passes)
    env = {
        **os.environ,
        "PASSES": passes,
    }
    passes_string = ",".join(
        ["loweratomic" if zkvm == "risc0" else "lower-atomic"] + profile.passes
    )
    pass_string = "" if passes_string == "" else f"-C passes={passes_string}"
    prepopulate_passes = (
        "" if profile.prepopulate_passes else "-C no-prepopulate-passes"
    )
    program_dir = get_program_path(program, zkvm)
    if zkvm == "sp1":
        ret = await _run_command(
            f"""
            CC=gcc CC_riscv32im_succinct_zkvm_elf=~/.sp1/bin/riscv32-unknown-elf-gcc \
                RUSTFLAGS="{prepopulate_passes} {pass_string} -C link-arg=-Ttext=0x00200800 -C panic=abort {profile.rustflags}" \
                RUSTUP_TOOLCHAIN=succinct \
                CARGO_BUILD_TARGET=riscv32im-succinct-zkvm-elf \
                cargo build --release --locked --features sp1
        """.strip(),
            program_dir,
            env,
            name,
        )
    elif zkvm == "risc0":
        ret = await _run_command(
            f"""
            CC=gcc CC_riscv32im_risc0_zkvm_elf=~/.risc0/cpp/bin/riscv32-unknown-elf-gcc \
                RUSTFLAGS="{prepopulate_passes} {pass_string} -C link-arg=-Ttext=0x00200800 -C panic=abort {profile.rustflags}" \
                RISC0_FEATURE_bigint2=1 \
                cargo +risc0 build --release --locked \
                    --target riscv32im-risc0-zkvm-elf --manifest-path Cargo.toml --features risc0
        """.strip(),
            program_dir,
            env,
            name,
        )
    else:
        raise ValueError(f"Unknown zkvm: {zkvm}")
    if ret != 0:
        raise ValueError(f"Error: Build failed with code {ret}")

    shutil.copyfile(source, target)
    logging.info(f"Copied binary to {target}")


async def _run_command(cmd, cwd, env, task_name):
    logging.debug(f"[{task_name}] Running command: {cmd}")
    process = await asyncio.create_subprocess_shell(
        cmd,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE,
        cwd=cwd,
        env=env,
    )

    async def stream_output(stream, name):
        while True:
            line = await stream.readline()
            if line:
                logging.debug(f"[{task_name}, {name}] {line.decode().rstrip()}")
            else:
                break

    await asyncio.gather(
        stream_output(process.stdout, "stdout"),
        stream_output(process.stderr, "stderr"),
    )

    return await process.wait()
