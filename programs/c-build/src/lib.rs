use std::{env, process::Command};

pub fn setup_build(program: &str) {
    println!("cargo:rerun-if-changed=NULL");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=PASSES");
    println!("cargo:rerun-if-env-changed=ZK_CFLAGS");
    println!("cargo:rerun-if-env-changed=RUSTFLAGS");

    let passes = env::var("PASSES").unwrap_or("".to_string());
    let lower_atomic_before_str = env::var("LOWER_ATOMIC_BEFORE").unwrap_or("".to_string());
    let lower_atomic_before = lower_atomic_before_str == "True";

    let mut passes_string = String::from(format!("PASSES={}", "lower-atomic"));
    if !passes.is_empty() {
        if lower_atomic_before {
            passes_string = format!("PASSES={},{}", "lower-atomic", &passes);
        } else {
            passes_string = format!("PASSES={},{}", &passes, "lower-atomic");
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

    #[cfg(feature = "x86")]
    let target = "-fPIC";
    #[cfg(not(feature = "x86"))]
    let target = "--target=riscv32-unknown-none -march=rv32im -mabi=ilp32";

    #[cfg(feature = "x86")]
    let llc_flags = "-relocation-model=pic";
    #[cfg(not(feature = "x86"))]
    let llc_flags = "";

    let cflags = env::var("ZK_CFLAGS").unwrap_or("".to_string());
    println!("cargo::warning=Done cleaning");
    let mut binding = Command::new("make");
    let make_command = binding
        .current_dir("..")
        .arg(passes_string)
        .arg(format!("ZK_CFLAGS={}", cflags))
        .arg(format!("ZK_TARGET_CFLAGS={}", target))
        .arg(format!("LLC_FLAGS={}", llc_flags))
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
