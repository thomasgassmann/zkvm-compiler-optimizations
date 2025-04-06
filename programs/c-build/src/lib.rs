use std::{env, process::Command};

pub fn setup_build(program: &str) {
    println!("cargo:rerun-if-changed=NULL");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=PASSES");
    println!("cargo:rerun-if-env-changed=RUSTFLAGS");
    println!("cargo:rerun-if-env-changed=LLVM_VERSION");

    let passes = env::var("PASSES").unwrap_or("".to_string());
    let llvm_version = env::var("C_LLVM_VERSION").unwrap_or("".to_string());
    let lower_atomic_before_str = env::var("LOWER_ATOMIC_BEFORE").unwrap_or("".to_string());
    let lower_atomic_before = lower_atomic_before_str == "True";
    let flag_name = match llvm_version.as_str() {
        "18" => "loweratomic",
        "19" => "lower-atomic",
        _ => panic!("Invalid LLVM version: {}", llvm_version),
    };

    let mut passes_string = String::from(format!("PASSES={}", &flag_name));
    if !passes.is_empty() {
        if lower_atomic_before {
            passes_string = format!("PASSES={},{}", &flag_name, &passes);
        } else {
            passes_string = format!("PASSES={},{}", &passes, &flag_name);
        }
    }

    println!("cargo::warning=Cleaning and building C with passes: {}", passes_string);
    let status = Command::new("make")
        .current_dir("..")
        .arg("clean")
        .status()
        .expect("Failed to run make");
    if !status.success() {
        panic!("Make clean failed with status: {:?}", status);
    }

    println!("cargo::warning=Done cleaning");
    let mut binding = Command::new("make");
    let make_command = binding
        .current_dir("..")
        .arg(passes_string)
        .arg(format!("PROGRAM={}", program))
        .arg("-B")
        .arg("all");
    println!("cargo::warning=Running make: {:?}", &make_command);
    let status = make_command
        .status()
        .expect("Failed to run make");

    if !status.success() {
        panic!("Make failed with status: {:?}", status);
    }

    println!("cargo::warning=Successfully built C");
    println!("cargo:rustc-link-search=native=./ctarget");
    println!("cargo:rustc-link-lib=static={}", program);
}
