import json
from dacite import from_dict
from zkbench.tune.exhaustive import Exhaustive


def read_exhaustive_stats(stats: str):
    stats = json.loads(open(stats).read())
    return from_dict(Exhaustive, stats)
