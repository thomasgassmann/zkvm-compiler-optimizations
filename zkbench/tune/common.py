from dataclasses import dataclass
import dataclasses
import hashlib
import json
import os
from typing import Literal
from zkbench.config import Profile


MODULE_PASSES = [
    "always-inline",
    "inline",
    "partial-inliner",
    "attributor",
    "add-discriminators",
    "globalsplit",
    "globaldce",
    "globalopt",
    "wholeprogramdevirt",
    "lower-global-dtors",
    "strip",
    "strip-dead-debug-info",
    "strip-dead-prototypes",
    "bounds-checking",
    "loop-extract",
    "mergefunc",
    "extract-blocks",
    "constmerge",
    "deadargelim",
    "function-attrs",
    "strip-gc-relocates",
    "hotcoldsplit",
    "argpromotion",
    "ipsccp",
    "synthetic-counts-propagation",
    "rel-lookup-table-converter",
    "aggressive-instcombine",
    "loop-mssa(licm)",
]
FUNCTION_PASSES = [
    "instcombine",
    "bdce",
    "correlated-propagation",
    "loop-sink",
    "loop-data-prefetch",
    "loop-fusion",
    "mergeicmps",
    "mldst-motion",
    "newgvn",
    "partially-inline-libcalls",
    "sroa",
    "sink",
    "speculative-execution",
    "slsr",
    "sccp",
    "gvn",
    "tailcallelim",
    "adce",
    "dse",
    "indvars",
    "jump-threading",
    "lcssa",
    "loop-unroll",
    "memcpyopt",
    "loop-simplify",
    "simplifycfg",
    "reassociate",
    "mem2reg",
    "reg2mem",
    "simple-loop-unswitch",
    "mergereturn",
    "break-crit-edges",
    "dce",
    "lower-invoke",
    "lower-switch",
    "callsite-splitting",
    "consthoist",
    "div-rem-pairs",
    "early-cse",
    "float2int",
    "gvn-hoist",
    "gvn-sink",
    "guard-widening",
    "irce",
    "instsimplify",
    "libcalls-shrinkwrap",
    "nary-reassociate",
    "separate-const-offset-from-gep",
]
LOOP_PASSES = [
    "loop-idiom",
    "loop-reduce",
    "loop-rotate",
    "loop-unroll-and-jam",
    "loop-unroll-full",
    "loop-deletion",
    "loop-instsimplify",
    "loop-interchange",
    "loop-predication",
    "loop-versioning-licm",
]
ALL_PASSES = MODULE_PASSES + FUNCTION_PASSES + LOOP_PASSES

OUT = "./bin/tune"
OUT_GENETIC = os.path.join(OUT, "genetic")
OUT_EXHAUSTIVE = os.path.join(OUT, "exhaustive")


def build_pass_list(ordered_passes: list[str]) -> str:
    top_level_items = []
    current_function_block = []
    current_module_block = []

    def flush_module_block():
        nonlocal current_module_block
        if current_module_block:
            top_level_items.append(f"module({','.join(current_module_block)})")
            current_module_block = []

    def flush_function_block():
        nonlocal current_function_block
        if current_function_block:
            top_level_items.append(f"function({','.join(current_function_block)})")
            current_function_block = []

    for p in ordered_passes:
        if p in MODULE_PASSES:
            flush_function_block()
            current_module_block.append(p)
        elif p in FUNCTION_PASSES:
            flush_module_block()
            current_function_block.append(p)
        elif p in LOOP_PASSES:
            flush_module_block()
            if current_function_block and current_function_block[-1].startswith(
                "loop("
            ):
                last_loop = current_function_block.pop()
                inner = last_loop[len("loop(") : -1]
                current_function_block.append(f"loop({inner + "," + p})")
            else:
                current_function_block.append(f"loop({p})")
        else:
            raise ValueError(f"Unknown pass type for: {p}")

    flush_module_block()
    flush_function_block()

    return ",".join(top_level_items)


@dataclass(frozen=True)
class ProfileConfig:
    name: str
    lto: Literal["off", "thin", "fat"]
    single_codegen_unit: bool
    opt_level: Literal["3", "2", "1", "0", "s", "z"]
    prepopulate_passes: bool
    passes: list[str]

    def get_unique_id(self, zkvm: str, program: str) -> str:
        return f"{self.name}-{zkvm}-{program}-{self._get_hash()[:8]}"

    def _get_hash(self):
        values = dataclasses.asdict(self)
        encoded_string = json.dumps(values).encode("utf-8")
        return hashlib.sha256(encoded_string).hexdigest()


def build_profile(config: ProfileConfig) -> Profile:
    rustflags = f"-C opt-level={config.opt_level}"
    if config.lto != "off":
        rustflags += f" -C lto={config.lto} -C embed-bitcode"
    if config.single_codegen_unit:
        rustflags += " -C codegen-units=1"
    cflags = f"-O{config.opt_level}"
    return Profile(
        profile_name=config.name,
        rustflags=rustflags,
        cflags=cflags,
        passes=config.passes,
        prepopulate_passes=config.prepopulate_passes,
    )
