mod bench;
mod input;
mod report_risc0;
mod report_sp1;
mod tune;
mod types;
mod utils;

use bench::{bench_utils::add_benchmarks_for, utils::has_previously_run};
use clap::{command, Parser, Subcommand};
use cpuprofiler::PROFILER;
use criterion::{profiler::Profiler, Criterion};
use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};
use tune::run_tune;
use types::{Config, MeasurementType, ProgramId, ProverId, TuneMetric};
use utils::read_config_json;

#[derive(Subcommand, Clone)]
pub enum EvalSubcommand {
    Criterion(CriterionArgs),
    Run(RunArgs),
    Tune(TuneArgs),
}

#[derive(Parser, Clone)]
#[command(about = "Evaluate the performance of a zkVM on a program.")]
pub struct EvalArgs {
    #[command(subcommand)]
    command: EvalSubcommand,
}

#[derive(Parser, Clone)]
pub struct CriterionArgs {
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
    #[arg(long)]
    force: bool,
    #[arg(long)]
    meta_only: bool,
}

#[derive(Parser, Clone)]
pub struct RunArgs {
    #[arg(long)]
    program: ProgramId,
    #[arg(long)]
    zkvm: ProverId,
    #[arg(long)]
    elf: String,
}

#[derive(Parser, Clone)]
pub struct TuneArgs {
    #[arg(long)]
    program: ProgramId,
    #[arg(long)]
    zkvm: ProverId,
    #[arg(long)]
    elf: String,
    #[arg(long)]
    filename: String,
    #[arg(long)]
    metric: TuneMetric,
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

fn run_criterion(args: CriterionArgs) {
    let config: Config = read_config_json();

    let c: &mut criterion::Criterion = &mut Criterion::default()
        .profile_time(if args.profile_time.is_some() {
            println!("Profiling for {} seconds", args.profile_time.unwrap());
            Some(Duration::from_secs(args.profile_time.unwrap()))
        } else {
            None
        })
        .with_profiler(Cpuprofiler);

    let programs = match args.program {
        Some(program) => program,
        None => config.programs.keys().cloned().collect(),
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

    for program in programs.iter() {
        for measurement in measurements.iter().rev() {
            for prover in zkvms.iter() {
                if has_previously_run(&program, prover, measurement) && !args.force {
                    println!("Skipping: {}-{}-{}", program, prover, measurement);
                    continue;
                }

                let group_name = format!("{}-{}-{}", program, prover, measurement);
                let mut group = c.benchmark_group(&group_name);
                group.sample_size(10);
                // if *measurement == MeasurementType::Prove {
                //     group.sampling_mode(criterion::SamplingMode::Flat);
                // }

                for profile in profiles.iter() {
                    add_benchmarks_for(
                        &program,
                        &prover,
                        &mut group,
                        &measurement,
                        &profile,
                        args.meta_only,
                    );
                }

                group.finish();
            }
        }
    }

    c.final_summary();
}

fn run_runner(run_args: RunArgs) {
    let elf: Vec<u8> = fs::read(run_args.elf).unwrap();
    let res = match run_args.zkvm {
        ProverId::Risc0 => report_risc0::Risc0Evaluator::eval(&elf, &run_args.program),
        ProverId::SP1 => report_sp1::SP1Evaluator::eval(&elf, &run_args.program),
    };
    println!("{:?}", res);
}

fn main() {
    sp1_core_machine::utils::setup_logger();
    let args = EvalArgs::parse();

    match args.command {
        EvalSubcommand::Criterion(criterion_args) => run_criterion(criterion_args),
        EvalSubcommand::Run(run_args) => run_runner(run_args),
        EvalSubcommand::Tune(tune_args) => run_tune(tune_args),
    }
}
