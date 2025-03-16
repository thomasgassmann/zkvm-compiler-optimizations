mod bench;

use bench::bench_utils::{add_benchmarks_for, read_config_json};
use clap::{command, Parser};
use cpuprofiler::PROFILER;
use criterion::{profiler::Profiler, Criterion};
use runner::types::{Config, MeasurementType, ProgramId, ProverId};
use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

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
    #[arg(long = "profile-time")]
    profile_time: Option<u64>,
}

struct Cpuprofiler;

impl Profiler for Cpuprofiler {
    fn start_profiling(&mut self, benchmark_id: &str, _benchmark_dir: &Path) {
        let absolute_path = fs::canonicalize(PathBuf::from("profiles")).unwrap();
        let path_name = absolute_path.join(PathBuf::from(format!("{}.profile", benchmark_id)));
        fs::create_dir_all(&path_name.parent().unwrap()).unwrap();
        println!("Profiling to: {:?}", path_name);
        PROFILER
            .lock()
            .unwrap()
            .start(path_name.to_str().unwrap())
            .unwrap();
    }

    fn stop_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {
        // Stop the profiler.
        PROFILER.lock().unwrap().stop().unwrap();
    }
}

fn main() {
    sp1_core_machine::utils::setup_logger();
    let config: Config = read_config_json();

    let args = EvalArgs::parse();
    let c: &mut criterion::Criterion = &mut Criterion::default()
        .profile_time(if args.profile_time.is_some() {
            println!("Profiling for {} seconds", args.profile_time.unwrap());
            Some(Duration::from_secs(args.profile_time.unwrap()))
        } else {
            None
        })
        .with_profiler(Cpuprofiler)
        .sample_size(10);

    let programs = match args.program {
        Some(program) => program,
        None => config.programs.list,
    };
    let measurements = match args.measurement {
        Some(measurement) => measurement,
        None => config.measurements,
    };
    let zkvms = match args.zkvm {
        Some(zkvm) => zkvm,
        None => config.zkvms,
    };
    let profiles = match args.profile {
        Some(profile) => profile,
        None => config.profiles.keys().cloned().collect(),
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
                let mut group = c.benchmark_group(&format!("{}-{}", program, prover));

                for profile in profiles.iter() {
                    add_benchmarks_for(&program, &prover, &mut group, &measurement, &profile);
                }

                group.finish();
            }
        }
    }

    c.final_summary();
}
