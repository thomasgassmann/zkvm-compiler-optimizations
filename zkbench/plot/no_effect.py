from zkbench.config import get_profiles_ids, get_programs, get_zkvms
from zkbench.plot.common import BASELINE, get_title, plot_sorted, read_program_meta


def plot_no_effect(dir: str):
    zkvms = get_zkvms()
    title = get_title("Percentage of programs where optimization had no effect", [", ".join(zkvms)])
    profiles = get_profiles_ids()
    profiles.remove(BASELINE)
    values = []
    for profile in profiles:
        values.append(
            len(
                [
                    x
                    for x in get_programs()
                    for zkvm in zkvms
                    if x != BASELINE
                    and read_program_meta(dir, x, zkvm, profile)["hash"]
                    == read_program_meta(dir, x, zkvm, BASELINE)["hash"]
                ]
            ) / (len(zkvms) * len(get_programs())) * 100
        )

    plot_sorted(
        [values],
        profiles,
        title,
        "Percentage of programs where optimization had no effect",
        [None],
    )
