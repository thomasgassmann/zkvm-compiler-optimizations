use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/hello.c");
    println!("cargo:rerun-if-changed=Makefile");

    let status = Command::new("make")
        .arg("all")
        .status()
        .expect("Failed to run make");

    if !status.success() {
        panic!("Make failed with status: {:?}", status);
    }

    println!("cargo:rustc-link-search=native=./ctarget");
    println!("cargo:rustc-link-lib=static=hello");
}
