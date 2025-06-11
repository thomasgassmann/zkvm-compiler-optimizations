use core::panic;

use libloading::{Library, Symbol};

use super::utils::{get_elf_hash, ElfStats};
use crate::{
    input::{
        get_bigmem_input, get_eddsa_times, get_factorial_input, get_fibonacci_input, get_keccak256_input, get_loop_sum_input, get_loop_unroll_input, get_merkle_input, get_regex_match_input, get_sha_bench_input, get_sha_chain_input, get_spec619_input, get_tailcall_input, load_mnist, load_rsp_input, rand_ecdsa_signature, rand_eddsa_signature
    },
    types::ProgramId,
};

type MainCoreBigMem = unsafe extern "C" fn(value: u32) -> ();
#[allow(improper_ctypes_definitions)]
type MainCoreEcdsaVerify = unsafe extern "C" fn(
    encoded_verifying_key: k256::EncodedPoint,
    message: Vec<u8>,
    signature: k256::ecdsa::Signature,
);
#[allow(improper_ctypes_definitions)]
type MainCoreEddsaVerify = unsafe extern "C" fn(
    items: Vec<(
        ed25519_dalek::VerifyingKey,
        Vec<u8>,
        ed25519_dalek::Signature,
    )>,
);
type MainCoreFactorial = unsafe extern "C" fn(n: u32) -> ();
type MainCoreFibonacci = unsafe extern "C" fn(n: u32) -> ();
#[allow(improper_ctypes_definitions)]
type MainCoreKeccak256 = unsafe extern "C" fn(data: Vec<u8>) -> ();
#[allow(improper_ctypes_definitions)]
type MainCoreLoopSum = unsafe extern "C" fn(data: Vec<i32>) -> ();
#[allow(improper_ctypes_definitions)]
type MainCoreMerkle =
    unsafe extern "C" fn(strings: Vec<String>, range: std::ops::Range<usize>) -> ();
type MainCore = unsafe extern "C" fn() -> ();
#[allow(improper_ctypes_definitions)]
type MainCoreRegexMatch = unsafe extern "C" fn(regex: String, text: String) -> ();
#[allow(improper_ctypes_definitions)]
type MainCoreRsp = unsafe extern "C" fn(input: &Vec<u8>) -> ();
#[allow(improper_ctypes_definitions)]
type MainCoreShaBench = unsafe extern "C" fn(input: Vec<u8>) -> ();
type MainCoreShaChain = unsafe extern "C" fn(input: [u8; 32], num_iters: u32) -> ();
type MainCoreSpec619 = unsafe extern "C" fn(it: i32, action: i32, sim_type: i32) -> ();
#[allow(improper_ctypes_definitions)]
type MainCoreSpec631 = unsafe extern "C" fn(input: String) -> ();
#[allow(improper_ctypes_definitions)]
type MainCoreTailcall = unsafe extern "C" fn(n: u128, r: u128) -> ();
#[allow(improper_ctypes_definitions)]
type MainCoreZkvmMnist = unsafe extern "C" fn(
    training_data: Vec<(Vec<f64>, Vec<f64>)>,
    test_data: Vec<(Vec<f64>, Vec<f64>)>,
) -> ();
type MainCoreLoopUnroll = unsafe extern "C" fn(reps: usize) -> ();

pub fn get_x86_stats(elf: &[u8], _: &ProgramId, _: &Option<String>) -> ElfStats {
    ElfStats {
        cycle_count: None,
        paging_cycles: None,
        size: elf.len(),
        hash: get_elf_hash(elf),
    }
}

