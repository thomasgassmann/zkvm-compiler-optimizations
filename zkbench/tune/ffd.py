import asyncio
from dataclasses import dataclass
import dataclasses
import json
import logging
import os

from dacite import from_dict

from zkbench.tune.runner import BuildResult, TuneRunner
from zkbench.tune.common import (
    BIN_OUT_FFD,
    EvalResult,
    MetricValue,
    ProfileConfig,
    TuneConfig,
    build_pass_list,
)
from pyDOE3 import fracfact_by_res
from itertools import compress
import random


def read_ffd_stats(stats: str):
    stats = json.loads(open(stats).read())
    return from_dict(FFDRun, stats)


@dataclass(frozen=True)
class FFDResult:
    row_index: int
    passes: list[str]
    active_factors: list[str]
    profile_config: ProfileConfig
    build_error: bool
    eval_result: EvalResult | None

@dataclass(frozen=True)
class FFDRun:
    results: list[FFDResult]
    metric: str
    programs: list[str]
    zkvms: list[str]
    config: TuneConfig
    factors: list[str]
    resolution: int
    design: list


def run_tune_ffd(
    programs: list[str],
    zkvms: list[str],
    metric: str,
    config: TuneConfig,
    out: str,
    resolution: int,
):
    assert len(config.allowed_opt_levels) == 1, "FFD tuning only supports a single optimization level"

    os.makedirs(out, exist_ok=True)

    pass_factors = (
        config.module_passes + config.function_passes + config.loop_passes
    )
    extra = []
    if config.tune_lto:
        extra.append("lto")
    if config.tune_codegen_units:
        extra.append("single_codegen_unit")
    if config.tune_prepopulate_passes:
        extra.append("prepopulate_passes")

    factors = pass_factors + extra

    design = fracfact_by_res(len(factors), resolution)

    runner = TuneRunner(
        BIN_OUT_FFD,
        metric,
        out,
        build_timeout=60 * 30,
        rebuild_failed=True,
        retry_build=False,
    )
    results: list[FFDResult] = []

    stats_path = os.path.join(out, "stats.json")

    def flush():
        with open(stats_path, "w") as f:
            json.dump(
                dataclasses.asdict(
                    FFDRun(
                        results,
                        metric,
                        programs,
                        zkvms,
                        config,
                        factors,
                        resolution,
                        design.tolist(),
                    )
                ),
                f,
                indent=2,
            )

    existing = read_ffd_stats(stats_path) if os.path.exists(stats_path) else None

    for idx, row in enumerate(design):
        active = list(compress(factors, row == 1))
        active_passes = [p for p in pass_factors if p in active]
        scgu = ("single_codegen_unit" in active) if config.tune_codegen_units else config.default_single_codegen_unit
        pp = ("prepopulate_passes" in active) if config.tune_prepopulate_passes else config.default_prepopulate_passes

        if existing is not None and idx < len(existing.results):
            logging.warning("Skipping row %d/%d, already exists", idx + 1, len(design))
            existing_row = existing.results[idx]

            assert existing_row.build_error is False
            assert existing_row.eval_result is not None
            assert existing_row.eval_result.has_error is False
            assert set(existing_row.active_factors) == set(active)
            assert set(programs) <= set(existing.programs)

            existing_row.eval_result.values = [
                v for v in existing_row.eval_result.values if v.program in programs
            ]

            assert set(existing.zkvms) == set(zkvms)
            assert len(existing_row.eval_result.values) == len(programs) * len(zkvms)
            assert all(
                [
                    not p.timeout and p.metric != -1
                    for p in existing_row.eval_result.values
                ]
            )

            results.append(existing_row)
            continue

        # we cannot get any info about ordering anyways
        # some sequences of passes seem to cause a SEGFAULT in
        # the rust compiler
        random.seed(42)
        remaining = None
        eval_result = EvalResult(False, values=[])
        while remaining is None or len(remaining) > 0:
            profile = ProfileConfig(
                name=f"ffd_row_{idx}",
                lto="fat" if "lto" in active else "off",
                passes=[build_pass_list(active_passes)],
                single_codegen_unit=scgu,
                opt_level=config.allowed_opt_levels[0],
                prepopulate_passes=pp,
            )

            logging.info(
                "Row %d/%d: %s - %s", idx + 1, len(design), profile.get_hash(), profile
            )
            if remaining is None:
                res = asyncio.get_event_loop().run_until_complete(
                    runner.run_build(programs, zkvms, profile)
                )
            else:
                remaining_zkvms = set([b.zkvm for b in remaining])
                res = []
                for current_zkvm in remaining_zkvms:
                    current_programs = set(
                        b.program for b in remaining if b.zkvm == current_zkvm
                    )
                    res.extend(
                        asyncio.get_event_loop().run_until_complete(
                            runner.run_build(
                                list(current_programs), [current_zkvm], profile
                            )
                        )
                    )

            successful = [r for r in res if r.success]
            metric_values = asyncio.get_event_loop().run_until_complete(
                asyncio.gather(
                    *[
                        runner.eval_metric(
                            metric,
                            c.zkvm,
                            c.program,
                            runner.get_out_path(profile, c.zkvm, c.program),
                            profile,
                        )
                        for c in successful
                    ],
                    return_exceptions=True,
                )
            )
            new_eval_res = EvalResult(
                False,
                values=[
                    val
                    for val in metric_values
                    if isinstance(val, MetricValue)
                    and not val.timeout
                    and not val.metric == -1
                ],
            )
            eval_result = eval_result.merge(new_eval_res)

            def get_eval(p: str, z: str) -> EvalResult:
                relevant = [
                    v for v in eval_result.values if v.program == p and v.zkvm == z
                ]
                if (
                    len(relevant) == 1
                    and not relevant[0].timeout
                    and relevant[0].metric != -1
                ):
                    return relevant[0]
                return None

            remaining = [
                BuildResult(p, z, False, "tmp")
                for p in programs
                for z in zkvms
                if get_eval(p, z) is None
            ]
            if len(remaining) > 0:
                logging.error("Build failed for config %s", profile)
                logging.info("Trying to shuffle passes and retry")
                random.shuffle(active_passes)
        results.append(
            FFDResult(idx, active_passes, active, profile, False, eval_result)
        )
        flush()
