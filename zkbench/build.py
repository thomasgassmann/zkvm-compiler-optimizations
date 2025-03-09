import logging
import os
import shutil

from zkbench.config import Profile, get_binary_path, get_profile_by_name, get_profiles_ids, get_program_dir_name, get_program_path, get_programs, get_zkvms

def run_build(program: str | None, zkvm: str | None, profile_name: str | None, force: bool):
    programs_to_build = [program] if program else get_programs()
    zkvms = [zkvm] if zkvm else get_zkvms()
    profiles_to_build = [profile_name] if profile_name else get_profiles_ids()

    logging.info(f"Programs to build: {', '.join(programs_to_build)}")
    logging.info(f"zkVMs to build on: {', '.join(zkvms)}")
    logging.info(f"Profiles to build: {', '.join(profiles_to_build)}")

    for zkvm in zkvms:
        for program in programs_to_build:
            for profile_name in profiles_to_build:
                program_dir = get_program_path(program, zkvm)
                if not os.path.isdir(program_dir):
                    raise ValueError(f"Error: Program directory {program_dir} does not exist")

                source = get_binary_path(program, zkvm, None)
                target = get_binary_path(program, zkvm, profile_name)
                if os.path.isfile(target) and not force:
                    logging.info(f"Skipping build as target already exists")
                    continue

                profile = get_profile_by_name(profile_name)
                logging.info(f"Building {program} on {zkvm} with profile {profile_name}")

                os.chdir(program_dir)
                passes = ",".join(profile.passes)
                os.environ["PASSES"] = passes
                if zkvm == "sp1":
                    ret = _build_sp1(profile)
                elif zkvm == "risc0":
                    ret = _build_risc0(profile)
                else:
                    raise ValueError(f"Unknown zkvm: {zkvm}")
                os.chdir("../..")
                if ret != 0:
                    raise ValueError(f"Error: Build failed with code {ret}")

                del os.environ["PASSES"]
                shutil.copyfile(source, target)
                logging.info(f"Copied binary to {target}")

def _build_sp1(profile: Profile):
    passes_string = ",".join(["lower-atomic"] + profile.passes)
    return os.system(
        f"""
        CC=gcc CC_riscv32im_succinct_zkvm_elf=~/.sp1/bin/riscv32-unknown-elf-gcc \
            RUSTFLAGS="-C no-prepopulate-passes -C passes={passes_string} -C link-arg=-Ttext=0x00200800 -C panic=abort {profile.rustflags}" \
            RUSTUP_TOOLCHAIN=succinct \
            CARGO_BUILD_TARGET=riscv32im-succinct-zkvm-elf \
            cargo build --release --locked --features sp1
    """.strip()
    )

def _build_risc0(profile: Profile):
    passes_string = ",".join(["loweratomic"] + profile.passes)
    return os.system(f"""
        CC=gcc CC_riscv32im_risc0_zkvm_elf=~/.risc0/cpp/bin/riscv32-unknown-elf-gcc \
            RUSTFLAGS="-C no-prepopulate-passes -C passes={passes_string} -C link-arg=-Ttext=0x00200800 -C panic=abort {profile.rustflags}" \
            RISC0_FEATURE_bigint2=1 \
            cargo +risc0 build --release --locked \
                --target riscv32im-risc0-zkvm-elf --manifest-path Cargo.toml --features risc0
    """)
