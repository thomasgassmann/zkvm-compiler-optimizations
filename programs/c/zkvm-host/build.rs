use std::{env, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=../guest/main.c");
    println!("cargo:rerun-if-changed=../Makefile");

    let passes = env::var("PASSES").unwrap_or("".to_string());
    let mut passes_string = String::from("PASSES=lower-atomic");
    if !passes.is_empty() {
        passes_string = format!("PASSES=lower-atomic,{}", &passes);
    }

    let status = Command::new("make")
        .current_dir("..")
        .arg(passes_string)
        .arg("-B")
        .arg("all")
        .status()
        .expect("Failed to run make");

    if !status.success() {
        panic!("Make failed with status: {:?}", status);
    }

    println!("cargo:rustc-link-search=native=./ctarget");
    println!("cargo:rustc-link-lib=static=zkvmc");
}
