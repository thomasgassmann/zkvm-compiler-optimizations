import asyncio
import logging
import os
import shutil
import threading

from zkbench.clean import run_clean
from zkbench.common import get_run_config, run_command
from zkbench.config import (
    Profile,
    get_source_binary_path,
    get_profile_by_name,
    get_program_path,
    get_target_binary_path,
)


async def run_build(
    programs: list[str],
    ignore_program: list[str],
    program_groups: list[str],
    zkvms: list[str],
    profile_names: list[str],
    force: bool,
    j: int,
    llvm: bool,
    features: list[str] | None = None,
    name: str | None = None,
    build_by_program: bool = False,
    clean_after_build: bool = False,
    subtractive: bool = False,
):
    programs_to_build, zkvms, profiles_to_build = get_run_config(
        programs, zkvms, profile_names, program_groups, ignore=ignore_program
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
                    raise ValueError(
                        f"Error: Program directory {program_dir} does not exist"
                    )

                target = get_target_binary_path(program, zkvm, profile_name if name is None else name)
                if os.path.isfile(target) and not force:
                    logging.info(f"Skipping build as target already exists")
                    continue

                build_jobs[program].append((profile_name, zkvm))

    if build_by_program:
        all_remaining = sum([len(jobs) for jobs in build_jobs.values()])
        all_total = all_remaining
        sem = asyncio.Semaphore(j)

        async def _build_all(p: str):
            nonlocal all_remaining
            all_zkvms = sorted(set([zkvm for _, zkvm in build_jobs[p]]))
            remaining = len(build_jobs[p])
            total = remaining
            for cz in all_zkvms:
                profiles = sorted(
                    set([profile_name for profile_name, z in build_jobs[p] if z == cz])
                )
                for profile_name in profiles:
                    logging.info(
                        f"Build jobs {p}: {remaining}/{total} ({all_remaining}/{all_total})"
                    )
                    async with sem:
                        await _build(
                            p,
                            profile_name,
                            cz,
                            llvm,
                            features=features,
                            name=name,
                            subtractive=subtractive,
                        )
                    remaining -= 1
                    all_remaining -= 1
                if clean_after_build:
                    run_clean([p], [cz])

        await asyncio.gather(*[_build_all(p) for p in build_jobs.keys()])
    else:
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
                    _build(
                        program,
                        profile_name,
                        zkvm,
                        llvm,
                        features=features,
                        name=name,
                        subtractive=subtractive,
                    )
                    for program, profile_name, zkvm in jobs_to_run
                ]
            )
        # TODO: clean after build


async def _build(
    program: str,
    profile_name: str,
    zkvm: str,
    llvm: bool,
    features=None,
    name=None,
    subtractive: bool = False,
):
    target = get_target_binary_path(
        program, zkvm, profile_name if name is None else name
    )
    os.makedirs(os.path.dirname(target), exist_ok=True)

    profile = get_profile_by_name(profile_name)
    await build_program(
        program, zkvm, profile, llvm, target, features=features, subtractive=subtractive
    )


