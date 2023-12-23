use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=icons/");
    Command::new("python")
        .arg("svg_to_dioxus.py")
        .output()
        .unwrap();
    match env::var("TAIGLA_BACKEND_URL") {
        Ok(b) => println!("cargo:rustc-env=TAIGLA_BACKEND_URL={b}"),
        Err(_) => println!("cargo:rustc-env=TAIGLA_BACKEND_URL=")
    }
}
