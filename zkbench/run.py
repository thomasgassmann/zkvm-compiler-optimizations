import os

def run_single(
    program: str,
    zkvm: str,
    elf: str,
):
    res = os.system(
        f"""
        RUSTFLAGS="-C target-cpu=native" cargo run --release -p runner -- run --program {program} --zkvm {zkvm} --elf {elf}
    """.strip()
    )
    if res != 0:
        raise ValueError(f"Error: Run failed with code {res}")
