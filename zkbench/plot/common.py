import json
import os


BASELINE = 'baseline'


def get_title(base: str, info: list[str | None]):
    title = base
    if any(map(lambda x: x is not None, info)):
        title += "(" + ", ".join([x for x in info if x is not None]) + ")"
    return title


def read_data(dir: str, program: str, zkvm: str, profile: str, measurement: str):
    path = os.path.join(dir, f"{program}-{zkvm}-{measurement}/{zkvm}-{measurement}", profile, "new/estimates.json")
    return json.load(open(path, 'r'))

def get_mean_ms(dir: str, program: str, zkvm: str, profile: str, measurement: str):
    data = read_data(dir, program, zkvm, profile, measurement)
    return data['mean']['point_estimate'] / 1_000_000