pub fn exec_x86_prepare<'a>(
    lib: &'a Library,
    program: &ProgramId,
    input_override: &Option<String>,
) -> Box<dyn FnOnce() + 'static> {
    macro_rules! load_main_core_fn {
        ($fn_ty:ty) => {{
            let main_core_symbol: Symbol<$fn_ty> = unsafe {
                lib.get(b"main_core")
                    .expect("couldn't find `main_core` in symbol table")
            };
            *main_core_symbol
        }};
    }

    match program {
        ProgramId::Bigmem => {
            let inp = get_bigmem_input();
            let main_core_fn: MainCoreBigMem = load_main_core_fn!(MainCoreBigMem);
            Box::new(move || unsafe {
                main_core_fn(inp);
            })
        }
        ProgramId::EcdsaVerify => {
            let main_core_fn: MainCoreEcdsaVerify = load_main_core_fn!(MainCoreEcdsaVerify);
            let (encoded_verifying_key, message, signature) = rand_ecdsa_signature();
            Box::new(move || unsafe {
                main_core_fn(encoded_verifying_key, message, signature);
            })
        }
        ProgramId::EddsaVerify => {
            let main_core_fn: MainCoreEddsaVerify = load_main_core_fn!(MainCoreEddsaVerify);
            let mut input = Vec::new();
            for _ in 0..get_eddsa_times() {
                input.push(rand_eddsa_signature());
            }
            Box::new(move || unsafe {
                main_core_fn(input);
            })
        }
        ProgramId::Factorial => {
            let main_core_fn: MainCoreFactorial = load_main_core_fn!(MainCoreFactorial);
            let inp = get_factorial_input();
            Box::new(move || unsafe {
                main_core_fn(inp);
            })
        }
        ProgramId::Fibonacci => {
            let main_core_fn: MainCoreFibonacci = load_main_core_fn!(MainCoreFibonacci);
            let inp = get_fibonacci_input();
            Box::new(move || unsafe {
                main_core_fn(inp);
            })
        }
        ProgramId::Keccak256 => {
            let main_core_fn: MainCoreKeccak256 = load_main_core_fn!(MainCoreKeccak256);
            let inp = get_keccak256_input();
            Box::new(move || unsafe {
                main_core_fn(inp);
            })
        }
        ProgramId::LoopSum => {
            let main_core_fn: MainCoreLoopSum = load_main_core_fn!(MainCoreLoopSum);
            let inp = get_loop_sum_input();
            Box::new(move || unsafe {
                main_core_fn(inp);
            })
        }
        ProgramId::Merkle => {
            let main_core_fn: MainCoreMerkle = load_main_core_fn!(MainCoreMerkle);
            let (strings, range) = get_merkle_input();
            Box::new(move || unsafe {
                main_core_fn(strings, range);
            })
        }
        ProgramId::Polybench2mm
        | ProgramId::Polybench3mm
        | ProgramId::PolybenchAdi
        | ProgramId::PolybenchAtax
        | ProgramId::PolybenchBicg
        | ProgramId::PolybenchCholesky
        | ProgramId::PolybenchCorrelation
        | ProgramId::PolybenchCovariance
        | ProgramId::PolybenchDeriche
        | ProgramId::PolybenchDoitgen
        | ProgramId::PolybenchDurbin
        | ProgramId::PolybenchFdtd2d
        | ProgramId::PolybenchFloydWarshall
        | ProgramId::PolybenchGemm
        | ProgramId::PolybenchGemver
        | ProgramId::PolybenchGesummv
        | ProgramId::PolybenchGramschmidt
        | ProgramId::PolybenchHeat3d
        | ProgramId::PolybenchJacobi1d
        | ProgramId::PolybenchJacobi2d
        | ProgramId::PolybenchLu
        | ProgramId::PolybenchLudcmp
        | ProgramId::PolybenchMvt
        | ProgramId::PolybenchNussinov
        | ProgramId::PolybenchSeidel2d
        | ProgramId::PolybenchSymm
        | ProgramId::PolybenchSyr2k
        | ProgramId::PolybenchSyrk
        | ProgramId::PolybenchTrisolv
        | ProgramId::PolybenchTrmm
        | ProgramId::NpbBt
        | ProgramId::NpbCg
        | ProgramId::NpbEp
        | ProgramId::NpbFt
        | ProgramId::NpbIs
        | ProgramId::NpbLu
        | ProgramId::NpbMg
        | ProgramId::NpbSp
        | ProgramId::Sha256
        | ProgramId::Spec605 => {
            let main_core_fn: MainCore = load_main_core_fn!(MainCore);
            Box::new(move || unsafe {
                main_core_fn();
            })
        }
        ProgramId::RegexMatch => {
            let main_core_fn: MainCoreRegexMatch = load_main_core_fn!(MainCoreRegexMatch);
            let (regex, text) = get_regex_match_input();
            Box::new(move || unsafe {
                main_core_fn(regex, text);
            })
        }
        ProgramId::Rsp => {
            let main_core_fn: MainCoreRsp = load_main_core_fn!(MainCoreRsp);
            let input = load_rsp_input(input_override);
            Box::new(move || unsafe {
                main_core_fn(&input);
            })
        }
        ProgramId::Sha2Bench | ProgramId::Sha3Bench => {
            let main_core_fn: MainCoreShaBench = load_main_core_fn!(MainCoreShaBench);
            let input = get_sha_bench_input();
            Box::new(move || unsafe {
                main_core_fn(input);
            })
        }
        ProgramId::Sha2Chain | ProgramId::Sha3Chain => {
            let main_core_fn: MainCoreShaChain = load_main_core_fn!(MainCoreShaChain);
            let (input, num_iters) = get_sha_chain_input();
            Box::new(move || unsafe {
                main_core_fn(input, num_iters);
            })
        }
        ProgramId::Spec619 => {
            let main_core_fn: MainCoreSpec619 = load_main_core_fn!(MainCoreSpec619);
            let (it, action, sim_type) = get_spec619_input();
            Box::new(move || unsafe {
                main_core_fn(it, action, sim_type);
            })
        }
        ProgramId::Spec631 => {
            let main_core_fn: MainCoreSpec631 = load_main_core_fn!(MainCoreSpec631);
            let str = include_str!("../../../inputs/spec-631/in.txt");
            let input = str.to_string();
            Box::new(move || unsafe {
                main_core_fn(input);
            })
        }
        ProgramId::Tailcall => {
            let main_core_fn: MainCoreTailcall = load_main_core_fn!(MainCoreTailcall);
            let (n, r) = get_tailcall_input();
            Box::new(move || unsafe {
                main_core_fn(n, r);
            })
        }
        ProgramId::ZkvmMnist => {
            let main_core_fn: MainCoreZkvmMnist = load_main_core_fn!(MainCoreZkvmMnist);
            let (training_data, test_data) = load_mnist();
            Box::new(move || unsafe {
                main_core_fn(training_data, test_data);
            })
        }
        ProgramId::LoopUnroll => {
            let main_core_fn: MainCoreLoopUnroll = load_main_core_fn!(MainCoreLoopUnroll);
            let reps = get_loop_unroll_input();
            Box::new(move || unsafe {
                let _keep_lib_alive = &lib;
                main_core_fn(reps);
            })
        }
        _ => panic!("Unsupported program for x86 execution: {:?}", program),
    }
}
