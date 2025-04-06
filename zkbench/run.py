import os


def run_single(program: str, zkvm: str, elf: str, force: bool):
    res = os.system(
        f"""
        ./target/release/runner run --program {program} --zkvm {zkvm} --elf {elf}
    """.strip()
    )
    if res != 0:
        raise ValueError(f"Error: Run failed with code {res}")
