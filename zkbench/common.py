import asyncio
from functools import wraps
import logging

import click

from zkbench.config import (
    get_default_profiles_ids,
    get_programs,
    get_programs_by_group,
    get_zkvms,
)


def coro(f):
    @wraps(f)
    def wrapper(*args, **kwargs):
        return asyncio.run(f(*args, **kwargs))

    return wrapper


def get_run_config(
    programs: list[str],
    zkvms: list[str],
    profiles: list[str],
    program_groups: list[str] | None = None,
):
    if program_groups:
        for program_group in program_groups:
            programs.extend(get_programs_by_group(program_group))

    programs = programs if len(programs) > 0 else get_programs()
    zkvms = zkvms if len(zkvms) > 0 else get_zkvms()
    profiles = profiles if len(profiles) > 0 else get_default_profiles_ids()
    return programs, zkvms, profiles


async def run_command(cmd, cwd, env, task_name, timeout=None):
    logging.debug(f"[{task_name}] Running command: {cmd}, cwd: {cwd}")
    process = await asyncio.create_subprocess_shell(
        cmd,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE,
        cwd=cwd,
        env=env,
    )

    async def stream_output(stream, name):
        while True:
            line = await stream.readline()
            if line:
                logging.debug(f"[{task_name}, {name}] {line.decode().rstrip()}")
            else:
                break

    try:
        await asyncio.wait_for(
            asyncio.gather(
                stream_output(process.stdout, "stdout"),
                stream_output(process.stderr, "stderr"),
            ),
            timeout=timeout,
        )
    except asyncio.TimeoutError:
        process.kill()
        raise asyncio.TimeoutError(
            f"[{task_name}] Command timed out after {timeout} seconds"
        )

    return await process.wait()


def get_log_level(level_str: str) -> int:
    try:
        try:
            import pydevd  # type: ignore

            return logging.DEBUG
        except ImportError:
            return getattr(logging, level_str) if level_str else logging.INFO
    except AttributeError:
        raise click.ClickException(f"Log level {level_str} not found.")


def setup_logger(level_str: str | int, log_file: str | None = None):
    log_formatter = logging.Formatter(
        "%(asctime)s [%(threadName)-12.12s] [%(levelname)-5.5s]  %(message)s"
    )
    root_logger = logging.getLogger()
    root_logger.propagate = True
    if isinstance(level_str, int):
        level = level_str
    else:
        level = get_log_level(level_str)
    root_logger.setLevel(level)

    if log_file:
        file_handler = logging.FileHandler(log_file)
        file_handler.setFormatter(log_formatter)
        root_logger.addHandler(file_handler)

    console_handler = logging.StreamHandler()
    console_handler.setFormatter(log_formatter)
    root_logger.addHandler(console_handler)
