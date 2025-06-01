use core::panic;
use std::{
    fs,
    time::{self, Instant},
};

use serde::Serialize;

use crate::{
    bench::{
        risc0_utils::{
            exec_risc0, exec_risc0_setup, get_risc0_stats, prove_core_risc0,
            prove_core_risc0_prepare,
        },
        sp1_utils::{
            exec_sp1, exec_sp1_prepare, get_sp1_stats, prove_core_sp1, prove_core_sp1_prepare,
        },
    },
    types::{ProverId, TuneMetric},
    TuneArgs,
};

#[derive(Debug, Serialize)]
struct Metric {
    metric: u128,
}

pub fn tune_time_operation<T, F: Fn(T), G: Fn() -> T>(
    prepare: G,
    operation: F,
    samples: u32,
) -> u128 {
    let mut total_duration = time::Duration::ZERO;
    for _ in 0..samples {
        let res = prepare();
        let start = Instant::now();
        operation(res);
        total_duration += start.elapsed();
    }
    total_duration.as_millis() / samples as u128
}

pub fn run_tune(args: TuneArgs) {
    let elf: Vec<u8> = fs::read(args.elf).unwrap();
    let num_samples = args.samples.unwrap_or(1);
    let metric_value: u128 = match (args.zkvm, args.metric) {
        (ProverId::Risc0, TuneMetric::CycleCount) => get_risc0_stats(&elf, &args.program, &None)
            .cycle_count
            .unwrap() as u128,
        (ProverId::SP1, TuneMetric::CycleCount) => get_sp1_stats(&elf, &args.program, &None)
            .cycle_count
            .unwrap() as u128,
        (ProverId::Risc0, TuneMetric::PagingCycleCount) => {
            get_risc0_stats(&elf, &args.program, &None)
                .paging_cycles
                .unwrap() as u128
        }
        (ProverId::Risc0, TuneMetric::ProveTime) => tune_time_operation(
            || prove_core_risc0_prepare(&elf, &args.program, &None),
            |(prover, context, session)| {
                prove_core_risc0(&prover, &context, &session);
            },
            num_samples,
        ),
        (ProverId::SP1, TuneMetric::ProveTime) => tune_time_operation(
            || prove_core_sp1_prepare(&elf, &args.program, &None),
            |(pk, _, stdin)| prove_core_sp1(&stdin, &pk),
            num_samples,
        ),
        (ProverId::SP1, TuneMetric::Gas) => {
            let (stdin, prover) = exec_sp1_prepare(&elf, &args.program, &None);
            let (_, report) = exec_sp1(&stdin, &prover, &elf);
            report.gas.unwrap() as u128
        }
        (ProverId::Risc0, TuneMetric::ExecTime) => tune_time_operation(
            || exec_risc0_setup(&elf, &args.program, &None),
            |mut executor| exec_risc0(&mut executor),
            num_samples,
        ),
        (ProverId::SP1, TuneMetric::ExecTime) => tune_time_operation(
            || exec_sp1_prepare(&elf, &args.program, &None),
            |(stdin, prover)| {
                exec_sp1(&stdin, &prover, &elf);
            },
            num_samples,
        ),
        _ => panic!("Unsupported combination of prover and metric"),
    };
    let metric_value = Metric {
        metric: metric_value,
    };
    fs::write(args.filename, serde_json::to_string(&metric_value).unwrap()).unwrap();
}
