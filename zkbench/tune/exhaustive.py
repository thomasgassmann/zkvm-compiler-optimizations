import asyncio
from dataclasses import dataclass
import dataclasses
import json
import logging
import os
from zkbench.tune.runner import TuneRunner
from zkbench.tune.common import (
    LTO_OPTIONS,
    OPT_LEVEL_OPTIONS,
    BIN_OUT_EXHAUSTIVE,
    EvalResult,
    ProfileConfig,
    TuneConfig,
    build_pass_list,
)
from itertools import product


@dataclass(frozen=True)
class ExhaustiveResult:
    passes: list[str]
    profile_config: ProfileConfig
    build_error: bool
    eval_result: EvalResult | None


@dataclass(frozen=True)
class Exhaustive:
    results: list[ExhaustiveResult]
    metric: str
    programs: list[str]
    zkvms: list[str]
    config: TuneConfig


def run_tune_exhaustive(
    programs: list[str],
    zkvms: list[str],
    metric: str,
    config: TuneConfig,
    out: str,
    depth: int,
):
    passes = config.module_passes + config.function_passes + config.loop_passes
    lto = ["off"] if not config.tune_lto else LTO_OPTIONS
    single_codegen_unit = [False] if not config.tune_codegen_units else [False, True]
    opt_level = OPT_LEVEL_OPTIONS if config.tune_opt_level else ["0"]
    prepopulate_passes = [True, False] if config.tune_prepopulate_passes else [False]
    builder_runner = TuneRunner(
        BIN_OUT_EXHAUSTIVE, metric, out, build_timeout=60 * 30, rebuild_failed=True
    )

    results = []

    def append_and_write(new_result: ExhaustiveResult):
        results.append(new_result)
        with open(os.path.join(out, "stats.json"), "w") as f:
            json.dump(
                dataclasses.asdict(
                    Exhaustive(results, metric, programs, zkvms, config)
                ),
                f,
            )

    for pass_config in product(passes, repeat=depth):
        for lto_config in lto:
            for codegen_unit_single in single_codegen_unit:
                for opt_level_config in opt_level:
                    for prepopulate_pass in prepopulate_passes:
                        profile_config = ProfileConfig(
                            name="exhaustive",
                            lto=lto_config,
                            passes=[build_pass_list(pass_config)],
                            single_codegen_unit=codegen_unit_single,
                            opt_level=opt_level_config,
                            prepopulate_passes=prepopulate_pass,
                        )

                        logging.info(f"Running with config {profile_config}")
                        res = asyncio.get_event_loop().run_until_complete(
                            builder_runner.run_build(programs, zkvms, profile_config)
                        )
                        if all([not r.success for r in res]):
                            logging.error(
                                f"Error building with config {profile_config}"
                            )
                            append_and_write(
                                ExhaustiveResult(
                                    passes=pass_config,
                                    profile_config=profile_config,
                                    build_error=True,
                                    eval_result=None,
                                )
                            )
                            continue

                        successful = [r for r in res if r.success]
                        eval_result = builder_runner.eval_all(
                            successful, profile_config
                        )
                        append_and_write(
                            ExhaustiveResult(
                                passes=pass_config,
                                profile_config=profile_config,
                                build_error=any([not r.success for r in res]),
                                eval_result=eval_result,
                            )
                        )
