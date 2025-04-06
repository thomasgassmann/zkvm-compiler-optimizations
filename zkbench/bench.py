import logging
import os


def run_bench(
    program: list[str],
    zkvm: list[str],
    measurement: list[str],
    profile: list[str],
    profile_time: int,
    force: bool,
    meta_only: bool,
):
    args = []
    if program:
        for p in program:
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
    if profile_time:
        args.append(f"--profile-time {profile_time}")
    if force:
        args.append("--force")
    if meta_only:
        args.append("--meta-only")

    arg_string = " ".join(args)
    logging.info(f"Running bench with args: {arg_string}")
    res = os.system(
        f"""
        ./target/release/runner criterion {arg_string}
    """.strip()
    )
    if res != 0:
        raise ValueError(f"Error: Bench failed with code {res}")
