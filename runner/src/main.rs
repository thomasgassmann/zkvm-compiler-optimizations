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

#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum CliSamplingMode {
    Flat,
    Linear,
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
    #[arg(long = "input-override")]
    input_override: Option<String>,
    #[arg(long = "sample-size")]
    sample_size: Option<usize>,
    #[arg(long = "sampling-mode")]
    sampling_mode: Option<CliSamplingMode>,
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
    #[arg(long)]
    samples: Option<u32>,
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

    if args.sample_size.is_some() {
        println!("Proving with sample size: {}", args.sample_size.unwrap());
    }

    for program in programs.iter() {
        let program_config = config.programs.get(program).unwrap();
        for measurement in measurements.iter().rev() {
            for prover in zkvms.iter() {
                let group_name = format!("{}-{}-{}", program, prover, measurement);
                let mut group = c.benchmark_group(&group_name);
                if measurement == &MeasurementType::Prove {
                    group.sample_size(args.sample_size.unwrap_or(10));
                } else if args.sample_size.is_some() {
                    group.sample_size(args.sample_size.unwrap());
                }

                if args.sampling_mode.is_some() {
                    let sampling_mode = match args.sampling_mode.unwrap() {
                        CliSamplingMode::Flat => criterion::SamplingMode::Flat,
                        CliSamplingMode::Linear => criterion::SamplingMode::Linear,
                    };
                    group.sampling_mode(sampling_mode);
                }

                for profile in profiles.iter() {
                    if program_config.skip.contains(profile) {
                        println!(
                            "Skipping: {program}-{prover}-{measurement}-{profile} (skip config)"
                        );
                        continue;
                    }

                    if has_previously_run(&program, prover, measurement, profile) && !args.force {
                        println!("Skipping: {program}-{prover}-{measurement}-{profile} (already run, ENSURE YOU ARE RUNNING ON THE SAME HARDWARE)");
                        continue;
                    }

                    println!(
                        "Running: {}-{}-{}-{}",
                        program, prover, measurement, profile
                    );
                    add_benchmarks_for(
                        &program,
                        &prover,
                        &mut group,
                        &measurement,
                        &profile,
                        args.meta_only,
                        &args.input_override,
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
        ProverId::X86 => {
            panic!("X86 evaluation is not implemented yet.");
        }
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
