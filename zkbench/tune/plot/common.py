import json
from dacite import from_dict
from zkbench.tune.common import MetricValue
from zkbench.tune.exhaustive import Exhaustive
from zkbench.tune.ffd import FFDRun
from zkbench.tune.genetic import Genetic


def read_exhaustive_stats(stats: str):
    stats = json.loads(open(stats).read())
    return from_dict(Exhaustive, stats)


def read_genetic_stats(stats: str):
    stats = json.loads(open(stats).read())
    return from_dict(Genetic, stats)


def read_ffd_stats(stats: str):
    stats = json.loads(open(stats).read())
    return from_dict(FFDRun, stats)


def get_metric_sum(
    l: list[MetricValue], program_list: list[str] | None, zkvm: str | None
) -> float:
    return sum(
        [
            v.metric
            for v in l
            if (program_list is None or v.program in program_list)
            and (v.zkvm == zkvm or zkvm is None)
            and not v.timeout
        ]
    )
