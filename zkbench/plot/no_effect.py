import logging
from zkbench.config import (
    get_default_profiles_ids,
    get_program_by_name,
    get_programs,
    get_zkvms_with_x86,
)
from zkbench.plot.common import (
    BASELINE,
    get_program_selection,
    get_title,
    plot_sorted,
    read_program_meta,
)


def plot_no_effect(
    dir: str,
    zkvm: str | None = None,
    program_group: str | None = None,
    program: str | None = None,
):
    zkvms = get_zkvms_with_x86() if zkvm is None else [zkvm]
    programs = (
        get_program_selection(program, program_group)
        if program_group or program
        else get_programs()
    )
    title = get_title(
        "Percentage of programs where optimization had no effect", [", ".join(zkvms)]
    )
    profiles = []
    values = []
    logging.info(
        f"Number of default profiles: {len(get_default_profiles_ids())}"
    )
    for profile in get_default_profiles_ids():
        if profile == BASELINE:
            continue

        try:
            current = (
                len(
                    [
                        x
                        for x in programs
                        for zkvm in zkvms
                        if x != BASELINE
                        and (
                            profile in get_program_by_name(x).skip
                            or read_program_meta(dir, x, zkvm, profile)["hash"]
                            == read_program_meta(dir, x, zkvm, BASELINE)["hash"]
                        )
                    ]
                )
                / (len(zkvms) * len(programs))
                * 100
            )
            if current == 100:
                logging.warning(
                    f"Profile {profile} has no effect on all programs for zkvm {zkvm}"
                )
            elif current >= 90:
                logging.warning(
                    f"Profile {profile} has no effect on more than 90% of all programs for zkvm {zkvm}"
                )
            values.append(current)
            profiles.append(profile)
        except FileNotFoundError:
            logging.warning(f"Data for profile {profile} not found")
            continue

    plot_sorted(
        [values],
        profiles,
        title,
        "Percentage of programs where optimization had no effect",
        [None],
    )
