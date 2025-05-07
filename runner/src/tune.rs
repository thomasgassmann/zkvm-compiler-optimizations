use core::panic;
use std::fs;

use serde::Serialize;

use crate::{
    bench::{
        risc0_utils::{get_risc0_stats, prove_core_risc0, prove_core_risc0_prepare},
        sp1_utils::{
            exec_sp1, exec_sp1_prepare, get_sp1_stats, prove_core_sp1, prove_core_sp1_prepare,
        },
    },
    types::{ProverId, TuneMetric},
    utils::time_operation,
    TuneArgs,
};

#[derive(Debug, Serialize)]
struct Metric {
    metric: u128,
}

pub fn run_tune(args: TuneArgs) {
    let elf: Vec<u8> = fs::read(args.elf).unwrap();
    let metric_value: u128 = match (args.zkvm, args.metric) {
        (ProverId::Risc0, TuneMetric::CycleCount) => {
            get_risc0_stats(&elf, &args.program, &None).cycle_count as u128
        }
        (ProverId::SP1, TuneMetric::CycleCount) => {
            get_sp1_stats(&elf, &args.program, &None).cycle_count as u128
        }
        (ProverId::Risc0, TuneMetric::ProveTime) => {
            let (prover, context, session) = prove_core_risc0_prepare(&elf, &args.program, &None);
            let (_, duration) = time_operation(|| prove_core_risc0(&prover, &context, &session));
            duration.as_millis()
        }
        (ProverId::SP1, TuneMetric::ProveTime) => {
            let (pk, _, stdin) = prove_core_sp1_prepare(&elf, &args.program, &None);
            let (_, duration) = time_operation(|| prove_core_sp1(&stdin, &pk));
            duration.as_millis()
        }
        (ProverId::Risc0, TuneMetric::Gas) => {
            panic!("Gas metric is not supported for Risc0");
        }
        (ProverId::SP1, TuneMetric::Gas) => {
            let (stdin, prover) = exec_sp1_prepare(&elf, &args.program, &None);
            let (_, report) = exec_sp1(&stdin, &prover, &elf);
            report.gas.unwrap() as u128
        }
    };
    let metric_value = Metric {
        metric: metric_value,
    };
    fs::write(args.filename, serde_json::to_string(&metric_value).unwrap()).unwrap();
}
