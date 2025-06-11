use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use risc0_zkvm::{
    compute_image_id, get_prover_server, ExecutorEnv, ExecutorImpl, ProverOpts, VerifierContext,
};

use crate::{
    input::set_risc0_input,
    risc0_rv32im::{disasm, get_insn_kind, DecodedInstruction, InsnKind},
    types::{PerformanceReport, ProgramId, ProverId},
    utils::time_operation,
};

// adapted from https://github.com/succinctlabs/zkvm-perf
pub struct Risc0Evaluator;

struct Report {
    address_writes: HashMap<u32, usize>,
    total_writes: usize,
    instructions: HashMap<InsnKind, usize>,
    cycles_by_pc: HashMap<u32, u64>,
    last_cycle: u64,
    instructions_at_pc: HashMap<u32, u32>,
}

impl Report {
    fn new() -> Self {
        Report {
            address_writes: HashMap::new(),
            total_writes: 0,
            instructions: HashMap::new(),
            cycles_by_pc: HashMap::new(),
            last_cycle: 0,
            instructions_at_pc: HashMap::new(),
        }
    }

    fn add_write(&mut self, addr: u32) {
        *self.address_writes.entry(addr).or_insert(0) += 1;
        self.total_writes += 1;
    }

    fn add_instruction(&mut self, word: u32, pc: u32, cycle: u64) {
        self.instructions_at_pc.entry(pc).or_insert(word);

        let insn = get_insn_kind(word);
        *self.instructions.entry(insn).or_insert(0) += 1;

        let new_cycles = cycle - self.last_cycle;

        *self.cycles_by_pc.entry(pc).or_insert(0) += new_cycles;
        self.last_cycle = cycle;
    }

    fn print_instructions(&self) {
        let mut sorted_instructions: Vec<_> = self.instructions.iter().collect();
        sorted_instructions.sort_by_key(|(insn, _count)| *insn);
        for (insn, count) in sorted_instructions {
            println!("{:?} executed {} times", insn, count);
        }

        let mut sorted_pcs: Vec<_> = self.cycles_by_pc.iter().collect();
        sorted_pcs.sort_by(|a: &(&u32, &u64), b| a.0.cmp(b.0));
        let mut total = 0;
        for (pc, count) in sorted_pcs {
            let ins_at_pc = self.instructions_at_pc.get(pc).unwrap();
            let insn = get_insn_kind(*ins_at_pc);
            let decoded = DecodedInstruction::new(*ins_at_pc);
            let disassembled = disasm(insn, &decoded);
            println!("{:#010x}: {}: {} cycles", pc, disassembled, count);

            total += count;
        }

        println!("Total cycles: {}", total);
    }

    fn print_memory(&self) {
        println!("total memory writes: {}", self.total_writes);

        let mut sorted_writes: Vec<_> = self.address_writes.iter().collect();
        sorted_writes.sort_by(|a: &(&u32, &usize), b| b.1.cmp(a.1));

        println!("Top 50 memory write locations:");
        for (addr, count) in sorted_writes.iter().take(50) {
            println!("address {:#010x} written {} times", addr, count);
        }
    }
}

impl Risc0Evaluator {
    pub fn eval(elf: &Vec<u8>, program: &ProgramId) -> PerformanceReport {
        let image_id = compute_image_id(elf.as_slice()).unwrap();

        let mut builder = ExecutorEnv::builder();
        set_risc0_input(&program, &mut builder, &None);
        let env = builder.build().unwrap();

        // Compute some statistics.
        let mut exec = ExecutorImpl::from_elf(env, &elf).unwrap();
        let session = exec.run().unwrap();
        let cycles = session.user_cycles;
        println!("cycles: {}", cycles);
        println!("cycles (paging): {}", session.paging_cycles);

        // Setup the prover.
        let mut builder = ExecutorEnv::builder();
        let rep = Arc::new(Mutex::new(Report::new()));
        builder.trace_callback({
            let c = Arc::clone(&rep);
            move |trace: risc0_zkvm::TraceEvent| {
                match trace {
                    risc0_zkvm::TraceEvent::RegisterSet { idx: _, value: _ } => {}
                    risc0_zkvm::TraceEvent::InstructionStart { cycle, pc, insn } => {
                        c.lock().unwrap().add_instruction(insn, pc, cycle);
                    }
                    risc0_zkvm::TraceEvent::MemorySet { addr, region: _ } => {
                        c.lock().unwrap().add_write(addr);
                    }
                };
                Ok(())
            }
        });
        set_risc0_input(&program, &mut builder, &None);
        let env = builder.build().unwrap();

        // Generate the session.
        let mut exec = ExecutorImpl::from_elf(env, &elf).unwrap();
        let (session, execution_duration) = time_operation(|| exec.run().unwrap());

        let report = rep.lock().unwrap();
        report.print_memory();
        report.print_instructions();

        // Generate the proof.
        let opts = ProverOpts::default();
        let prover = get_prover_server(&opts).unwrap();
        let ctx = VerifierContext::default();
        let (info, core_prove_duration) =
            time_operation(|| prover.prove_session(&ctx, &session).unwrap());
        let receipt = info.receipt;

        let composite_receipt = receipt.inner.composite().unwrap();
        let num_segments = composite_receipt.segments.len();

        // Get the core proof size by summing across all segments.
        let mut core_proof_size = 0;
        for segment in composite_receipt.segments.iter() {
            core_proof_size += segment.seal.len() * 4;
        }

        // Verify the core proof.
        let ((), core_verify_duration) = time_operation(|| receipt.verify(image_id).unwrap());

        // Now compress the proof with recursion.
        let (compressed_proof, compress_duration) =
            time_operation(|| prover.compress(&ProverOpts::succinct(), &receipt).unwrap());

        // Verify the recursive proof
        let ((), recursive_verify_duration) =
            time_operation(|| compressed_proof.verify(image_id).unwrap());

        let succinct_receipt = compressed_proof.inner.succinct().unwrap();

        // Get the recursive proof size.
        let recursive_proof_size = succinct_receipt.seal.len() * 4;
        let prove_duration = core_prove_duration + compress_duration;

        let core_khz = cycles as f64 / core_prove_duration.as_secs_f64() / 1_000.0;
        let overall_khz = cycles as f64 / prove_duration.as_secs_f64() / 1_000.0;

        // Create the performance report.
        PerformanceReport {
            program: program.to_string(),
            prover: ProverId::Risc0.to_string(),
            shards: num_segments,
            cycles: cycles as u64,
            speed: (cycles as f64) / prove_duration.as_secs_f64(),
            execution_duration: execution_duration.as_secs_f64(),
            prove_duration: prove_duration.as_secs_f64(),
            core_prove_duration: core_prove_duration.as_secs_f64(),
            core_verify_duration: core_verify_duration.as_secs_f64(),
            core_proof_size,
            core_khz,
            compress_prove_duration: compress_duration.as_secs_f64(),
            compress_verify_duration: recursive_verify_duration.as_secs_f64(),
            compress_proof_size: recursive_proof_size,
            overall_khz,
        }
    }
}
