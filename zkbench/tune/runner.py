import asyncio
import dataclasses
import json
import logging
import os
from zkbench.build import build_program
from zkbench.clean import run_clean
from zkbench.common import run_command
from zkbench.tune.common import EvalResult, MetricValue, ProfileConfig, build_profile, is_metric_parallelizable
from dacite import from_dict


CLEAN_CYCLE = 5


class TuneRunner:

    def __init__(self, out: str, metric: str, cache_dir: str = None):
        self._clean_cycles = {}
        self._out = out
        self._metric = metric
        self._cache_dir = cache_dir

    def filename(
        self, profile_config: ProfileConfig, program: str, zkvm: str, metric: str
    ):
        h = profile_config.get_hash()[:10]
        return os.path.join(
            self._cache_dir,
            f"{h}/{profile_config.name}-{program}-{zkvm}-{metric}.json",
        )

    def get_out_path(self, config: ProfileConfig, zkvm: str, program: str) -> str:
        return os.path.join(self._out, config.get_unique_id(zkvm, program))

    async def _build(self, program: str, zkvm: str, profile_config: ProfileConfig, out: str):
        if self._cache_dir is not None and os.path.exists(
            self.filename(profile_config, program, zkvm, self._metric)
        ):
            logging.info(
                f"Not building, already done: "
                + self.filename(profile_config, program, zkvm, self._metric)
            )
            return

        profile = build_profile(profile_config)
        if program not in self._clean_cycles:
            self._clean_cycles[program] = 0
        if self._clean_cycles[program] >= CLEAN_CYCLE:
            self._clean_cycles[program] = 0
            logging.info(f"Cleaning {program} for {zkvm}")
            run_clean([program], [zkvm])
        await build_program(program, zkvm, profile, False, out)
        self._clean_cycles[program] += 1
        logging.info(f"Built {program} for {zkvm}")

    async def _build_for_all_zkvms(self, 
        program: str, zkvms: list[str], profile_config: ProfileConfig
    ):
        for zkvm in zkvms:
            out = self.get_out_path(profile_config, zkvm, program)
            await self._build(program, zkvm, profile_config, out)

    async def run_build(
            self,
        programs: list[str],
        zkvms: list[str],
        profile_config: ProfileConfig,
    ):
        await asyncio.gather(
            *[self._build_for_all_zkvms(program, zkvms, profile_config) for program in programs]
        )

    def eval_all(self, programs: list[str], zkvms: list[str], profile_config: ProfileConfig):
        values = []
        if is_metric_parallelizable(self._metric):
            try:
                values = asyncio.get_event_loop().run_until_complete(
                    asyncio.gather(
                        *[
                            self.eval_metric(
                                self._metric,
                                zkvm,
                                program,
                                self.get_out_path(profile_config, zkvm, program),
                                profile_config,
                            )
                            for zkvm in zkvms
                            for program in programs
                        ]
                    )
                )
            except Exception as e:
                logging.error(f"Error during evaluation: {e}")
                return EvalResult(has_error=True, values=[])
        else:
            for zkvm in zkvms:
                for program in programs:
                    try:
                        current_metric = asyncio.get_event_loop().run_until_complete(
                            self.eval_metric(
                                self._metric,
                                zkvm,
                                program,
                                self.get_out_path(profile_config, zkvm, program),
                                profile_config,
                            )
                        )
                        values.append(current_metric)
                    except Exception as e:
                        logging.error(f"Error during evaluation: {e}")
                        return EvalResult(has_error=True, values=[])
        return EvalResult(
            has_error=False,
            values=values,
        )

    async def eval_metric(
        self,
        metric: str,
        zkvm: str,
        program: str,
        elf: str,
        profile_config: ProfileConfig,
    ) -> MetricValue:
        f = self.filename(profile_config, program, zkvm, self._metric)
        if self._cache_dir is not None and os.path.exists(f):
            logging.info(f"Not evaluating, already done: " + f)
            return from_dict(MetricValue, json.loads(open(f, "r").read()))

        filename = os.path.basename(elf)
        stats_file = os.path.join(self._out, f"{filename}.json")
        try:
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
            val = MetricValue(
                zkvm=zkvm,
                program=program,
                metric=metric,
            )

            if self._cache_dir is not None:
                os.makedirs(os.path.dirname(f), exist_ok=True)
                with open(f, "w") as f:
                    f.write(json.dumps(dataclasses.asdict(val)))

            return val
        finally:
            os.remove(stats_file)
            os.remove(elf)
