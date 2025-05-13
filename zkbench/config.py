import dataclasses
import hashlib
import json
from dataclasses import dataclass
import os
from typing import List


@dataclass(frozen=True)
class Profile:
    profile_name: str
    rustflags: str
    cflags: str
    passes: List[str]
    prepopulate_passes: bool
    lower_atomic_before: bool = False

    def __getattribute__(self, name):
        if name == "name":
            return self.profile_name
        return super().__getattribute__(name)

    def get_unique_id(self, zkvm: str, program: str) -> str:
        return f"{self.name}-{zkvm}-{program}-{self.get_hash()[:8]}"

    def get_hash(self):
        values = dataclasses.asdict(self)
        encoded_string = json.dumps(values).encode("utf-8")
        return hashlib.sha256(encoded_string).hexdigest()


@dataclass
class Program:
    program_name: str
    specific: bool | None
    groups: list[str]
    skip: list[str] | None


CONFIG = json.load(open("config.json", "r"))


def get_profile_by_name(profile_name: str) -> Profile:
    return Profile(
        profile_name,
        CONFIG["profiles"][profile_name]["rustflags"],
        CONFIG["profiles"][profile_name]["cflags"],
        CONFIG["profiles"][profile_name]["passes"],
        CONFIG["profiles"][profile_name]["prepopulate_passes"],
        CONFIG["profiles"][profile_name].get("lower_atomic_before", False),
    )


def get_default_profiles_ids() -> List[str]:
    return [
        profile_name
        for profile_name in get_profiles_ids()
        if not CONFIG["profiles"][profile_name].get("no_default", False)
    ]


def get_profiles() -> List[Profile]:
    return [get_profile_by_name(profile_name) for profile_name in get_profiles_ids()]


def get_programs():
    return list(CONFIG["programs"].keys())


def get_program_by_name(program_name: str) -> Program:
    program = CONFIG["programs"][program_name]
    return Program(
        program_name,
        program.get("specific", None),
        program.get("groups", []),
        program.get("skip", []),
    )


def get_program_groups_from_programs(programs: List[str]) -> set[str]:
    groups = set()
    for program_id in programs:
        program = get_program_by_name(program_id)
        groups.update(program.groups)
    return groups


def get_program_groups() -> set[str]:
    groups = set()
    for program_id in get_programs():
        program = get_program_by_name(program_id)
        groups.update(program.groups)
    return groups


def get_programs_by_group(group: str) -> List[str]:
    programs = []
    for program_id in get_programs():
        program = get_program_by_name(program_id)
        if group in program.groups:
            programs.append(program_id)
    return programs


def get_measurements():
    return CONFIG["measurements"]


def get_zkvms():
    return CONFIG["zkvms"]


def get_zkvm_specific_programs():
    return [
        program
        for program in get_programs()
        if CONFIG["programs"][program].get("specific", False)
    ]


def is_zkvm_specific(program_id: str):
    return program_id in get_zkvm_specific_programs()


def get_profiles_ids() -> List[str]:
    return list(CONFIG["profiles"].keys())


def get_program_dir_name(program_id: str, zkvm: str) -> str:
    return (
        program_id
        if program_id not in get_zkvm_specific_programs()
        else f"{program_id}-{zkvm}"
    )


def get_program_path(program_id: str, zkvm: str) -> str:
    return f"./programs/{get_program_dir_name(program_id, zkvm)}"


def get_source_binary_path(
    program_id: str, zkvm: str, target_dir: str | None = None
) -> str:
    dir_name = get_program_dir_name(program_id, zkvm)
    base = f"./programs/{dir_name}/target" if not target_dir else target_dir
    if zkvm == "sp1":
        path = os.path.join(base, "riscv32im-succinct-zkvm-elf/release", dir_name)
    elif zkvm == "risc0":
        path = os.path.join(base, "riscv32im-risc0-zkvm-elf/release", dir_name)
    else:
        raise ValueError(f"Unknown zkvm: {zkvm}")
    return path


def get_target_binary_path(program_id: str, zkvm: str, profile: str):
    return f"./bin/{program_id}/{zkvm}/{profile}"
