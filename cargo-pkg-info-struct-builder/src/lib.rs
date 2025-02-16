use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn inject_build_metadata() {
    // Get the consuming package's root directory (not OUT_DIR)
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let generated_dir: PathBuf = Path::new(&manifest_dir).join("src");
    let dest_path = generated_dir.join("cargo_pkg_info.rs");

    // Ensure the generated directory exists
    fs::create_dir_all(&generated_dir).expect("Failed to create generated directory");

    // FIXME: For some reason `CARGO_BUILD_TARGET` is not working, but this does
    let build_target = env::var("TARGET").unwrap();
    println!("cargo:rustc-env=BUILD_TARGET={}", build_target);

    // Embed inject.rs as bytes at compile time
    const CONTENTS: &[u8] = include_bytes!("inject.rs");

    // Write the embedded bytes to cargo_pkg_info.rs
    fs::write(&dest_path, CONTENTS).expect("Failed to write metadata file");

    // Ensure Cargo rebuilds if Cargo.toml changes
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=inject.rs");
}
