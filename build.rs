use std::env;
use std::process::{Command, Output};

fn run_icon(python_bin: &str) -> std::io::Result<Output> {
    Command::new(python_bin)
        .arg("svg_to_dioxus.py")
        .output()
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=icons/");
    let _ = run_icon("python")
        .map_err(|_| run_icon("python3").unwrap());
    match env::var("TAIGLA_BACKEND_URL") {
        Ok(b) => println!("cargo:rustc-env=TAIGLA_BACKEND_URL={b}"),
        Err(_) => println!("cargo:rustc-env=TAIGLA_BACKEND_URL=")
    }
}
