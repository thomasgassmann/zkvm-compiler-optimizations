use core::panic;

use libloading::{os::unix::Library, Symbol};

use super::utils::{get_elf_hash, ElfStats};
use crate::{
    input::{
        get_bigmem_input, get_eddsa_times, get_example_input, get_factorial_input,
        get_fibonacci_input, get_keccak256_input, get_loop_sum_input, get_merkle_input,
        get_regex_match_input, get_sha_bench_input, get_sha_chain_input, get_spec619_input,
        get_tailcall_input, load_mnist, load_rsp_input, rand_ecdsa_signature, rand_eddsa_signature,
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
#[allow(improper_ctypes_definitions)]
type MainCoreExample = unsafe extern "C" fn(a: Vec<i32>, b: Vec<i32>) -> ();

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
) -> (
    Box<dyn FnMut() -> Box<dyn std::any::Any>>,
    Box<dyn FnMut(Box<dyn std::any::Any>)>,
) {
    macro_rules! load_main_core_fn {
        ($fn_ty:ty) => {{
            let main_core_symbol: Symbol<$fn_ty> = unsafe {
                std::mem::transmute::<_, Symbol<$fn_ty>>(
                    lib.get::<Symbol<$fn_ty>>(b"main_core")
                        .expect("couldn't find `main_core` in symbol table"),
                )
            };
            *main_core_symbol
        }};
    }

    match program {
        ProgramId::Bigmem => {
            let inp = get_bigmem_input();
            let main_core_fn: MainCoreBigMem = load_main_core_fn!(MainCoreBigMem);
            (
                Box::new(move || Box::new(inp)),
                Box::new(move |inp| unsafe {
                    let inp = *inp.downcast::<u32>().expect("Invalid input type");
                    main_core_fn(inp);
                }),
            )
        }
        ProgramId::EcdsaVerify => {
            let main_core_fn: MainCoreEcdsaVerify = load_main_core_fn!(MainCoreEcdsaVerify);
            let (encoded_verifying_key, message, signature) = rand_ecdsa_signature();
            (
                Box::new(move || Box::new((encoded_verifying_key, message.clone(), signature))),
                Box::new(move |inp| unsafe {
                    let (encoded_verifying_key, message, signature) = *inp
                        .downcast::<(k256::EncodedPoint, Vec<u8>, k256::ecdsa::Signature)>()
                        .expect("Invalid input type");
                    main_core_fn(encoded_verifying_key, message, signature);
                }),
            )
        }
        ProgramId::EddsaVerify => {
            let main_core_fn: MainCoreEddsaVerify = load_main_core_fn!(MainCoreEddsaVerify);
            let mut input = Vec::new();
            for _ in 0..get_eddsa_times() {
                input.push(rand_eddsa_signature());
            }
            (
                Box::new(move || Box::new(input.clone())),
                Box::new(move |b| unsafe {
                    let items = *b
                        .downcast::<Vec<(
                            ed25519_dalek::VerifyingKey,
                            Vec<u8>,
                            ed25519_dalek::Signature,
                        )>>()
                        .expect("Invalid input type for EddsaVerify");
                    main_core_fn(items);
                }),
            )
        }
        ProgramId::Factorial => {
            let main_core_fn: MainCoreFactorial = load_main_core_fn!(MainCoreFactorial);
            let inp = get_factorial_input();
            (
                Box::new(move || Box::new(inp)),
                Box::new(move |inp| unsafe {
                    let inp = *inp
                        .downcast::<u32>()
                        .expect("Invalid input type for factorial");
                    main_core_fn(inp);
                }),
            )
        }
        ProgramId::Fibonacci => {
            let main_core_fn: MainCoreFibonacci = load_main_core_fn!(MainCoreFibonacci);
            let inp = get_fibonacci_input();
            (
                Box::new(move || Box::new(inp)),
                Box::new(move |inp| unsafe {
                    let inp = *inp
                        .downcast::<u32>()
                        .expect("Invalid input type for fibonacci");
                    main_core_fn(inp);
                }),
            )
        }
        ProgramId::Keccak256 => {
            let main_core_fn: MainCoreKeccak256 = load_main_core_fn!(MainCoreKeccak256);
            let inp = get_keccak256_input();
            (
                Box::new(move || Box::new(inp.clone())),
                Box::new(move |inp| unsafe {
                    let inp = inp
                        .downcast::<Vec<u8>>()
                        .expect("Invalid input type for keccak256");
                    main_core_fn(*inp);
                }),
            )
        }
        ProgramId::LoopSum => {
            let main_core_fn: MainCoreLoopSum = load_main_core_fn!(MainCoreLoopSum);
            let inp = get_loop_sum_input();
            (
                Box::new(move || Box::new(inp.clone())),
                Box::new(move |inp| unsafe {
                    let inp = inp
                        .downcast::<Vec<i32>>()
                        .expect("Invalid input type for loop sum");
                    main_core_fn(*inp);
                }),
            )
        }
        ProgramId::Merkle => {
            let main_core_fn: MainCoreMerkle = load_main_core_fn!(MainCoreMerkle);
            let (strings, range) = get_merkle_input();
            (
                Box::new(move || Box::new((strings.clone(), range.clone()))),
                Box::new(move |b| unsafe {
                    let (strings, range) = *b
                        .downcast::<(Vec<String>, std::ops::Range<usize>)>()
                        .expect("Invalid input type for merkle");
                    main_core_fn(strings, range);
                }),
            )
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
            (
                Box::new(move || Box::new(())),
                Box::new(move |_| unsafe {
                    main_core_fn();
                }),
            )
        }
        ProgramId::RegexMatch => {
            let main_core_fn: MainCoreRegexMatch = load_main_core_fn!(MainCoreRegexMatch);
            let (regex, text) = get_regex_match_input();
            (
                Box::new(move || Box::new((regex.clone(), text.clone()))),
                Box::new(move |b| unsafe {
                    let (regex, text) = *b
                        .downcast::<(String, String)>()
                        .expect("Invalid input type for regex match");
                    main_core_fn(regex, text);
                }),
            )
        }
        ProgramId::Rsp => {
            let main_core_fn: MainCoreRsp = load_main_core_fn!(MainCoreRsp);
            let input = load_rsp_input(input_override);
            (
                Box::new(move || Box::new(input.clone())),
                Box::new(move |b| unsafe {
                    let input = b.downcast::<Vec<u8>>().expect("Invalid input type for RSP");
                    main_core_fn(&input);
                }),
            )
        }
        ProgramId::Sha2Bench | ProgramId::Sha3Bench => {
            let main_core_fn: MainCoreShaBench = load_main_core_fn!(MainCoreShaBench);
            let input = get_sha_bench_input();
            (
                Box::new(move || Box::new(input.clone())),
                Box::new(move |b| unsafe {
                    let input = b
                        .downcast::<Vec<u8>>()
                        .expect("Invalid input type for SHA bench");
                    main_core_fn(*input);
                }),
            )
        }
        ProgramId::Sha2Chain | ProgramId::Sha3Chain => {
            let main_core_fn: MainCoreShaChain = load_main_core_fn!(MainCoreShaChain);
            let (input, num_iters) = get_sha_chain_input();
            (
                Box::new(move || Box::new((input.clone(), num_iters))),
                Box::new(move |b| unsafe {
                    let (input, num_iters) = *b
                        .downcast::<([u8; 32], u32)>()
                        .expect("Invalid input type for SHA chain");
                    main_core_fn(input, num_iters);
                }),
            )
        }
        ProgramId::Spec619 => {
            let main_core_fn: MainCoreSpec619 = load_main_core_fn!(MainCoreSpec619);
            let (it, action, sim_type) = get_spec619_input();
            (
                Box::new(move || Box::new((it, action, sim_type))),
                Box::new(move |b| unsafe {
                    let (it, action, sim_type) = *b
                        .downcast::<(i32, i32, i32)>()
                        .expect("Invalid input type for Spec619");
                    main_core_fn(it, action, sim_type);
                }),
            )
        }
        ProgramId::Spec631 => {
            let main_core_fn: MainCoreSpec631 = load_main_core_fn!(MainCoreSpec631);
            let str = include_str!("../../../inputs/spec-631/in.txt");
            let input = str.to_string();
            (
                Box::new(move || Box::new(input.clone())),
                Box::new(move |b| unsafe {
                    let input = b
                        .downcast::<String>()
                        .expect("Invalid input type for Spec631");
                    main_core_fn(*input);
                }),
            )
        }
        ProgramId::Tailcall => {
            let main_core_fn: MainCoreTailcall = load_main_core_fn!(MainCoreTailcall);
            let (n, r) = get_tailcall_input();
            (
                Box::new(move || Box::new((n, r))),
                Box::new(move |b| unsafe {
                    let (n, r) = *b
                        .downcast::<(u128, u128)>()
                        .expect("Invalid input type for Tailcall");
                    main_core_fn(n, r);
                }),
            )
        }
        ProgramId::ZkvmMnist => {
            let main_core_fn: MainCoreZkvmMnist = load_main_core_fn!(MainCoreZkvmMnist);
            let (training_data, test_data) = load_mnist();
            (
                Box::new(move || Box::new((training_data.clone(), test_data.clone()))),
                Box::new(move |b| unsafe {
                    let (training_data, test_data) = *b
                        .downcast::<(Vec<(Vec<f64>, Vec<f64>)>, Vec<(Vec<f64>, Vec<f64>)>)>()
                        .expect("Invalid input type for ZkvmMnist");
                    main_core_fn(training_data, test_data);
                }),
            )
        }
        ProgramId::Example => {
            let main_core_fn: MainCoreExample = load_main_core_fn!(MainCoreExample);
            let inp = get_example_input();
            (
                Box::new(move || Box::new(inp.clone())),
                Box::new(move |inp| unsafe {
                    let inp = *inp
                        .downcast::<(Vec<i32>, Vec<i32>)>()
                        .expect("Invalid input type for example");
                    main_core_fn(inp.0, inp.1);
                }),
            )
        }
        _ => panic!("Unsupported program for x86 execution: {:?}", program),
    }
}
