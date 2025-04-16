from zkbench.config import get_profiles_ids, get_programs, get_zkvms
from zkbench.plot.common import BASELINE, get_title, plot_sorted, read_program_meta


def plot_opt_no_effect(dir: str):
    zkvms = get_zkvms()
    title = get_title("Percentage of optimizations that had no effect", [", ".join(zkvms)])
    programs = get_programs()
    values = []
    for program in programs:
        values.append(
            len(
                [
                    x
                    for x in get_profiles_ids()
                    for zkvm in zkvms
                    if x != BASELINE
                    and read_program_meta(dir, program, zkvm, x)["hash"]
                    == read_program_meta(dir, program, zkvm, BASELINE)["hash"]
                ]
            ) / (len(get_profiles_ids()) * len(zkvms)) * 100
        )

    plot_sorted(
        [values], programs, title, "Number of optimizations with no effect", [None]
    )
