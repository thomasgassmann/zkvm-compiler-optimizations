import logging
import os

from zkbench.config import get_default_profiles_ids, get_programs_by_group
from zkbench.plot.common import get_program_selection


def run_euler(
    program: list[str],
    ignore_program: list[str],
    program_group: list[str],
    zkvm: list[str],
    measurement: list[str],
    profile: list[str],
    profile_time: int | None,
    force: bool,
    meta_only: bool,
    input_override: str = None,
    sample_size: int | None = None,
    sampling_mode: str | None = None,
    runner_path: str | None = None,
    euler_d: int | None = None,
    euler_h: int | None = None,
    euler_log_out: str | None = None,
    euler_criterion_home: str | None = None,
    euler_dry: bool = False,
):
    assert len(measurement) == 1 and measurement[0] == "exec", "Only exec for now"
    assert profile_time is None, "No profiling!"

    measurement = measurement[0]
    all_programs = get_program_selection(program, program_group, ignore=ignore_program)
    for p in all_programs:
        for z in zkvm:
            cmd = f"JOBNAME={p}-{z} SP1_PROVER=cpu ./scripts/euler/run_no_gpu.sh"
            if euler_d is not None:
                cmd = f"TIMED={euler_d} " + cmd
            if euler_h is not None:
                cmd = f"TIMEH={euler_h} " + cmd
            if euler_log_out is not None:
                cmd = f"OUT={euler_log_out} " + cmd
            if euler_criterion_home is not None:
                cmd = f"CRITERION_HOME={euler_criterion_home} " + cmd
            cmd += f" bench --measurement exec --program {p} --zkvm {z}"
            if meta_only:
                cmd += " --meta-only"
            if input_override:
                cmd += f" --input-override {input_override}"
            if sample_size is not None:
                cmd += f" --sample-size {sample_size}"
            if sampling_mode is not None:
                cmd += f" --sampling-mode {sampling_mode}"
            if force:
                cmd += " --force"
            if runner_path:
                cmd += f" --runner-path {runner_path}"
            if profile:
                for pr in profile:
                    cmd += f" --profile {pr}"
            if euler_dry:
                print(cmd)
            else:
                logging.info(f"Running command: {cmd}")
                res = os.system(cmd)
                if res != 0:
                    raise ValueError(f"Error: Euler run failed with code {res}")


def run_bench(
    program: list[str],
    ignore_program: list[str],
    program_group: list[str],
    zkvm: list[str],
    measurement: list[str],
    profile: list[str],
    profile_time: int | None,
    force: bool,
    meta_only: bool,
    input_override: str = None,
    sample_size: int | None = None,
    sampling_mode: str | None = None,
    runner_path: str | None = None,
    euler_d: int | None = None,
    euler_h: int | None = None,
    euler_log_out: str | None = None,
    euler_criterion_home: str | None = None,
    euler_dry: bool = False,
):
    if euler_d or euler_h or euler_log_out or euler_criterion_home or euler_dry:
        run_euler(
            program,
            ignore_program,
            program_group,
            zkvm,
            measurement,
            profile,
            profile_time,
            force,
            meta_only,
            input_override,
            sample_size,
            sampling_mode,
            runner_path,
            euler_d,
            euler_h,
            euler_log_out,
            euler_criterion_home,
            euler_dry,
        )
        return

    args = []
    all_programs = get_program_selection(program, program_group, ignore=ignore_program)
    for p in set(all_programs):
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
    runner_path = runner_path or "./target/release/runner"
    logging.info(f"Running bench with args: {arg_string}")
    res = os.system(
        f"""
        {runner_path} criterion {arg_string}
    """.strip()
    )
    if res != 0:
        raise ValueError(f"Error: Bench failed with code {res}")
