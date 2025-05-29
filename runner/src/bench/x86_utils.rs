use core::panic;

use libloading::{Library, Symbol};

use crate::{
    input::{get_bigmem_input, rand_ecdsa_signature},
    types::{ProgramId, ProverId},
    utils::get_elf,
};

type MainCoreBigMem = unsafe extern "C" fn(value: u32) -> ();
#[allow(improper_ctypes_definitions)]
type MainCoreEcdsaVerify = unsafe extern "C" fn(
    encoded_verifying_key: k256::EncodedPoint,
    message: Vec<u8>,
    signature: k256::ecdsa::Signature,
);

pub fn exec_x86_prepare(
    program: &ProgramId,
    prover: &ProverId,
    profile: &String,
    _input_override: &Option<String>,
) -> Box<dyn FnOnce() + 'static> {
    let elf_path = get_elf(program, prover, profile);

    let lib =
        unsafe { Library::new(&elf_path) }.expect("couldn't dlopen the binary as a shared object");

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
                let _keep_lib_alive = &lib;
                main_core_fn(inp);
            })
        }
        ProgramId::EcdsaVerify => {
            let main_core_fn: MainCoreEcdsaVerify = load_main_core_fn!(MainCoreEcdsaVerify);
            let (encoded_verifying_key, message, signature) = rand_ecdsa_signature();
            Box::new(move || unsafe {
                let _keep_lib_alive = &lib;
                main_core_fn(encoded_verifying_key, message, signature);
            })
        }
        _ => panic!("Unsupported program for x86 execution: {:?}", program),
    }
}

#[inline(always)]
pub fn exec_x86(f: Box<dyn FnOnce() + 'static>) -> () {
    f();
}
