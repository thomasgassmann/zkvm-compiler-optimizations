import os


def run_bench(program: str | None, zkvm: str | None, measurement: str | None):
    args = []
    if program:
        args.append(f"--program {program}")
    if zkvm:
        args.append(f"--zkvm {zkvm}")
    if measurement:
        args.append(f"--measurement {measurement}")

    res = os.system(
        f"""
        cargo run --release -p runner -- {' '.join(args)}
    """.strip()
    )
    if res != 0:
        raise ValueError(f"Error: Bench failed with code {res}")
