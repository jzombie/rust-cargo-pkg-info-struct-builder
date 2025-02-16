use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Injects build metadata into a specified file at the provided path.
///
/// This function is typically used during the build process to generate
/// a metadata file that contains build-specific information such as:
/// - The build timestamp in UTC (`BUILD_TIME_UTC`)
/// - The target architecture/OS (`BUILD_TARGET`)
///
/// It writes the metadata into a file specified by `project_dest_path`. If the
/// directory for the file does not exist, it is created. The function also ensures
/// that if the `Cargo.toml` or `inject.rs` files change, the build will be rerun.
///
/// # Arguments
///
/// * `project_dest_path` - A `PathBuf` representing the relative path to the target file.
///   This file is where the build metadata will be injected.
///
/// # Panics
///
/// This function will panic if:
/// - The `CARGO_MANIFEST_DIR` environment variable is not set.
/// - The destination directory cannot be created.
/// - The file cannot be written.
/// - The parent directory of the file cannot be determined.
/// ```
pub fn inject_build_metadata(project_dest_path: PathBuf) {
    // Get the consuming package's root directory (not OUT_DIR)
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let dest_path: PathBuf = Path::new(&manifest_dir).join(project_dest_path);
    let destination_dir = dest_path.parent().unwrap();

    // Ensure the generated directory exists
    fs::create_dir_all(&destination_dir).expect("Failed to create generated directory");

    // Note: `CARGO_BUILD_TARGET` does not work reliably for this
    let build_target = env::var("TARGET").unwrap();
    println!("cargo:rustc-env=BUILD_TARGET={}", build_target);

    // Custom
    let build_time_utc = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        .to_string();
    println!("cargo:rustc-env=BUILD_TIME_UTC={}", build_time_utc);

    // Embed inject.rs as bytes at compile time
    const CONTENTS: &[u8] = include_bytes!("inject.rs");

    // Write the embedded bytes to cargo_pkg_info.rs
    fs::write(&dest_path, CONTENTS).expect("Failed to write metadata file");

    // Ensure Cargo rebuilds if Cargo.toml changes
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=inject.rs");
}
