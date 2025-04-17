import asyncio
import dataclasses
import json
import logging
import os
import uuid
from dataclasses import dataclass
from opentuner import ConfigurationManipulator
from opentuner import ScheduleParameter, EnumParameter
from opentuner import MeasurementInterface
from opentuner import Result
from opentuner.tuningrunmain import the_logging_config
import opentuner

from zkbench.build import build_program
from zkbench.clean import run_clean
from zkbench.common import run_command
from zkbench.config import Profile
from zkbench.tune.common import (
    ALL_PASSES,
    OUT_GENETIC,
    ProfileConfig,
    TuneConfig,
    build_pass_list,
    build_profile,
)

CLEAN_CYCLE = 15


def is_metric_parallelizable(metric: str) -> bool:
    return metric in ["cycle-count"]


def get_out_path(config: ProfileConfig, zkvm: str, program: str) -> str:
    return os.path.join(OUT_GENETIC, config.get_unique_id(zkvm, program))


async def _eval(metric: str, zkvm: str, program: str, elf: str):
    filename = os.path.basename(elf)
    stats_file = os.path.join(OUT_GENETIC, f"{filename}.json")
    res = await run_command(
        f"""
        ./target/release/runner tune 
            --program {program} 
            --zkvm {zkvm} 
            --elf {elf}
            --filename {stats_file}
            --metric {metric}
    """.strip().replace(
            "\n", " "
        ),
        None,
        {
            **os.environ,
        },
        filename,
    )

    if res != 0:
        raise Exception(f"Failed to calculate metric the program: {elf}")

    metric = int(json.loads(open(stats_file).read())["metric"])
    logging.info(f"Metric for {program} on {zkvm}: {metric}")
    os.remove(stats_file)
    os.remove(elf)
    return metric


clean_cycles = {}


async def _build(program: str, zkvm: str, profile: Profile, out: str):
    global clean_cycles
    if program not in clean_cycles:
        clean_cycles[program] = 0
    if clean_cycles[program] >= CLEAN_CYCLE:
        clean_cycles[program] = 0
        logging.info(f"Cleaning {program} for {zkvm}")
        run_clean([program], [zkvm])
    await build_program(program, zkvm, profile, False, out)
    clean_cycles[program] += 1
    logging.info(f"Built {program} for {zkvm}")


async def _build_for_all_zkvms(
    program: str, zkvms: list[str], profile: Profile, profile_config: ProfileConfig
):
    for zkvm in zkvms:
        out = get_out_path(profile_config, zkvm, program)
        await _build(program, zkvm, profile, out)


