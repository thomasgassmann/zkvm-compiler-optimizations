from zkbench.config import get_profiles_ids
from zkbench.plot.common import get_average_across, get_mean_ms, get_title, plot_sorted


def plot_average_duration(dir: str, zkvm: str | None, measurement: str | None, program: str | None):
    title = get_title("Average duration by profile", [zkvm, measurement, program])

    profiles = get_profiles_ids()
    relative_improvements = get_average_across(
        dir, zkvm, measurement, program, profiles, lambda dir, program, zkvm, profile, measurement: get_mean_ms(dir, program, zkvm, profile, measurement)
    )

    plot_sorted(relative_improvements, profiles, title, 'average duration (ms)')
