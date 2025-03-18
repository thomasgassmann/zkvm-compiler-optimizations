use std::{env, process::Command};
use std::{thread, time};

pub fn setup_build(program: &str) {
    println!("cargo:rerun-if-changed=NULL");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=PASSES");
    println!("cargo:rerun-if-env-changed=RUSTFLAGS");

    let passes = env::var("PASSES").unwrap_or("".to_string());
    let mut passes_string = String::from("PASSES=lower-atomic");
    if !passes.is_empty() {
        passes_string = format!("PASSES=lower-atomic,{}", &passes);
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

    thread::sleep(time::Duration::from_secs(3));
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