import asyncio
import dataclasses
import json
import logging
import os
from opentuner import ConfigurationManipulator
from opentuner import ScheduleParameter, EnumParameter, IntegerParameter
from opentuner import MeasurementInterface
from opentuner import Result
from opentuner.tuningrunmain import the_logging_config
import opentuner

from zkbench.config import Profile, get_profile_by_name
from zkbench.tune.runner import TuneRunner
from zkbench.tune.common import (
    ALL_KNOBS,
    LTO_OPTIONS,
    OPT_LEVEL_OPTIONS,
    BIN_OUT_GENETIC,
    MetricValue,
    ProfileConfig,
    TuneConfig,
    build_pass_list,
    build_profile,
)


@dataclasses.dataclass(frozen=True)
class Genetic:
    profile_configs: list[ProfileConfig | Profile]
    values: list[int]
    metrics: list[list[MetricValue]]
    best_metric: int
    best_profile: ProfileConfig | Profile
    metric: str
    programs: list[str]
    zkvms: list[str]
    config: TuneConfig
    mode_name: str
    baselines: dict[str, list[MetricValue]] | None = None


def add_common_params(manipulator: ConfigurationManipulator, config: TuneConfig):
    manipulator.add_parameter(
        EnumParameter("lto", LTO_OPTIONS if config.tune_lto else ["off"])
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
            config.allowed_opt_levels if config.tune_opt_level else ["0"],
        )
    )
    manipulator.add_parameter(
        EnumParameter(
            "prepopulate_passes",
            (
                [True, False]
                if config.tune_prepopulate_passes
                else [config.default_prepopulate_passes]
            ),
        )
    )


class Mode:
    def get_manipulator(self, config: TuneConfig):
        raise NotImplementedError("This method should be overridden by subclasses.")

    def get_profile_config(self, desired_result) -> ProfileConfig | Profile:
        raise NotImplementedError("This method should be overridden by subclasses.")


class DepthMode(Mode):
    def __init__(self, depth: int):
        self.depth = depth

    def get_name(self):
        return f"depth-{self.depth}"

    def get_profile_config(self, desired_result):
        cfg = desired_result.configuration.data
        used_passes = []
        for i in range(self.depth):
            current_pass = cfg[f"depth-{i}"]
            if current_pass != "NOOP":
                used_passes.append(current_pass)

        profile = build_profile(
            ProfileConfig(
                name="genetic",
                lto=cfg["lto"],
                single_codegen_unit=cfg["single_codegen_unit"],
                opt_level=cfg["opt_level"],
                prepopulate_passes=cfg["prepopulate_passes"],
                passes=[build_pass_list(used_passes)],
            )
        )

        rust_knobs = ""
        c_knobs = ""
        for knob, min_int, max_int in ALL_KNOBS:
            if cfg["enable_" + knob]:
                value = cfg[knob]
                rust_knobs += f" -Cllvm-args=-{knob}={value}"
                c_knobs += f" -mllvm -{knob}={value}"

        return Profile(
            profile.profile_name,
            profile.rustflags + rust_knobs,
            profile.cflags + c_knobs,
            profile.passes,
            profile.prepopulate_passes,
            profile.lower_atomic_before,
        )

    def get_manipulator(self, config: TuneConfig):
        manipulator = ConfigurationManipulator()
        all_passes = (
            config.module_passes
            + config.function_passes
            + config.loop_passes
            + ["NOOP"]
        )
        for i in range(self.depth):
            manipulator.add_parameter(EnumParameter(f"depth-{i}", all_passes))
        add_common_params(manipulator, config)

        for knob, min_int, max_int in ALL_KNOBS:
            manipulator.add_parameter(IntegerParameter(knob, min_int, max_int))
            manipulator.add_parameter(EnumParameter("enable_" + knob, [True, False]))

        return manipulator


