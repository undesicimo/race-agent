use std::{env, path::PathBuf};

fn main() {
    if env::var("CARGO_CFG_WINDOWS").is_err() {
        return;
    }

    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR set"))
        .join("app.manifest");

    println!("cargo:rerun-if-changed={}", manifest.display());
    println!("cargo:rustc-link-arg-bin=collector-windows=/MANIFEST:EMBED");
    println!(
        "cargo:rustc-link-arg-bin=collector-windows=/MANIFESTINPUT:{}",
        manifest.display()
    );
}
