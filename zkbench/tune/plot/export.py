import os
import mdutils
from zkbench.config import get_program_groups_from_programs
from zkbench.plot.export import export_plot
from zkbench.tune.exhaustive import Exhaustive
from zkbench.tune.genetic import Genetic
from zkbench.tune.plot.common import read_exhaustive_stats, read_genetic_stats
from zkbench.tune.plot.exhaustive import plot_exhaustive_depth2
from zkbench.tune.plot.genetic import plot_genetic


def export_genetic(stats_path: str, out: str):
    stats: Genetic = read_genetic_stats(stats_path)
    path = os.path.join(out, "README.md")

    md_file = mdutils.MdUtils(file_name=path)
    md_file.new_header(level=1, title=f"Genetic run for metric {stats.metric}")

    md_file.new_header(level=2, title=f"Programs")
    md_file.new_list(items=stats.programs)
    md_file.new_header(level=2, title=f"zkVMs")
    md_file.new_list(items=stats.zkvms)

    md_file.new_header(level=2, title=f"Best profile")
    md_file.new_list(
        [
            f"Best profile: {stats.best_profile}",
            "Metric: " + str(stats.best_metric),
            "Mode: " + str(stats.mode_name),
            f"Tune config: {stats.config}",
        ]
    )

    md_file.new_header(level=2, title=f"Overview")
    export_plot(
        out,
        None,
        md_file,
        "genetic-plot",
        lambda: plot_genetic(stats_path),
    )

    if len(stats.programs) > 1:
        md_file.new_header(level=2, title=f"Overview by program")
        for program in stats.programs:
            md_file.new_header(level=3, title=f"Program {program}")
            export_plot(
                out,
                None,
                md_file,
                f"genetic-plot-{program}",
                lambda: plot_genetic(stats_path, program, None),
            )

        groups = get_program_groups_from_programs(stats.programs)
        if len(groups) > 1:
            md_file.new_header(level=2, title=f"Overview by program group")
            for group in groups:
                md_file.new_header(level=3, title=f"Group {group}")
                export_plot(
                    out,
                    None,
                    md_file,
                    f"genetic-plot-{group}",
                    lambda: plot_genetic(stats_path, None, None),
                )

    if len(stats.zkvms) > 1:
        md_file.new_header(level=2, title=f"Overview by zkVM")
        for zkvm in stats.zkvms:
            md_file.new_header(level=3, title=f"zkVM {zkvm}")
            export_plot(
                out,
                None,
                md_file,
                f"genetic-plot-{zkvm}",
                lambda: plot_genetic(stats_path, None, zkvm),
            )

    md_file.new_header(level=2, title=f"Baseline values")
    md_file.new_list(
        f"{baseline}: {stats.baselines[baseline]}" for baseline in stats.baselines
    )

    md_file.create_md_file()


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

    groups = get_program_groups_from_programs(stats.programs)
    if len(groups) > 1:
        md_file.new_header(level=2, title=f"Exhaustive for program group")
        for group in groups:
            md_file.new_header(level=3, title=f"Group {group}")
            export_plot(
                out,
                None,
                md_file,
                f"exhaustive-depth2-{group}",
                lambda: plot_exhaustive_depth2(stats_path, None, None, group),
            )

    md_file.create_md_file()
