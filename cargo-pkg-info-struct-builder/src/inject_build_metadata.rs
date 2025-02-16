use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use toml::Value;

/// Escapes newlines (`\n`) as `\\n` in a string.
macro_rules! escape_newlines {
    ($s:expr) => {
        $s.replace("\n", "\\n")
    };
}

/// Injects build metadata, including license content if available.
///
/// This function gathers metadata such as:
/// - The build timestamp in UTC (`BUILD_TIME_UTC`)
/// - The target architecture/OS (`BUILD_TARGET`)
/// - The contents of the license file (`LICENSE_CONTENT`), if specified in `Cargo.toml`
///
/// It writes the metadata into a file specified by `project_dest_path`. If the
/// directory for the file does not exist, it is created. The function also ensures
/// that if `Cargo.toml`, `inject.rs`, or the license file changes, the build will be rerun.
///
/// # Arguments
///
/// * `project_dest_path` - A `PathBuf` representing the relative path to the target file.
///   This file is where the build metadata will be injected.
///
/// # Behavior
/// - Reads the license file path from `Cargo.toml`, if available.
/// - Reads the license file's content and sets it as an environment variable (`LICENSE_CONTENT`).
/// - Ensures that Cargo rebuilds if `Cargo.toml`, `inject.rs`, or the license file changes.
///
/// # Panics
///
/// This function will panic if:
/// - The `CARGO_MANIFEST_DIR` environment variable is not set.
/// - The destination directory cannot be created.
/// - The file cannot be written.
/// - The parent directory of the file cannot be determined.
pub fn inject_build_metadata(project_dest_path: PathBuf) {
    // Retrieve the manifest directory
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));

    let dest_path = manifest_dir.join(&project_dest_path);
    let destination_dir = dest_path.parent().unwrap();

    // Ensure the generated directory exists
    fs::create_dir_all(&destination_dir).expect("Failed to create generated directory");

    // Retrieve the build target
    let build_target = env::var("TARGET").unwrap_or_else(|_| "unknown-target".to_string());
    println!("cargo:rustc-env=BUILD_TARGET={}", build_target);

    // Get the build time in UTC
    let build_time_utc = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        .to_string();
    println!("cargo:rustc-env=BUILD_TIME_UTC={}", build_time_utc);

    // Read and set the license content if available
    if let Some(license_path) = get_license_file_path(&manifest_dir) {
        if let Ok(license_content) = fs::read_to_string(&license_path) {
            // Set license content as an environment variable
            println!(
                "cargo:rustc-env=LICENSE_CONTENT={}",
                escape_newlines!(license_content)
            );
        }
    }

    // Embed inject.rs as bytes at compile time
    const CONTENTS: &[u8] = include_bytes!("inject_build_metadata.struct.rs");

    // Check if the file already exists with the same content
    if let Ok(existing_contents) = fs::read(&dest_path) {
        if existing_contents == CONTENTS {
            // If the contents are identical, skip rewriting
            println!(
                "No changes to {}; skipping file write.",
                dest_path.display()
            );
            // We do still print rerun-if-changed triggers, so keep going
        } else {
            // Write the embedded bytes to cargo_pkg_info.rs
            fs::write(&dest_path, CONTENTS).expect("Failed to write metadata file");
        }
    } else {
        // If we couldn't read it (likely doesn't exist), just create it
        fs::write(&dest_path, CONTENTS).expect("Failed to write metadata file");
    }

    // Ensure Cargo rebuilds if Cargo.toml or the license file changes
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=inject.rs");
    if let Some(license_path) = get_license_file_path(&manifest_dir) {
        println!("cargo:rerun-if-changed={}", license_path.display());
    }
}

/// Reads `Cargo.toml`, parses it, and extracts the value of a specified field.
///
/// This function reads the `[package]` section of `Cargo.toml` and returns the value
/// of the requested key if it exists.
///
/// # Arguments
///
/// * `manifest_dir` - The path to the consuming package's root directory.
/// * `field` - The key in `Cargo.toml` to retrieve (e.g., `"license-file"`, `"version"`, etc.).
///
/// # Returns
///
/// * `Some(String)` - The extracted value if found.
/// * `None` - If the key does not exist.
pub fn get_cargo_field(manifest_dir: &Path, field: &str) -> Option<String> {
    let cargo_toml_path = manifest_dir.join("Cargo.toml");
    let cargo_toml_content = fs::read_to_string(&cargo_toml_path).ok()?;
    let cargo_toml: Value = toml::from_str(&cargo_toml_content).ok()?;

    cargo_toml
        .get("package")?
        .get(field)?
        .as_str()
        .map(|s| s.to_string())
}

/// Retrieves the absolute path of the `license-file` from `Cargo.toml`, if available.

/// Reads `Cargo.toml`, extracts the `license-file` path, and returns it.
///
/// This function looks for the `license-file` key in `Cargo.toml` and extracts
/// its value, which represents the relative path to the license file. If found,
/// the function returns the absolute path to the file.
///
/// # Arguments
///
/// * `manifest_dir` - A reference to the project's root directory.
///
/// # Returns
///
/// * `Some(PathBuf)` - The absolute path to the license file if found.
/// * `None` - If `license-file` is not defined in `Cargo.toml`.
///
/// # Behavior
///
/// - Reads the `Cargo.toml` file and scans for the `license-file` key.
/// - If found, it extracts and constructs the full path relative to `manifest_dir`.
/// - If no `license-file` key is found, it returns `None`.
///
/// # Notes
///
/// - This function does **not** verify whether the license file exists.
/// - It does **not** parse TOML properly; it just scans lines for the `license-file` key.
/// - If `Cargo.toml` uses unconventional formatting, it might not be detected.
pub fn get_license_file_path(manifest_dir: &Path) -> Option<PathBuf> {
    get_cargo_field(manifest_dir, "license-file").map(|rel_path| manifest_dir.join(rel_path))
}