def create_tuner(
    programs: list[str],
    zkvms: list[str],
    metric: str,
    out_stats: str,
    config: TuneConfig,
):
    class PassTuner(MeasurementInterface):
        def __init__(self, *args, **kwargs):
            super().__init__(*args, **kwargs)
            self._best = float("inf")
            self._best_config = None
            self._values = []
            self._profile_configs = []

        def manipulator(self):
            manipulator = ConfigurationManipulator()
            all_passes = (
                config.module_passes + config.function_passes + config.loop_passes
            )
            manipulator.add_parameter(ScheduleParameter("passes", all_passes, {}))
            for current in all_passes:
                manipulator.add_parameter(EnumParameter(current, ["on", "off"]))
            manipulator.add_parameter(
                EnumParameter(
                    "lto", ["off", "thin", "fat"] if config.tune_lto else ["off"]
                )
            )
            manipulator.add_parameter(
                EnumParameter(
                    "single_codegen_unit",
                    [True, False] if config.tune_codegen_units else [False],
                )
            )
            manipulator.add_parameter(
                EnumParameter(
                    "opt_level",
                    ["0", "1", "2", "3", "s", "z"] if config.tune_opt_level else ["0"],
                )
            )
            manipulator.add_parameter(
                EnumParameter(
                    "prepopulate_passes",
                    [True, False] if config.tune_prepopulate_passes else [False],
                )
            )
            return manipulator

        def run(self, desired_result, input, limit):
            # we can prebild binaries using compile, run_precompiled and compile_and_run
            # however this might cause issues if we build the same program in parallel
            cfg = desired_result.configuration.data
            used_passes = []
            for current_pass in cfg["passes"]:
                if cfg[current_pass] == "on":
                    used_passes.append(current_pass)

            # TODO: currently pass it only applied once, we can apply pass multiple times
            pass_list = [build_pass_list(used_passes)]
            profile_config = ProfileConfig(
                name="genetic",
                lto=cfg["lto"],
                single_codegen_unit=cfg["single_codegen_unit"],
                opt_level=cfg["opt_level"],
                prepopulate_passes=cfg["prepopulate_passes"],
                passes=pass_list,
            )
            profile = build_profile(profile_config)

            # first build all the binaries
            try:
                asyncio.get_event_loop().run_until_complete(
                    asyncio.gather(
                        *[
                            _build_for_all_zkvms(
                                program, zkvms, profile, profile_config
                            )
                            for program in programs
                        ]
                    )
                )
            except Exception as e:
                logging.error(f"Error during build for profile {profile_config}: {e}")
                return Result(time=float("inf"), state="ERROR")

            # then calculate metrics
            metric_sum = 0
            if is_metric_parallelizable(metric):
                try:
                    values = asyncio.get_event_loop().run_until_complete(
                        asyncio.gather(
                            *[
                                _eval(
                                    metric,
                                    zkvm,
                                    program,
                                    get_out_path(profile_config, zkvm, program),
                                )
                                for zkvm in zkvms
                                for program in programs
                            ]
                        )
                    )
                    metric_sum = sum(values)
                except Exception as e:
                    logging.error(f"Error during evaluation: {e}")
                    return Result(time=float("inf"), state="ERROR")
            else:
                for zkvm in zkvms:
                    for program in programs:
                        try:
                            current_metric = (
                                asyncio.get_event_loop().run_until_complete(
                                    _eval(
                                        metric,
                                        zkvm,
                                        program,
                                        get_out_path(profile_config, zkvm, program),
                                    )
                                )
                            )
                            metric_sum += current_metric
                        except Exception as e:
                            logging.error(f"Error during evaluation: {e}")
                            return Result(time=float("inf"), state="ERROR")

            self._values.append(metric_sum)
            self._profile_configs.append(dataclasses.asdict(profile_config))

            logging.info(f"Configuration {profile_config} has metric {metric_sum}")
            if metric_sum < self._best or self._best_config is None:
                logging.info(
                    f"Found better configuration: {profile_config} with metric {metric_sum}"
                )
                self._best = metric_sum
                self._best_config = profile_config
            else:
                logging.info(
                    f"Configuration {self._best_config} remains best with metric {self._best}"
                )

            with open(out_stats, "w") as f:
                json.dump(
                    {
                        "profile_configs": self._profile_configs,
                        "values": self._values,
                        "best_metric": self._best,
                        "best_profile": dataclasses.asdict(self._best_config),
                        "metric": metric,
                        "programs": programs,
                        "zkvms": zkvms,
                        "config": dataclasses.asdict(profile_config),
                    },
                    f,
                )

            return Result(time=metric_sum, state="OK")

    return PassTuner


def run_tune_genetic(
    programs: list[str], zkvms: list[str], metric: str, config: TuneConfig
):
    os.makedirs(OUT_GENETIC, exist_ok=True)
    arg_parser = opentuner.default_argparser()

    the_logging_config["handlers"]["console"]["level"] = logging.getLevelName(
        logging.getLogger().level
    )
    the_logging_config["loggers"][""]["level"] = logging.getLevelName(
        logging.getLogger().level
    )

    out_stats = os.path.join(
        OUT_GENETIC,
        f"stats-{'-'.join(zkvms)}-{'-'.join(programs)}-{metric}-{str(uuid.uuid4())[:5]}.json",
    )
    create_tuner(programs, zkvms, metric, out_stats, config).main(
        arg_parser.parse_args([])
    )