class DefaultMode(Mode):

    def get_name(self):
        return "default"

    def get_profile_config(self, desired_result):
        # we can prebild binaries using compile, run_precompiled and compile_and_run
        # however this might cause issues if we build the same program in parallel
        cfg = desired_result.configuration.data
        used_passes = []
        for current_pass in cfg["passes"]:
            if cfg[current_pass] == "on":
                used_passes.append(current_pass)

        # pass is only applied once, we can apply pass multiple times
        pass_list = [build_pass_list(used_passes)]
        return ProfileConfig(
            name="genetic",
            lto=cfg["lto"],
            single_codegen_unit=cfg["single_codegen_unit"],
            opt_level=cfg["opt_level"],
            prepopulate_passes=cfg["prepopulate_passes"],
            passes=pass_list,
        )

    def get_manipulator(self, config: TuneConfig):
        manipulator = ConfigurationManipulator()
        all_passes = config.module_passes + config.function_passes + config.loop_passes
        manipulator.add_parameter(ScheduleParameter("passes", all_passes, {}))
        for current in all_passes:
            manipulator.add_parameter(EnumParameter(current, ["on", "off"]))
        add_common_params(manipulator, config)
        return manipulator


def create_tuner(
    programs: list[str],
    zkvms: list[str],
    metric: str,
    out: str,
    config: TuneConfig,
    mode: Mode,
    baselines: list[str],
):
    runner = TuneRunner(
        out=BIN_OUT_GENETIC, metric=metric, cache_dir=out, build_timeout=60 * 20
    )

    baseline_results: dict[str, list[MetricValue]] = {}
    for baseline in baselines:
        profile = get_profile_by_name(baseline)
        if profile is None:
            raise ValueError(f"Baseline profile {baseline} not found.")

        success = asyncio.get_event_loop().run_until_complete(
            runner.run_build(
                programs,
                zkvms,
                profile,
            )
        )
        if not success:
            raise ValueError(f"Error during build for baseline {baseline}")
        eval_result = runner.eval_all(programs, zkvms, profile)
        if eval_result.has_error:
            raise ValueError(f"Error during evaluation for baseline {baseline}")
        baseline_results[baseline] = eval_result.values

    class PassTuner(MeasurementInterface):
        def __init__(self, *args, **kwargs):
            super().__init__(*args, **kwargs)
            self._best = float("inf")
            self._best_config = None
            self._values = []
            self._profile_configs = []
            self._metrics = []

        def manipulator(self):
            return mode.get_manipulator(config)

        def run(self, desired_result, input, limit):
            profile_config = mode.get_profile_config(desired_result)
            logging.info(f"Running with profile config: {profile_config}")

            # first build all the binaries
            res = asyncio.get_event_loop().run_until_complete(
                runner.run_build(
                    programs,
                    zkvms,
                    profile_config,
                )
            )
            if not res:
                logging.error(f"Error during build for profile {profile_config}")
                return Result(time=float("inf"), state="ERROR")

            # then calculate metrics
            eval_result = runner.eval_all(programs, zkvms, profile_config)
            if eval_result.has_error:
                return Result(time=float("inf"), state="ERROR")

            values = eval_result.values
            self._metrics.append(values)

            metric_sum = sum([v.metric for v in values])
            self._values.append(metric_sum)
            self._profile_configs.append(profile_config)

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

            with open(os.path.join(out, "stats.json"), "w") as f:
                json.dump(
                    dataclasses.asdict(
                        Genetic(
                            self._profile_configs,
                            self._values,
                            self._metrics,
                            self._best,
                            self._best_config,
                            metric,
                            programs,
                            zkvms,
                            config,
                            mode_name=mode.get_name(),
                            baselines=baseline_results,
                        )
                    ),
                    f,
                )

            return Result(time=metric_sum, state="OK")

    return PassTuner


def run_tune_genetic(
    programs: list[str],
    zkvms: list[str],
    metric: str,
    config: TuneConfig,
    mode: str,
    out: str,
    depth: int | None,
    baselines: list[str] | None = None,
):
    arg_parser = opentuner.default_argparser()

    the_logging_config["handlers"]["console"]["level"] = logging.getLevelName(
        logging.getLogger().level
    )
    the_logging_config["loggers"][""]["level"] = logging.getLevelName(
        logging.getLogger().level
    )

    if mode == "default":
        mode = DefaultMode()
    elif mode == "depth":
        mode = DepthMode(depth)
    else:
        raise ValueError(f"Unknown mode: {mode}")

    create_tuner(programs, zkvms, metric, out, config, mode, baselines or []).main(
        arg_parser.parse_args([])
    )
