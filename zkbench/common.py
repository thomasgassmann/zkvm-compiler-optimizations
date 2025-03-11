import asyncio
from functools import wraps

from zkbench.config import get_profiles_ids, get_programs, get_zkvms

def coro(f):
    @wraps(f)
    def wrapper(*args, **kwargs):
        return asyncio.run(f(*args, **kwargs))

    return wrapper


def get_run_config(program: str | None, zkvm: str | None, profile: str | None):
    programs = [program] if program else get_programs()
    zkvms = [zkvm] if zkvm else get_zkvms()
    profiles = [profile] if profile else get_profiles_ids()
    return programs, zkvms, profiles
