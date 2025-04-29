import logging
import os

from zkbench.config import (
    get_measurements,
    get_program_by_name,
    get_program_groups,
    get_programs,
    get_programs_by_group,
    get_zkvms,
)
from mdutils.mdutils import MdUtils

from zkbench.plot.average_duration import plot_average_duration
from zkbench.plot.average_improvement import plot_average_improvement
from zkbench.plot.common import save_path
from zkbench.plot.cycle_count import plot_cycle_count
from zkbench.plot.cycle_count_abs import plot_cycle_count_abs
from zkbench.plot.cycle_count_duration import (
    plot_cycle_count_duration,
    plot_cycle_count_stats,
)
from zkbench.plot.prove_exec import plot_prove_exec


def export_plot(out, subdir, md_file, name, fn):
    file_name = f"{name}.png"
    file_path = (
        os.path.join(out, subdir, file_name) if subdir else os.path.join(out, file_name)
    )
    os.makedirs(os.path.dirname(file_path), exist_ok=True)
    if not os.path.exists(file_path):
        with save_path(file_path):
            fn()

    md_file.new_line(md_file.new_inline_image(path=f"./{file_name}", text=name))


def export_program(dir: str, out: str, program_name: str):
    path = os.path.join(out, "programs", program_name + ".md")
    program = get_program_by_name(program_name)

    md_file = MdUtils(file_name=path)
    md_file.new_header(level=1, title=f"{program_name} report")

    md_file.new_header(level=2, title="Groups")
    group_links = [
        md_file.new_inline_link(f"../program-groups/{group}.md", group)
        for group in program.groups
    ]
    md_file.new_list(group_links)

    md_file.new_header(level=2, title="Speedup by profile")
    export_plot(
        out,
        "programs",
        md_file,
        f"{program_name}-speedup",
        lambda: plot_average_improvement(
            dir,
            zkvm=None,
            program=program_name,
            program_group=None,
            speedup=True,
            global_average=False,
        ),
    )

    md_file.new_header(level=2, title="Cycle count by profile")
    for zkvm in get_zkvms():
        md_file.new_header(level=3, title=f"Cycle count by profile ({zkvm})")
        export_plot(
            out,
            "programs",
            md_file,
            f"{program_name}-{zkvm}-cycle-count",
            lambda: plot_cycle_count_abs(
                dir,
                zkvm=zkvm,
                program=program_name,
            ),
        )

    md_file.new_header(level=3, title="Cycle count relation to measurement")
    for measurement in get_measurements():
        md_file.new_header(level=4, title=f"Cycle count vs. {measurement}")
        export_plot(
            out,
            "programs",
            md_file,
            f"{program_name}-cycle-count-vs-{measurement}",
            lambda: plot_cycle_count_duration(
                dir, measurement=measurement, program=program_name, relative=False
            ),
        )

    md_file.new_header(level=2, title="% faster")
    export_plot(
        out,
        "programs",
        md_file,
        f"{program_name}-improvement",
        lambda: plot_average_improvement(
            dir,
            zkvm=None,
            program=program_name,
            program_group=None,
            speedup=False,
            global_average=False,
        ),
    )

    for zkvm in get_zkvms():
        md_file.new_header(level=3, title=f"{zkvm} % faster")
        export_plot(
            out,
            "programs",
            md_file,
            f"{program_name}-{zkvm}-improvement",
            lambda: plot_average_improvement(
                dir,
                zkvm=zkvm,
                program=program_name,
                program_group=None,
                speedup=False,
                global_average=False,
            ),
        )

    md_file.new_header(level=2, title="Duration")
    for measurement in get_measurements():
        md_file.new_header(level=3, title=f"{measurement} duration")
        export_plot(
            out,
            "programs",
            md_file,
            f"{program_name}-{measurement}-duration",
            lambda: plot_average_duration(
                dir,
                zkvm=None,
                measurement=measurement,
                program=program_name,
            ),
        )

        for zkvm in get_zkvms():
            md_file.new_header(level=4, title=f"{zkvm} {measurement} duration")
            export_plot(
                out,
                "programs",
                md_file,
                f"{program_name}-{zkvm}-{measurement}-duration",
                lambda: plot_average_duration(
                    dir,
                    zkvm=zkvm,
                    measurement=measurement,
                    program=program_name,
                ),
            )

    md_file.new_header(level=2, title="Prove vs. exec duration")
    export_plot(
        out,
        "programs",
        md_file,
        f"{program_name}-prove-vs-exec",
        lambda: plot_prove_exec(dir, program=program_name, program_group=None),
    )

    md_file.create_md_file()


