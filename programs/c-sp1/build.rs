use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("c_src/hi.c")
        .compile("hi");

    let bindings = bindgen::Builder::default()
        .header("c_src/hi.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
