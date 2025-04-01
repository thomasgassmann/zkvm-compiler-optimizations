import asyncio
import json
import logging
import os
from opentuner import ConfigurationManipulator
from opentuner import ScheduleParameter, EnumParameter
from opentuner import MeasurementInterface
from opentuner import Result
import opentuner

from zkbench.build import build_program
from zkbench.common import run_command, setup_logger
from zkbench.config import Profile
from zkbench.tune.common import (
    ALL_PASSES,
    ProfileConfig,
    build_pass_list,
    build_profile,
)


OUT = "./bin/tune/genetic/"


def get_out_path(config: ProfileConfig, zkvm: str, program: str) -> str:
    return os.path.join(OUT, config.get_unique_id(zkvm, program))


async def _build_and_eval(program: str, zkvm: str, profile: Profile, out: str):
    try:
        await build_program(program, zkvm, profile, False, out)
    except Exception as e:
        # TODO: some configuration are still invalid, figure out if we can further reduce that
        logging.error(f"Failed to build {program} for {zkvm}: {e}")
        return float("inf")
    logging.info(f"Built {program} for {zkvm}")
    filename = os.path.basename(out)
    stats_file = os.path.join(OUT, f"{filename}.json")
    # TODO: support metrics other than cycle count
    res = await run_command(
        f"""
        ./target/release/runner stats --program {program} --zkvm {zkvm} --elf {out} --filename {stats_file}
    """,
        None,
        {
            **os.environ,
        },
        out,
    )

    if res != 0:
        raise Exception(f"Failed to run the program: {profile}")

    cycle_count = json.loads(open(stats_file).read())["cycle_count"]
    logging.info(f"Cycle count for {program} on {zkvm}: {cycle_count}")
    os.remove(stats_file)
    os.remove(out)
    return cycle_count


def create_tuner(programs: list[str], zkvms: list[str]):
    class PassTuner(MeasurementInterface):
        def __init__(self, *args, **kwargs):
            super().__init__(*args, **kwargs)
            self._best = float("inf")
            self._best_config = None

        def manipulator(self):
            manipulator = ConfigurationManipulator()
            manipulator.add_parameter(ScheduleParameter("passes", ALL_PASSES, {}))
            for current in ALL_PASSES:
                manipulator.add_parameter(EnumParameter(current, ["on", "off"]))
            manipulator.add_parameter(EnumParameter("lto", ["off", "thin", "fat"]))
            manipulator.add_parameter(
                EnumParameter("single_codegen_unit", [True, False])
            )
            manipulator.add_parameter(
                EnumParameter("opt_level", ["0", "1", "2", "3", "s", "z"])
            )
            manipulator.add_parameter(
                EnumParameter("prepopulate_passes", [True, False])
            )
            return manipulator

        def run(self, desired_result, input, limit):
            cfg = desired_result.configuration.data
            used_passes = []
            for current_pass in cfg["passes"]:
                if cfg[current_pass] == "on":
                    used_passes.append(current_pass)

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

            current_sum = 0
            for zkvm in zkvms:
                try:
                    res = asyncio.get_event_loop().run_until_complete(
                        asyncio.gather(
                            *[
                                _build_and_eval(
                                    program,
                                    zkvm,
                                    profile,
                                    get_out_path(profile_config, zkvm, program),
                                )
                                for program in programs
                            ]
                        )
                    )

                    current_sum += sum(res)
                    if current_sum == float("inf"):
                        return Result(time=float("inf"))
                except Exception as e:
                    logging.error(f"Error during evaluation: {e}")
                    return Result(time=float("inf"))

            if current_sum < self._best or self._best_config is None:
                logging.info(
                    f"Found better configuration: {profile_config} with cycle count {current_sum}"
                )
                self._best = current_sum
                self._best_config = profile_config
            else:
                logging.info(
                    f"Configuration {self._best_config} remains best with cycle count {self._best}"
                )

            return Result(time=current_sum)

    return PassTuner


def run_tune_genetic(programs: list[str], zkvms: list[str]):
    os.makedirs(OUT, exist_ok=True)
    arg_parser = opentuner.default_argparser()
    # TODO: opentuner overwrites the logging config
    create_tuner(programs, zkvms).main(arg_parser.parse_args([]))
