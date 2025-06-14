import asyncio
import dataclasses
import json
import logging
import os
from zkbench.build import build_program
from zkbench.clean import run_clean
from zkbench.common import run_command
from zkbench.config import Profile
from zkbench.tune.common import (
    METRIC_TIMEOUT,
    SAMPLED_METRICS,
    EvalResult,
    MetricValue,
    ProfileConfig,
    build_profile,
    is_metric_parallelizable,
)
from dacite import from_dict


CLEAN_CYCLE = 15
N_SAMPLES = 3


@dataclasses.dataclass(frozen=True)
class BuildResult:
    program: str
    zkvm: str
    success: bool
    err: str | None


class TuneRunner:

    def __init__(
        self,
        out: str,
        metric: str,
        cache_dir: str | None = None,
        build_timeout: int | None = None,
        rebuild_failed: bool = False,
        retry_build: bool = True,
    ):
        self._clean_cycles = {}
        self._out = out
        self._metric = metric
        self._cache_dir = cache_dir
        self._no_clean = os.environ.get("NO_CLEAN", "False").lower() in (
            "true",
            "1",
            "yes",
        )
        self._build_timeout = build_timeout
        self._rebuild_failed = rebuild_failed
        self._retry_build = retry_build

    def get_build_path(self, zkvm: str, program: str):
        return os.path.join(
            os.path.abspath(self._cache_dir), "build", f"{program}-{zkvm}"
        )

    def get_result_path(self, profile_config: ProfileConfig | Profile):
        return os.path.join(
            os.path.abspath(self._cache_dir), profile_config.get_hash()[:10]
        )

    def filename(
        self,
        profile_config: ProfileConfig | Profile,
        program: str,
        zkvm: str,
        metric: str,
    ):
        h = self.get_result_path(profile_config)
        return os.path.join(
            h,
            f"{profile_config.name}-{program}-{zkvm}-{metric}.json",
        )

    def get_out_path(
        self, config: ProfileConfig | Profile, zkvm: str, program: str
    ) -> str:
        return os.path.join(self._out, config.get_unique_id(zkvm, program))

    async def _build(
        self, program: str, zkvm: str, profile_config: ProfileConfig | Profile, out: str
    ):
        if self._cache_dir is not None and os.path.exists(
            self.filename(profile_config, program, zkvm, self._metric)
        ):
            f = self.filename(profile_config, program, zkvm, self._metric)
            logging.info(f"Not building, already done: " + f)
            existing = from_dict(MetricValue, json.loads(open(f, "r").read()))
            if not existing.timeout and existing.metric != -1:
                logging.info(
                    f"Not building, already evaluated: {program} on {zkvm} with metric {existing.metric}"
                )
                self.write_cache(program, zkvm, profile_config, existing)
                return
            if not self._rebuild_failed:
                return

        if os.path.exists(out):
            logging.info(f"Not building, out already exists: {out}")
            return

        if not isinstance(profile_config, Profile):
            profile = build_profile(profile_config)
        else:
            profile = profile_config
        if program not in self._clean_cycles:
            self._clean_cycles[program] = 0
        if self._clean_cycles[program] >= CLEAN_CYCLE and not self._no_clean:
            self._clean_cycles[program] = 0
            logging.info(f"Cleaning {program} for {zkvm}")
            await run_clean(
                [program], [zkvm], get_path=lambda p, z: self.get_build_path(z, p)
            )
        await build_program(
            program,
            zkvm,
            profile,
            False,
            out,
            verbose=False,
            timeout=self._build_timeout,
            target_dir=self.get_build_path(zkvm, program),
        )
        self._clean_cycles[program] += 1
        logging.info(f"Built {program} for {zkvm}")

    async def _build_for_zkvm(
        self, program: str, zkvm: str, profile_config: ProfileConfig | Profile
    ):
        try:
            out = self.get_out_path(profile_config, zkvm, program)
            await self._build(program, zkvm, profile_config, out)
            return BuildResult(program, zkvm, success=True, err=None)
        except Exception as e:
            return BuildResult(program, zkvm, success=False, err=str(e))

    async def clean(self, programs: list[str], zkvms: list[str]):
        await run_clean(
            programs, zkvms, get_path=lambda p, z: self.get_build_path(z, p)
        )
        for program in programs:
            self._clean_cycles[program] = 0

    async def _try_build(
        self,
        programs: list[str],
        zkvms: list[str],
        profile_config: ProfileConfig | Profile,
    ):
        res = []
        for zkvm in zkvms:
            res.extend(
                await asyncio.gather(
                    *[
                        self._build_for_zkvm(program, zkvm, profile_config)
                        for program in programs
                    ],
                    return_exceptions=True,
                )
            )
        return res

    async def run_build(
        self,
        programs: list[str],
        zkvms: list[str],
        profile_config: ProfileConfig | Profile,
    ) -> list[BuildResult]:
        res = await self._try_build(programs, zkvms, profile_config)
        if any(not r.success for r in res) and self._retry_build:
            unsuccessful = [r for r in res if not r.success]
            unsuccessful_string = ", ".join(
                f"{r.program}-{r.zkvm}" for r in unsuccessful
            )
            logging.error(f"Build failed for some programs: {unsuccessful_string}")
            await self.clean(programs, zkvms)
            for u in unsuccessful:
                current = await self._try_build([u.program], [u.zkvm], profile_config)
                if current[0].success:
                    res.remove(u)
                    res.append(current[0])
        return res

    def eval_all(
        self,
        res: list[BuildResult],
        profile_config: ProfileConfig | Profile,
    ) -> EvalResult:
        for c in res:
            if not c.success:
                logging.warning(
                    f"Skipping evaluation for {c.program} on {c.zkvm} due to build failure: {c.err}"
                )
        eval_res = [c for c in res if c.success]

        values = []
        if is_metric_parallelizable(self._metric):
            try:
                values = asyncio.get_event_loop().run_until_complete(
                    asyncio.gather(
                        *[
                            self.eval_metric(
                                self._metric,
                                c.zkvm,
                                c.program,
                                self.get_out_path(profile_config, c.zkvm, c.program),
                                profile_config,
                            )
                            for c in eval_res
                        ]
                    )
                )
            except Exception as e:
                logging.error(f"Error during evaluation: {e}")
                return EvalResult(has_error=True, values=[])
        else:
            for c in eval_res:
                try:
                    current_metric = asyncio.get_event_loop().run_until_complete(
                        self.eval_metric(
                            self._metric,
                            c.zkvm,
                            c.program,
                            self.get_out_path(profile_config, c.zkvm, c.program),
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

    def write_cache(
        self,
        program: str,
        zkvm: str,
        profile_config: ProfileConfig | Profile,
        metric_value: MetricValue,
    ):
        if self._cache_dir is None:
            return
        f = self.filename(profile_config, program, zkvm, self._metric)
        os.makedirs(os.path.dirname(f), exist_ok=True)
        with open(f, "w") as f:
            f.write(json.dumps(dataclasses.asdict(metric_value)))

    async def eval_metric(
        self,
        metric: str,
        zkvm: str,
        program: str,
        elf: str,
        profile_config: ProfileConfig | Profile,
    ) -> MetricValue:
        if self._cache_dir is not None:
            f = self.filename(profile_config, program, zkvm, self._metric)
            if os.path.exists(f):
                logging.info(f"Not evaluating, already done: " + f)
                existing = from_dict(MetricValue, json.loads(open(f, "r").read()))
                if not self._rebuild_failed:
                    return existing
                if not existing.timeout and existing.metric != -1:
                    return existing

        filename = os.path.basename(elf)
        stats_file = os.path.join(self._out, f"{filename}.json")
        logging.info(f"Running {metric} for {program} on {zkvm}")
        timeout = METRIC_TIMEOUT[metric] * (
            N_SAMPLES if metric in SAMPLED_METRICS else 1
        )
        try:
            res = await run_command(
                f"""
                ./target/release/runner tune 
                    --program {program}
                    --zkvm {zkvm}
                    --elf {elf}
                    --filename {stats_file}
                    --metric {metric}
                    --samples {N_SAMPLES}
            """.strip().replace(
                    "\n", " "
                ),
                None,
                {
                    **os.environ,
                },
                filename,
                timeout=timeout,
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

            self.write_cache(program, zkvm, profile_config, val)
            return val
        except asyncio.TimeoutError:
            logging.error(f"Timeout for {program} on {zkvm}")
            val = MetricValue(zkvm=zkvm, program=program, metric=-1, timeout=True)
            self.write_cache(program, zkvm, profile_config, val)
            return val
        finally:
            os.remove(stats_file)
            os.remove(elf)
