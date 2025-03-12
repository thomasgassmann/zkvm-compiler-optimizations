import asyncio
from functools import wraps

from zkbench.config import get_profiles_ids, get_programs, get_zkvms

def coro(f):
    @wraps(f)
    def wrapper(*args, **kwargs):
        return asyncio.run(f(*args, **kwargs))

    return wrapper


def get_run_config(programs: list[str], zkvms: list[str], profiles: list[str]):
    programs = programs if len(programs) > 0 else get_programs()
    zkvms = zkvms if len(zkvms) > 0 else get_zkvms()
    profiles = profiles if len(profiles) > 0 else get_profiles_ids()
    return programs, zkvms, profiles
