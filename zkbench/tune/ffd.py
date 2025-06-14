import asyncio
from dataclasses import dataclass
import dataclasses
import json
import logging
import os

from zkbench.tune.runner import TuneRunner
from zkbench.tune.common import (
    BIN_OUT_FFD,
    EvalResult,
    ProfileConfig,
    TuneConfig,
    build_pass_list,
)
from pyDOE3 import fracfact_by_res
from itertools import compress
import random

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
    design: list[list[int]]


def run_tune_ffd(
    programs: list[str],
    zkvms: list[str],
    metric: str,
    config: TuneConfig,
    out: str,
    resolution: int,
):
    random.seed(42)
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

    def flush():
        with open(os.path.join(out, "stats.json"), "w") as f:
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

    for idx, row in enumerate(design):
        active = list(compress(factors, row == 1))
        active_passes = [p for p in pass_factors if p in active]
        scgu = ("single_codegen_unit" in active) if config.tune_codegen_units else config.default_single_codegen_unit
        pp = ("prepopulate_passes" in active) if config.tune_prepopulate_passes else config.default_prepopulate_passes

        REPEATS = 5
        # we cannot get any info about ordering anyways
        # some sequences of passes seem to cause a SEGFAULT in
        # the rust compiler
        for _ in range(REPEATS):
            profile = ProfileConfig(
                name=f"ffd_row_{idx}",
                lto="fat" if "lto" in active else "off",
                passes=[build_pass_list(active_passes)],
                single_codegen_unit=scgu,
                opt_level=config.allowed_opt_levels[0],
                prepopulate_passes=pp,
            )

            logging.info("Row %d/%d: %s", idx + 1, len(design), profile)
            res = asyncio.get_event_loop().run_until_complete(
                runner.run_build(programs, zkvms, profile)
            )
            if any([not r.success for r in res]):
                logging.error("Build failed for config %s", profile)
                logging.info("Trying to shuffle passes and retry")
                random.shuffle(active_passes)
                continue
            break

        if all([not r.success for r in res]):
            logging.error("Build failed for config %s", profile)
            results.append(FFDResult(idx, active_passes, active, profile, True, None))
            flush()
            continue

        successful = [r for r in res if r.success]
        eval_result = runner.eval_all(successful, profile)
        build_error = any([not r.success for r in res])
        results.append(
            FFDResult(idx, active_passes, active, profile, build_error, eval_result)
        )
        flush()
