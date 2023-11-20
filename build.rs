use std::env;

fn main() {
    match env::var("TAIGLA_BACKEND_URL") {
        Ok(b) => println!("cargo:rustc-env=TAIGLA_BACKEND_URL={b}"),
        Err(_) => println!("cargo:rustc-env=TAIGLA_BACKEND_URL=")
    }
}