def get_build_command(
    zkvm: str,
    profile: Profile,
    llvm: bool,
    verbose: bool,
    target_dir: str | None,
    features: list[str] | None,
    cmd: str,
    subtractive: bool = False,
):
    passes = ",".join(profile.passes)
    env = {
        **os.environ,
        "CARGO_ZK_PASSES": passes,
        "CARGO_ZK_CFLAGS": profile.cflags,
        "CARGO_ZK_LOWER_ATOMIC_BEFORE": str(profile.lower_atomic_before),
        "THREAD_ID": str(threading.get_ident()),
    }
    if target_dir is not None:
        env["CARGO_TARGET_DIR"] = target_dir

    verbosity = "--verbose" if verbose else ""
    additional_features = (
        "" if features is None else " ".join([f"--features {f}" for f in features])
    )

    lower_atomic_pass = ["lower-atomic"]
    passes_string = ",".join(
        (profile.passes + lower_atomic_pass)
        if not profile.lower_atomic_before
        else (lower_atomic_pass + profile.passes)
    )
    pass_string = "" if passes_string == "" else f"-C passes={passes_string}"
    if subtractive:
        pass_string = f"-C passes=" + ",".join(lower_atomic_pass)

    prepopulate_passes = (
        "" if profile.prepopulate_passes or subtractive else "-C no-prepopulate-passes"
    )
    rflags = profile.rustflags if not subtractive else profile.subtractive
    llvm_flag = "--emit=llvm-ir" if llvm else ""
    # setting CC below uses gcc for dependencies with c code, ideally we should use clang
    # to apply the same optimization passes, this currently only seems to affect rsp-risc0
    if zkvm == "sp1":
        return (
            f"""
            CC=gcc CC_riscv32im_succinct_zkvm_elf=~/.sp1/bin/riscv32-unknown-elf-gcc \
                RUSTFLAGS="{prepopulate_passes} {pass_string} -C link-arg=-Ttext=0x00200800 -C panic=abort {rflags} {llvm_flag}" \
                RUSTUP_TOOLCHAIN=succinct \
                CARGO_BUILD_TARGET=riscv32im-succinct-zkvm-elf \
                cargo +succinct {cmd} --release --locked --features sp1 {verbosity} {additional_features}
        """.strip(),
            env,
        )
    elif zkvm == "risc0":
        return (
            f"""
            CC=gcc CC_riscv32im_risc0_zkvm_elf=~/.risc0/cpp/bin/riscv32-unknown-elf-gcc \
                RUSTFLAGS="{prepopulate_passes} {pass_string} -C link-arg=-Ttext=0x00200800 -C panic=abort {rflags} {llvm_flag}" \
                RISC0_FEATURE_bigint2=1 \
                cargo +risc0 {cmd} --release --locked --features risc0 \
                    --target riscv32im-risc0-zkvm-elf {verbosity} {additional_features}
        """.strip(),
            env,
        )
    elif zkvm == "x86":
        return (
            f"""
            RUSTFLAGS="{prepopulate_passes} {pass_string} -C panic=abort {rflags} {llvm_flag}" \
                cargo +nightly-2025-01-30-x86_64-unknown-linux-gnu {cmd} --release --locked --features x86 --lib {verbosity} {additional_features}
        """.strip(),
            env,
        )
    else:
        raise ValueError(f"Unknown zkvm: {zkvm}")


async def build_program(
    program: str,
    zkvm: str,
    profile: Profile,
    llvm: bool,
    target: str,
    verbose: bool = False,
    timeout=None,
    target_dir=None,
    features=None,
    subtractive: bool = False,
):
    source = get_source_binary_path(program, zkvm, target_dir)
    profile_name = profile.profile_name
    name = f"{program}-{zkvm}-{profile_name}"
    logging.info(f"Building {program} on {zkvm} with profile {profile_name}")

    program_dir = get_program_path(program, zkvm)
    # setting CC below uses gcc for dependencies with c code, ideally we should use clang
    # to apply the same optimization passes, this currently only seems to affect rsp-risc0
    cmd, env = get_build_command(
        zkvm,
        profile,
        llvm,
        verbose,
        target_dir,
        features,
        "build",
        subtractive=subtractive,
    )
    ret = await run_command(
        cmd,
        program_dir,
        env,
        name,
        timeout=timeout,
    )
    if ret != 0:
        logging.error(
            f"{program}-{zkvm}-{profile_name}: Build failed with code {ret}, passes: {profile.passes}"
        )
        raise ValueError(f"Error: Build failed with code {ret}")

    if not llvm:
        temp_target = f"{target}.tmp"
        shutil.copyfile(source, temp_target)
        os.replace(temp_target, target)
        logging.info(f"Copied binary to {target}")
