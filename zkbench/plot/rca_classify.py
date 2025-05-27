import numpy as np
from zkbench.config import get_default_profiles_ids, get_programs, get_zkvms
from zkbench.plot.common import BASELINE, get_point_estimate_median_ms


def find_improvement(dir: str, program: str, zkvm: str, profile: str, measurement: str):
    baseline = get_point_estimate_median_ms(dir, program, zkvm, BASELINE, measurement)
    current = get_point_estimate_median_ms(dir, program, zkvm, profile, measurement)
    return (baseline - current) / baseline


def find_degradation(dir: str, program: str, zkvm: str, profile: str, measurement: str):
    baseline = get_point_estimate_median_ms(dir, program, zkvm, BASELINE, measurement)
    current = get_point_estimate_median_ms(dir, program, zkvm, profile, measurement)
    return (current - baseline) / baseline


def find_exec_prove_discrepancy(dir: str, program: str, zkvm: str, profile: str):
    baseline_exec = get_point_estimate_median_ms(dir, program, zkvm, BASELINE, 'exec')
    current_exec = get_point_estimate_median_ms(dir, program, zkvm, profile, 'exec')
    baseline_prove = get_point_estimate_median_ms(dir, program, zkvm, BASELINE, 'prove')
    current_prove = get_point_estimate_median_ms(dir, program, zkvm, profile, 'prove')

    exec_value = (baseline_exec - current_exec) / baseline_exec
    prove_value = (baseline_prove - current_prove) / baseline_prove
    if exec_value * prove_value < 0:
        return abs(exec_value - prove_value)
    return 0



def find_by_strategy(
    dir: str,
    program: str,
    zkvm: str,
    profile: str,
    strategy: str,
    measurement: str | None
):
    if strategy == "improve":
        return find_improvement(dir, program, zkvm, profile, measurement)
    elif strategy == "degrade":
        return find_degradation(dir, program, zkvm, profile, measurement)
    elif strategy == "exec_prove":
        return find_exec_prove_discrepancy(dir, program, zkvm, profile)
    else:
        raise ValueError(f"Unknown strategy: {strategy}")


def classify_rca(
    dir: str,
    threshold: float,
    avg_programs: bool,
    strategy: str,
    measurement: str | None,
    zkvm: str | None = None,
):
    res = []

    profiles = get_default_profiles_ids()
    profiles.remove(BASELINE)

    # remove uninteresting cases
    profiles.remove("o3")
    profiles.remove("o2")
    profiles.remove("o1")
    profiles.remove("o0")
    profiles.remove("oz")
    profiles.remove("os")

    zkvms = get_zkvms() if zkvm is None else [zkvm]
    programs = get_programs()

    for zkvm in zkvms:
        for profile in profiles:
            program_values = []
            for program in programs:
                res = find_by_strategy(
                    dir, program, zkvm, profile, strategy, measurement
                )
                program_values.append(res)
            if avg_programs:
                res = np.average(program_values, axis=0)
                if res >= threshold:
                    print(f"{zkvm}-{profile}-{strategy}: {res} (avg)")
            else:
                for program, value in zip(programs, program_values):
                    if value >= threshold:
                        print(f"{zkvm}-{profile}-{program}-{strategy}: {value}")
