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
from zkbench.config import Profile

PASSES = [
    "sccp",
    "gvn",
    "tailcallelim",
    "adce",
    "dse",
    "indvars",
    "aggressive-instcombine",
    "jump-threading",
    "lcssa",
    "loop-reduce",
    "loop-rotate",
    "loop-simplify",
    "loop-unroll",
    "loop-unroll-and-jam",
    "loop-unroll-full",
    "loop-mssa(licm)",
    "loop-deletion",
    "memcpyopt",
    "simplifycfg",
    "reassociate",
    "mem2reg",
    "reg2mem",
    "simple-loop-unswitch",
    "mergereturn",
]


def create_tuner(program: str, zkvm: str):
    class PassTuner(MeasurementInterface):
        def manipulator(self):
            manipulator = ConfigurationManipulator()
            manipulator.add_parameter(ScheduleParameter("passes", PASSES, {}))
            for current in PASSES:
                manipulator.add_parameter(EnumParameter(current, ["on", "off"]))
            return manipulator

        def run(self, desired_result, input, limit):
            cfg = desired_result.configuration.data
            pass_list = []
            for current_pass in cfg["passes"]:
                if cfg[current_pass] == "on":
                    pass_list.append(current_pass)

            out = "./bin/tune"
            profile = Profile(
                profile_name="tune",
                rustflags="-C opt-level=3",
                passes=pass_list,
                prepopulate_passes=True,
            )

            try:
                asyncio.get_event_loop().run_until_complete(
                    build_program(
                        program=program,
                        zkvm=zkvm,
                        profile=profile,
                        llvm=False,
                        target=out,
                    )
                )
            except KeyboardInterrupt:
                raise
            except:
                logging.error(f"Failed to build program with passes: {pass_list}")
                return Result(time=float("inf"))

            res = os.system(
                f"""
                ./target/release/runner stats --program {program} --zkvm {zkvm} --elf {out} --filename /tmp/tune
            """.strip()
            )

            if res != 0:
                raise Exception(f"Failed to run the program with passes: {pass_list}")

            cycle_count = json.loads(open("/tmp/tune").read())["cycle_count"]

            print(cycle_count)
            return Result(time=cycle_count)

    return PassTuner


def run_tune(program: str, zkvm: str):
    arg_parser = opentuner.default_argparser()
    create_tuner(program, zkvm).main(arg_parser.parse_args([]))
