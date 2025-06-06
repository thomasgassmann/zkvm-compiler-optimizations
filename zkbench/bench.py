import logging
import os

from zkbench.config import get_default_profiles_ids, get_programs_by_group


def run_bench(
    program: list[str],
    program_group: list[str],
    zkvm: list[str],
    measurement: list[str],
    profile: list[str],
    profile_time: int,
    force: bool,
    meta_only: bool,
    input_override: str = None,
    sample_size: int | None = None,
    sampling_mode: str | None = None,
):
    args = []
    if program or program_group:
        programs = [] if not program else program
        if program_group:
            for group in program_group:
                programs.extend(get_programs_by_group(group))

        for p in set(programs):
            args.append(f"--program {p}")
    if zkvm:
        for z in zkvm:
            args.append(f"--zkvm {z}")
    if measurement:
        for m in measurement:
            args.append(f"--measurement {m}")
    if profile:
        for p in profile:
            args.append(f"--profile {p}")
    else:
        for p in get_default_profiles_ids():
            args.append(f"--profile {p}")
    if profile_time:
        args.append(f"--profile-time {profile_time}")
    if force:
        args.append("--force")
    if meta_only:
        args.append("--meta-only")
    if input_override:
        args.append(f"--input-override {input_override}")
    if sample_size is not None:
        args.append(f"--sample-size {sample_size}")
    if sampling_mode is not None:
        args.append(f"--sampling-mode {sampling_mode}")

    arg_string = " ".join(args)
    logging.info(f"Running bench with args: {arg_string}")
    res = os.system(
        f"""
        ./target/release/runner criterion {arg_string}
    """.strip()
    )
    if res != 0:
        raise ValueError(f"Error: Bench failed with code {res}")
