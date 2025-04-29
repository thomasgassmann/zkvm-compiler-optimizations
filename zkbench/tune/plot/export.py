import os
import mdutils
from zkbench.plot.export import export_plot
from zkbench.tune.exhaustive import Exhaustive
from zkbench.tune.plot.common import read_exhaustive_stats
from zkbench.tune.plot.exhaustive import plot_exhaustive_depth2


def export_exhaustive_depth2(stats_path: str, out: str):
    stats: Exhaustive = read_exhaustive_stats(stats_path)
    path = os.path.join(out, "README.md")

    md_file = mdutils.MdUtils(file_name=path)
    md_file.new_header(level=1, title=f"Exhaustive for metric {stats.metric}")

    md_file.new_header(level=2, title=f"Overview")
    export_plot(
        out,
        None,
        md_file,
        "exhaustive-depth2-main",
        lambda: plot_exhaustive_depth2(stats_path, None, None),
    )

    for zkvm in stats.zkvms:
        md_file.new_header(level=2, title=f"Exhaustive for zkvm {zkvm}")
        export_plot(
            out,
            None,
            md_file,
            f"exhaustive-depth2-{zkvm}",
            lambda: plot_exhaustive_depth2(stats_path, None, zkvm),
        )

    for program in stats.programs:
        md_file.new_header(level=2, title=f"Exhaustive for program {program}")
        export_plot(
            out,
            None,
            md_file,
            f"exhaustive-depth2-{program}",
            lambda: plot_exhaustive_depth2(stats_path, program, None),
        )

    md_file.create_md_file()
