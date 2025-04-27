import json
from dacite import from_dict
from zkbench.tune.exhaustive import Exhaustive
from zkbench.tune.genetic import Genetic


def read_exhaustive_stats(stats: str):
    stats = json.loads(open(stats).read())
    return from_dict(Exhaustive, stats)


def read_genetic_stats(stats: str):
    stats = json.loads(open(stats).read())
    return from_dict(Genetic, stats)
