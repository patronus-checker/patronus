use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=PATRONUS_PROVIDER_DIR");
    println!(
        "cargo:rustc-env=PATRONUS_PROVIDER_DIR={}",
        env::var("PATRONUS_PROVIDER_DIR").unwrap_or("/usr/lib/patronus".to_string())
    );
}
