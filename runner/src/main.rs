
mod bench;

use bench::bench_utils::{add_benchmarks_for, read_config_json};
use clap::{command, Parser};
use criterion::Criterion;
use runner::types::{Config, MeasurementType, ProgramId, ProverId};

#[derive(Parser, Clone)]
#[command(about = "Evaluate the performance of a zkVM on a program.")]
pub struct EvalArgs {
    #[arg(long)]
    program: Option<Vec<ProgramId>>,
    #[arg(long)]
    zkvm: Option<Vec<ProverId>>,
    #[arg(long)]
    measurement: Option<Vec<MeasurementType>>,
    #[arg(long)]
    profile: Option<Vec<String>>,
}

fn main() {
    sp1_core_machine::utils::setup_logger();
    let config: Config = read_config_json();

    let args = EvalArgs::parse();
    let c: &mut criterion::Criterion = &mut Criterion::default().sample_size(10);

    let programs = match args.program {
        Some(program) => program,
        None => config.programs.list
    };
    let measurements = match args.measurement {
        Some(measurement) => measurement,
        None => config.measurements
    };
    let zkvms = match args.zkvm {
        Some(zkvm) => zkvm,
        None => config.zkvms
    };
    let profiles = match args.profile {
        Some(profile) => profile,
        None => config.profiles.keys().cloned().collect()
    };

    for program in programs.iter() {
        for measurement in measurements.iter() {
            for prover in zkvms.iter() {
                for profile in profiles.iter() {
                    println!("Bench: {}-{}-{}-{}", program, prover, measurement, profile);                    
                }
            }
        }
    }

    for program in programs {
        for measurement in measurements.iter() {
            for prover in zkvms.iter() {
                let mut group = c.benchmark_group(&format!("{}-{}-{}", program, prover, measurement));

                for profile in profiles.iter() {
                    add_benchmarks_for(&program, &prover, &mut group, &measurement, &profile);
                }
                
                group.finish();
            }
        }
    }

    c.final_summary();
}