def export_program_group(dir: str, out: str, group_name: str):
    path = os.path.join(out, "program-groups", group_name + ".md")

    md_file = MdUtils(file_name=path)
    md_file.new_header(level=1, title=f"{group_name} report")

    md_file.new_header(level=2, title="Programs")
    program_links = [
        md_file.new_inline_link(f"../programs/{program}.md", program)
        for program in get_programs_by_group(group_name)
    ]
    md_file.new_list(program_links)

    md_file.new_header(level=2, title="Speedup by profile")
    export_plot(
        out,
        "program-groups",
        md_file,
        f"group-{group_name}-speedup",
        lambda: plot_average_improvement(
            dir,
            zkvm=None,
            program=None,
            program_group=group_name,
            speedup=True,
            global_average=False,
        ),
    )

    md_file.new_header(level=2, title="% faster")
    export_plot(
        out,
        "program-groups",
        md_file,
        f"group-{group_name}-improvement",
        lambda: plot_average_improvement(
            dir,
            zkvm=None,
            program=None,
            program_group=group_name,
            speedup=False,
            global_average=False,
        ),
    )

    for zkvm in get_zkvms():
        md_file.new_header(level=3, title=f"{zkvm} % faster")
        export_plot(
            out,
            "program-groups",
            md_file,
            f"group-{group_name}-{zkvm}-improvement",
            lambda: plot_average_improvement(
                dir,
                zkvm=zkvm,
                program=None,
                program_group=group_name,
                speedup=False,
                global_average=False,
            ),
        )

    md_file.new_header(level=2, title="Prove vs. exec duration")
    export_plot(
        out,
        "program-groups",
        md_file,
        f"{group_name}-prove-vs-exec",
        lambda: plot_prove_exec(dir, program_group=group_name, program=None),
    )

    md_file.create_md_file()


def export_program_overview(dir: str, out: str):
    path = os.path.join(out, "programs.md")

    md_file = MdUtils(file_name=path)
    md_file.new_header(level=1, title=f"Program overview")

    md_file.new_header(level=2, title="Programs")
    program_links = [
        md_file.new_inline_link(f"./programs/{program}.md", program)
        for program in get_programs()
    ]
    md_file.new_list(program_links)

    md_file.new_header(level=2, title="Program groups")
    group_links = [
        md_file.new_inline_link(f"./program-groups/{program_group}.md", program_group)
        for program_group in get_program_groups()
    ]
    md_file.new_list(group_links)

    md_file.new_header(level=2, title="% faster by profile")
    export_plot(
        out,
        None,
        md_file,
        f"improvement-average-by-profile",
        lambda: plot_average_improvement(
            dir,
            zkvm=None,
            program=None,
            program_group=None,
            speedup=False,
            global_average=True,
        ),
    )

    md_file.new_header(level=2, title="Cycle count")
    export_plot(
        out,
        None,
        md_file,
        f"cycle-count-by-profile",
        lambda: plot_cycle_count(
            dir,
            program=None,
        ),
    )

    for measurement in get_measurements():
        md_file.new_header(level=3, title=f"Cycle count vs. {measurement}")
        export_plot(
            out,
            None,
            md_file,
            f"cycle-count-vs-{measurement}",
            lambda: plot_cycle_count_duration(
                dir, measurement=measurement, program=None, relative=False
            ),
        )

        md_file.new_header(
            level=3, title=f"Cycle count vs. {measurement} (pearson vs. spearman)"
        )
        export_plot(
            out,
            None,
            md_file,
            f"cycle-count-vs-{measurement}-coefficients",
            lambda: plot_cycle_count_stats(
                dir, measurement=measurement, relative=False
            ),
        )

    md_file.new_header(level=2, title="Prove vs. exec duration")
    export_plot(
        out,
        None,
        md_file,
        "prove-vs-exec",
        # TODO: this behaves differently compared to running it through the cli?
        lambda: plot_prove_exec(
            dir,
            program=None,
            program_group=None,
        ),
    )

    md_file.create_md_file()


def export_report(dir: str, out: str):
    os.makedirs(out, exist_ok=True)

    for program_name in get_programs():
        try:
            export_program(dir, out, program_name)
        except Exception as e:
            logging.error(f"Program export failed for {program_name}: {e}")
    export_program_overview(dir, out)

    for group_name in get_program_groups():
        try:
            export_program_group(dir, out, group_name)
        except Exception as e:
            logging.error(f"Group export failed for {group_name}: {e}")
