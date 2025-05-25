import numpy as np
from zkbench.config import (
    get_default_profiles_ids,
    get_measurements,
    get_programs,
    get_zkvms,
)
from zkbench.plot.common import get_sample_times_ms, has_data_on


def list_by_stddev(dir: str, threshold: int, measurement: str | None):
    measurements = get_measurements() if measurement is None else [measurement]
    for program in get_programs():
        for zkvm in get_zkvms():
            for measurement in measurements:
                if has_data_on(dir, program, zkvm, measurement):
                    for profile in get_default_profiles_ids():
                        data = get_sample_times_ms(
                            dir, program, zkvm, profile, measurement
                        )
                        variance = np.std(np.array(data))
                        if variance >= threshold:
                            print(
                                f"{program} {zkvm} {profile} {measurement} variance: {variance:.2f}"
                            )
