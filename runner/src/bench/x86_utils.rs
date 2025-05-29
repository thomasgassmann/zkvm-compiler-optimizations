use core::panic;

use libloading::{Library, Symbol};

use crate::{
    input::get_bigmem_input,
    types::{ProgramId, ProverId},
    utils::get_elf,
};

type MainCoreBigMem = unsafe extern "C" fn(value: u32) -> ();

pub fn exec_x86_prepare(
    program: &ProgramId,
    prover: &ProverId,
    profile: &String,
    _input_override: &Option<String>,
) -> Box<dyn Fn() + 'static> {
    let elf_path = get_elf(program, prover, profile);

    let lib =
        unsafe { Library::new(&elf_path) }.expect("couldn't dlopen the binary as a shared object");

    let main_core_symbol: Symbol<MainCoreBigMem> = unsafe {
        lib.get(b"main_core")
            .expect("couldn't find `main_core` in symbol table")
    };
    let main_core_fn: MainCoreBigMem = *main_core_symbol;

    match program {
        ProgramId::Bigmem => {
            let inp = get_bigmem_input();
            Box::new(move || unsafe {
                let _keep_lib_alive = &lib;
                main_core_fn(inp);
            })
        }
        _ => panic!("Unsupported program for x86 execution: {:?}", program),
    }
}

#[inline(always)]
pub fn exec_x86(f: Box<dyn Fn() + 'static>) -> () {
    f();
}
